//! Pinned VMA 3.4.0 `ComputeDefragmentation_Extensive`, `MoveDataToFreeBlocks`
//! and the Extensive ignored-block branch in `DefragmentPassEnd`.
//! See LICENSE-VMA-MIT. Adaptation: wait for explicit pass resolution before
//! considering a proposed evacuation empty; retain native blocks through the
//! exclusive context and publish phase/order changes only after planning succeeds.
use super::*;

#[derive(Clone, Copy)]
enum Phase {
	Evacuate(ResourceGroup),
	Pack(ResourceGroup),
	Cleanup,
	Done,
}

#[derive(Clone, Copy)]
pub(super) struct ExtensiveState {
	phase: Phase,
	// Index relative to this domain, not the allocator's mutable native block vector.
	first_free: Option<usize>,
	immovable: usize,
}

impl Default for ExtensiveState {
	fn default() -> Self {
		Self {
			phase: Phase::Evacuate(ResourceGroup::Textures),
			first_free: None,
			immovable: 0,
		}
	}
}

// Like the donor, Extensive places ignored blocks at the end of the vector and
// excludes them from future evacuation sources. They remain valid destinations.
pub(super) fn exclude_ignored(blocks: &mut [OrderedBlock], domains: &mut [DomainState]) {
	for domain in domains {
		let Some(start) = blocks.iter().position(|block| block.domain == domain.key) else {
			continue;
		};
		let count = blocks[start..]
			.iter()
			.take_while(|block| block.domain == domain.key)
			.count();
		let state = &mut domain.extensive;
		let mut active = count - state.immovable;
		let mut index = 0;
		while index < active {
			if !blocks[start + index].immovable {
				index += 1;
				continue;
			}
			active -= 1;
			state.immovable += 1;
			blocks.swap(start + index, start + active);
			if let Some(boundary) = state.first_free.as_mut() {
				if index < *boundary {
					*boundary -= 1;
					if index < *boundary {
						blocks.swap(start + index, start + *boundary);
					}
				}
				*boundary = (*boundary).min(active);
			}
		}
	}
}

fn selection(
	sources: std::ops::Range<usize>,
	targets: std::ops::Range<usize>,
	earlier_only: bool,
	within: Within,
	group: ResourceGroup,
) -> Selection {
	Selection {
		sources,
		targets,
		earlier_only,
		within,
		group,
	}
}

pub(super) unsafe fn plan(
	allocator: &mut Allocator<'_>,
	scan: &Scan<'_>,
	range: std::ops::Range<usize>,
	state: &mut ExtensiveState,
	moves: &mut HostVec<'_, Move>,
	counters: &mut PassCounters,
) -> Result<(), AllocError> {
	let active = range.end - range.start - state.immovable;
	let boundary = state.first_free.get_or_insert(active);
	*boundary = (*boundary).min(active);
	loop {
		if counters.exhausted(scan.max_allocations, moves.len()) {
			return Ok(());
		}
		let first_free = range.start + state.first_free.expect("initialized domain boundary");
		match state.phase {
			Phase::Done => return Ok(()),
			Phase::Evacuate(group) => {
				if first_free == range.start {
					let before = moves.len();
					// Donor's final Fast consolidation, followed by same-block cleanup.
					let choice = selection(
						range.clone(),
						range.clone(),
						true,
						Within::Never,
						ResourceGroup::All,
					);
					// SAFETY: existing compatible storage; caller resolves all proposed moves.
					unsafe { plan_selected(allocator, scan, &choice, moves, counters) }?;
					if moves.len() != before || counters.exhausted(scan.max_allocations, moves.len()) {
						return Ok(());
					}
					// Cancellation and copy limits must resume this consolidation,
					// rather than prematurely skipping to same-block-only cleanup.
					state.phase = Phase::Cleanup;
					continue;
				}
				let last = first_free - 1;
				let handle = scan.blocks[last].handle;
				if allocator
					.blocks
					.iter()
					.find(|block| block.handle == handle)
					.expect("held evacuation block")
					.metadata
					.is_empty()
				{
					state.first_free = Some(last - range.start);
					state.phase = Phase::Pack(group);
					continue;
				}
				let before = moves.len();
				let choice = selection(
					last..first_free,
					range.start..last,
					true,
					Within::Never,
					ResourceGroup::All,
				);
				// SAFETY: evacuate only this block, without growing storage or moving its resources yet.
				unsafe { plan_selected(allocator, scan, &choice, moves, counters) }?;
				if moves.len() != before || counters.exhausted(scan.max_allocations, moves.len()) {
					// Proposals are not completed copies. Keep this phase until a later
					// begin_pass observes the block empty after explicit source release.
					return Ok(());
				}
				// Make room in earlier blocks before retrying the evacuation.
				let compact = selection(
					range.start..last,
					0..0,
					false,
					Within::Full,
					ResourceGroup::All,
				);
				// SAFETY: lower-offset, non-overlapping reservations in existing source blocks.
				unsafe { plan_selected(allocator, scan, &compact, moves, counters) }?;
				if moves.len() != before || counters.exhausted(scan.max_allocations, moves.len()) {
					return Ok(());
				}
				let fast = selection(
					range.start..first_free,
					range.start..first_free,
					true,
					Within::Never,
					ResourceGroup::All,
				);
				// SAFETY: consolidation stays in the not-yet-packed prefix.
				unsafe { plan_selected(allocator, scan, &fast, moves, counters) }?;
				if moves.len() != before || counters.exhausted(scan.max_allocations, moves.len()) {
					return Ok(());
				}
				// Native pinning/exclusions or fragmentation can prevent evacuation.
				// Finish the bounded feasible cleanup instead of spinning on this block.
				state.phase = Phase::Cleanup;
			}
			Phase::Pack(group) => {
				let before = moves.len();
				let choice = selection(
					range.start..first_free,
					first_free..range.end,
					false,
					Within::Never,
					group,
				);
				// SAFETY: class-compatible destinations still enforce actual native granularity.
				unsafe { plan_selected(allocator, scan, &choice, moves, counters) }?;
				if moves.len() != before || counters.exhausted(scan.max_allocations, moves.len()) {
					return Ok(());
				}
				let remaining = scan.sources.iter().any(|source| {
					(range.start..first_free).contains(&source.rank)
						&& group.admits(allocator.live[&source.serial].class)
				});
				if remaining {
					state.phase = Phase::Evacuate(group);
				} else {
					state.phase = match group {
						ResourceGroup::Textures => Phase::Pack(ResourceGroup::Buffers),
						ResourceGroup::Buffers => Phase::Pack(ResourceGroup::All),
						ResourceGroup::All => Phase::Cleanup,
					};
				}
			}
			Phase::Cleanup => {
				let before = moves.len();
				let choice = selection(range.clone(), 0..0, false, Within::Full, ResourceGroup::All);
				// SAFETY: final same-block compaction preserves the source domain and native extents.
				unsafe { plan_selected(allocator, scan, &choice, moves, counters) }?;
				if moves.len() == before && !counters.exhausted(scan.max_allocations, moves.len()) {
					state.phase = Phase::Done;
				}
				return Ok(());
			}
		}
	}
}
