// Development-only, actual VMA on a real Vulkan driver. No synthetic PFNs.
// Native VMA and Khronos headers are supplied by the repository's hash-pinned fetchers.
#define VK_NO_PROTOTYPES
#include <vulkan/vulkan_core.h>
#define VMA_VULKAN_HEADERS_ALREADY_INCLUDED 1
#define VMA_STATIC_VULKAN_FUNCTIONS 0
#define VMA_DYNAMIC_VULKAN_FUNCTIONS 1
#define VMA_IMPLEMENTATION
#include "vk_mem_alloc.h"
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstring>
#include <functional>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>
#ifdef _WIN32
#include <windows.h>
#else
#include <dlfcn.h>
#endif

using Clock = std::chrono::steady_clock;
constexpr uint64_t BlockSize = 64ull * 1024 * 1024;
constexpr uint64_t Sizes[] = {64, 256, 4096, 16384, 65536};

// Clang compiler barrier corresponding to Rust's black_box. Returned placement
// tokens stay observable even in the host-only virtual workload.
template<class T> static void opaque(const T& value) {
	__asm__ __volatile__("" : : "g"(&value) : "memory");
}

static void require(bool condition, const char* message) {
	if (!condition) throw std::runtime_error(message);
}

static void check(VkResult result) {
	if (result != VK_SUCCESS) throw std::runtime_error("Vulkan/VMA status " + std::to_string(result));
}

static uint64_t elapsed(Clock::time_point start) {
	return std::chrono::duration_cast<std::chrono::nanoseconds>(Clock::now() - start).count();
}

template<class T> static T resolve(PFN_vkGetInstanceProcAddr get, VkInstance instance, const char* name) {
	auto pointer = get(instance, name);
	require(pointer != nullptr, name);
	return reinterpret_cast<T>(pointer);
}

template<size_t N> static void array(const uint8_t (&values)[N]) {
	std::cout << '[';
	for (size_t i = 0; i < N; ++i) { if (i) std::cout << ','; std::cout << unsigned(values[i]); }
	std::cout << ']';
}

struct Token {
	VkBuffer buffer{};
	VkImage image{};
	VmaAllocation allocation{};
	VmaVirtualAllocation virtualAllocation{};
	VkDeviceSize offset{}, size{};
};

int main(int argc, char** argv) try {
	std::string mode = "virtual";
	std::string trace = "reverse", algorithm = "tlsf", strategy = "default", mapping = "none";
	uint32_t deviceIndex = 0, memoryType = 0, api = VK_API_VERSION_1_2;
	size_t batches = 32, batchSize = 128;
	for (int i = 1; i < argc; i += 2) {
		require(i + 1 < argc, "missing argument value");
		std::string key = argv[i], value = argv[i + 1];
		if (key == "--case") mode = value;
		else if (key == "--device") deviceIndex = std::stoul(value);
		else if (key == "--memory-type") memoryType = std::stoul(value);
		else if (key == "--batches") batches = std::stoul(value);
		else if (key == "--batch-size") batchSize = std::stoul(value);
		else if (key == "--trace") trace = value;
		else if (key == "--algorithm") algorithm = value;
		else if (key == "--strategy") strategy = value;
		else if (key == "--mapping") mapping = value;
		else if (key == "--api") {
			require(value.size() == 3 && value[0] == '1' && value[1] == '.' && value[2] >= '1' && value[2] <= '4', "API must be 1.1..1.4");
			api = VK_MAKE_API_VERSION(0, 1, value[2] - '0', 0);
		} else throw std::runtime_error("unknown argument");
	}
	require(batches > 0 && batches <= 4096 && batchSize > 0 && batchSize <= 256 && memoryType < 32, "invalid workload bounds");
	require(mode == "virtual" || mode == "buffer-pool" || mode == "buffer-separate" || mode == "image-pool" || mode == "image-separate", "unknown VMA workload");
	require(trace == "reverse" || trace == "fifo" || trace == "shuffle" || trace == "churn", "invalid trace");
	require(algorithm == "tlsf" || algorithm == "linear", "invalid algorithm");
	require(mapping == "none" || (mapping == "cycle" && mode != "virtual"), "invalid mapping policy");
	require(trace != "churn" || batchSize >= 2, "churn needs two live entries");
	uint32_t strategyFlags = 0;
	if (strategy == "min-memory") strategyFlags = VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT;
	else if (strategy == "min-time") strategyFlags = VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT;
	else if (strategy == "min-offset") strategyFlags = VMA_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT;
	else require(strategy == "default", "invalid strategy");
	const bool pooled = mode == "buffer-pool" || mode == "image-pool";
	const bool images = mode == "image-pool" || mode == "image-separate";
	require(pooled || mode == "virtual" || (algorithm == "tlsf" && strategy == "default"), "separate storage has no placement algorithm");
	VkInstance instance{};
	VkDevice device{};
	VkPhysicalDevice physical{};
	VkPhysicalDeviceProperties properties{};
	VkPhysicalDeviceIDProperties ids{VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES};
	uint32_t deviceCount = 0, family = 0;
	PFN_vkGetInstanceProcAddr get{};
	PFN_vkDestroyInstance destroyInstance{};
	PFN_vkDestroyDevice destroyDevice{};
	PFN_vkGetBufferMemoryRequirements bufferRequirements{};
	PFN_vkGetImageMemoryRequirements imageRequirements{};
	VmaAllocator allocator{};
	VmaPool pool{};
	VmaVirtualBlock virtualBlock{};
	if (mode != "virtual") {
#ifdef _WIN32
		auto library = LoadLibraryA("vulkan-1.dll");
		require(library != nullptr, "vulkan-1.dll unavailable");
		get = reinterpret_cast<PFN_vkGetInstanceProcAddr>(GetProcAddress(library, "vkGetInstanceProcAddr"));
#else
#if defined(__APPLE__)
		auto library = dlopen("libvulkan.dylib", RTLD_NOW | RTLD_LOCAL);
#elif defined(__ANDROID__)
		auto library = dlopen("libvulkan.so", RTLD_NOW | RTLD_LOCAL);
#else
		auto library = dlopen("libvulkan.so.1", RTLD_NOW | RTLD_LOCAL);
#endif
		require(library != nullptr, "Vulkan loader unavailable");
		get = reinterpret_cast<PFN_vkGetInstanceProcAddr>(dlsym(library, "vkGetInstanceProcAddr"));
#endif
		require(get != nullptr, "vkGetInstanceProcAddr unavailable");
		auto createInstance = resolve<PFN_vkCreateInstance>(get, nullptr, "vkCreateInstance");
		VkApplicationInfo app{VK_STRUCTURE_TYPE_APPLICATION_INFO}; app.apiVersion = api;
		VkInstanceCreateInfo create{VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO}; create.pApplicationInfo = &app;
		check(createInstance(&create, nullptr, &instance));
		destroyInstance = resolve<PFN_vkDestroyInstance>(get, instance, "vkDestroyInstance");
		auto enumerate = resolve<PFN_vkEnumeratePhysicalDevices>(get, instance, "vkEnumeratePhysicalDevices");
		check(enumerate(instance, &deviceCount, nullptr));
		std::vector<VkPhysicalDevice> devices(deviceCount);
		check(enumerate(instance, &deviceCount, devices.data()));
		require(deviceIndex < deviceCount, "device index absent"); physical = devices[deviceIndex];
		auto getProperties = resolve<PFN_vkGetPhysicalDeviceProperties>(get, instance, "vkGetPhysicalDeviceProperties");
		getProperties(physical, &properties); require(properties.apiVersion >= api, "physical API below request");
		VkPhysicalDeviceProperties2 props2{VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2}; props2.pNext = &ids;
		resolve<PFN_vkGetPhysicalDeviceProperties2>(get, instance, "vkGetPhysicalDeviceProperties2")(physical, &props2);
		uint32_t queueCount = 0;
		auto getQueues = resolve<PFN_vkGetPhysicalDeviceQueueFamilyProperties>(get, instance, "vkGetPhysicalDeviceQueueFamilyProperties");
		getQueues(physical, &queueCount, nullptr);
		std::vector<VkQueueFamilyProperties> queues(queueCount); getQueues(physical, &queueCount, queues.data());
		while (family < queueCount && (!queues[family].queueCount || !(queues[family].queueFlags & VK_QUEUE_COMPUTE_BIT))) ++family;
		require(family < queueCount, "compute queue absent");
		float priority = 1.0f;
		VkDeviceQueueCreateInfo queue{VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO}; queue.queueFamilyIndex = family; queue.queueCount = 1; queue.pQueuePriorities = &priority;
		VkDeviceCreateInfo createDevice{VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO}; createDevice.queueCreateInfoCount = 1; createDevice.pQueueCreateInfos = &queue;
		check(resolve<PFN_vkCreateDevice>(get, instance, "vkCreateDevice")(physical, &createDevice, nullptr, &device));
		auto getDevice = resolve<PFN_vkGetDeviceProcAddr>(get, instance, "vkGetDeviceProcAddr");
		destroyDevice = reinterpret_cast<PFN_vkDestroyDevice>(getDevice(device, "vkDestroyDevice"));
		require(destroyDevice != nullptr, "vkDestroyDevice unavailable");
		bufferRequirements = reinterpret_cast<PFN_vkGetBufferMemoryRequirements>(getDevice(device, "vkGetBufferMemoryRequirements"));
		imageRequirements = reinterpret_cast<PFN_vkGetImageMemoryRequirements>(getDevice(device, "vkGetImageMemoryRequirements"));
		require(bufferRequirements && imageRequirements, "native requirements PFNs unavailable");
		VmaVulkanFunctions functions{}; functions.vkGetInstanceProcAddr = get; functions.vkGetDeviceProcAddr = getDevice;
		VmaAllocatorCreateInfo allocatorInfo{}; allocatorInfo.instance = instance; allocatorInfo.physicalDevice = physical;
		allocatorInfo.device = device; allocatorInfo.vulkanApiVersion = api; allocatorInfo.pVulkanFunctions = &functions;
		check(vmaCreateAllocator(&allocatorInfo, &allocator));
		VkPhysicalDeviceMemoryProperties memory{};
		resolve<PFN_vkGetPhysicalDeviceMemoryProperties>(get, instance, "vkGetPhysicalDeviceMemoryProperties")(physical, &memory);
		require(memoryType < memory.memoryTypeCount && (memory.memoryTypes[memoryType].propertyFlags & VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT), "host-visible memory type required");
		if (pooled) {
			VmaPoolCreateInfo info{}; info.memoryTypeIndex = memoryType; info.blockSize = BlockSize;
			info.minBlockCount = 1; info.maxBlockCount = 1;
			if (algorithm == "linear") info.flags = VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT;
			check(vmaCreatePool(allocator, &info, &pool));
		}
	} else {
		VmaVirtualBlockCreateInfo info{}; info.size = BlockSize;
		if (algorithm == "linear") info.flags = VMA_VIRTUAL_BLOCK_CREATE_LINEAR_ALGORITHM_BIT;
		check(vmaCreateVirtualBlock(&info, &virtualBlock));
	}
	std::vector<Token> tokens; tokens.reserve(batchSize);
	std::vector<std::pair<size_t, size_t>> swaps;
	std::vector<size_t> order(batchSize); for (size_t i = 0; i < batchSize; ++i) order[i] = i;
	auto addSwap = [&](size_t left, size_t right) { swaps.emplace_back(left, right); std::swap(order[left], order[right]); };
	if (trace == "fifo") { for (size_t i = 0; i < batchSize / 2; ++i) addSwap(i, batchSize - 1 - i); }
	else if (trace == "shuffle") {
		uint64_t random = 0x6a09e667f3bcc909ull;
		for (size_t i = batchSize - 1; i > 0; --i) { random ^= random << 13; random ^= random >> 7; random ^= random << 17; addSwap(i, random % (i + 1)); }
	} else if (trace == "churn") {
		for (size_t i = 0; i < batchSize; ++i) {
			size_t source = i < (batchSize + 1) / 2 ? i * 2 : (i - (batchSize + 1) / 2) * 2 + 1;
			addSwap(i, std::find(order.begin(), order.end(), source) - order.begin());
		}
	}
	uint64_t traceChecksum = 0xcbf29ce484222325ull;
	for (auto i = order.rbegin(); i != order.rend(); ++i) traceChecksum = (traceChecksum ^ *i) * 0x100000001b3ull;
	const size_t replacements = trace == "churn" ? batchSize / 2 : 0;
	uint64_t allocateNs = 0, freeNs = 0, mapNs = 0, checksum = 0, peakNative = 0, peakBytes = 0, peakAllocationBytes = 0;
	auto allocate = [&](uint64_t size) {
		Token token{}; token.size = size; opaque(token.size);
		if (virtualBlock) {
			VmaVirtualAllocationCreateInfo info{}; info.size = size; info.alignment = 64;
			info.flags = strategyFlags;
			// Pinned VMA's linear statistics identifies live entries by non-null
			// user data. This sentinel is never dereferenced or consumer-owned.
			info.pUserData = reinterpret_cast<void*>(uintptr_t(1));
			check(vmaVirtualAllocate(virtualBlock, &info, &token.virtualAllocation, &token.offset));
		} else {
			VmaAllocationCreateInfo options{}; options.requiredFlags = VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT;
			options.memoryTypeBits = 1u << memoryType; options.pool = pool; options.flags = strategyFlags;
			if (!pool) options.flags |= VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT;
			if (images) {
				VkImageCreateInfo info{VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO}; info.imageType = VK_IMAGE_TYPE_2D;
				info.format = VK_FORMAT_R8_UNORM; info.extent = {64, uint32_t(size / 64), 1};
				info.mipLevels = info.arrayLayers = 1; info.samples = VK_SAMPLE_COUNT_1_BIT;
				info.tiling = VK_IMAGE_TILING_OPTIMAL; info.usage = VK_IMAGE_USAGE_TRANSFER_SRC_BIT | VK_IMAGE_USAGE_TRANSFER_DST_BIT;
				info.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
				check(vmaCreateImage(allocator, &info, &options, &token.image, &token.allocation, nullptr));
			} else {
				VkBufferCreateInfo info{VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO}; info.size = size;
				info.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT; info.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
				check(vmaCreateBuffer(allocator, &info, &options, &token.buffer, &token.allocation, nullptr));
			}
		}
		return token;
	};
	auto release = [&](Token token) {
		opaque(token);
		if (virtualBlock) vmaVirtualFree(virtualBlock, token.virtualAllocation);
		else if (images) vmaDestroyImage(allocator, token.image, token.allocation);
		else vmaDestroyBuffer(allocator, token.buffer, token.allocation);
	};
	auto inspect = [&]() {
		if (virtualBlock) {
			auto sorted = tokens; std::sort(sorted.begin(), sorted.end(), [](auto a, auto b) { return a.offset < b.offset; });
			uint64_t end = 0;
			for (auto token : sorted) { require(token.offset >= end && token.offset % 64 == 0, "virtual overlap/alignment"); end = token.offset + token.size; require(end <= BlockSize, "virtual extent"); }
			VmaStatistics stats{}; vmaGetVirtualBlockStatistics(virtualBlock, &stats); require(stats.allocationCount == batchSize, "virtual count");
			peakAllocationBytes = stats.allocationBytes;
		} else {
			struct Range { VkDeviceMemory memory; uint64_t offset, size; };
			std::vector<Range> ranges; ranges.reserve(tokens.size());
			for (const auto& token : tokens) {
				VmaAllocationInfo info{}; vmaGetAllocationInfo(allocator, token.allocation, &info);
				VkMemoryRequirements requirements{};
				if (images) imageRequirements(device, token.image, &requirements); else bufferRequirements(device, token.buffer, &requirements);
				require(info.memoryType == memoryType && info.size >= requirements.size && info.offset % requirements.alignment == 0 && (requirements.memoryTypeBits & (1u << memoryType)), "native requirements mismatch");
				const uint64_t nativeSize = pool ? BlockSize : info.size;
				require(info.offset <= nativeSize && info.size <= nativeSize - info.offset, "native allocation exceeds backing extent");
				ranges.push_back({info.deviceMemory, info.offset, info.size});
			}
			std::sort(ranges.begin(), ranges.end(), [](auto a, auto b) { return std::less<VkDeviceMemory>{}(a.memory, b.memory) || (a.memory == b.memory && a.offset < b.offset); });
			for (size_t i = 1; i < ranges.size(); ++i) require(ranges[i-1].memory != ranges[i].memory || ranges[i-1].offset + ranges[i-1].size <= ranges[i].offset, "native overlap");
			VmaTotalStatistics stats{}; vmaCalculateStatistics(allocator, &stats); require(stats.total.statistics.allocationCount == batchSize, "native count");
			peakNative = stats.total.statistics.blockCount; peakBytes = stats.total.statistics.blockBytes; peakAllocationBytes = stats.total.statistics.allocationBytes;
		}
	};
	for (size_t batch = 0; batch < batches; ++batch) {
		auto start = Clock::now();
		for (size_t i = 0; i < batchSize; ++i) {
			tokens.push_back(allocate(Sizes[i % 5]));
		}
		allocateNs += elapsed(start);
		if (batch == 0) inspect();
		for (size_t i = 0; i < batchSize; ++i) checksum += Sizes[i % 5];
		for (auto swap : swaps) std::swap(tokens[swap.first], tokens[swap.second]);
		if (replacements) {
			start = Clock::now();
			for (size_t i = 0; i < replacements; ++i) { auto token = tokens.back(); tokens.pop_back(); release(token); }
			freeNs += elapsed(start);
			start = Clock::now();
			for (size_t i = replacements; i > 0; --i) tokens.push_back(allocate(Sizes[((i - 1) * 2 + 1) % 5]));
			allocateNs += elapsed(start);
			for (size_t i = 0; i < replacements; ++i) checksum += Sizes[(i * 2 + 1) % 5];
			if (batch == 0) inspect();
		}
		if (mapping == "cycle") {
			start = Clock::now();
			for (size_t i = 0; i < tokens.size(); ++i) {
				void* pointer{}; check(vmaMapMemory(allocator, tokens[i].allocation, &pointer));
				uint64_t pattern = uint64_t(i) ^ 0x9e3779b97f4a7c15ull; opaque(pattern);
				std::memcpy(pointer, &pattern, sizeof(pattern)); vmaUnmapMemory(allocator, tokens[i].allocation);
			}
			mapNs += elapsed(start);
			for (size_t i = 0; i < tokens.size(); ++i) {
				void* pointer{}; check(vmaMapMemory(allocator, tokens[i].allocation, &pointer));
				uint64_t observed{}; std::memcpy(&observed, pointer, sizeof(observed)); vmaUnmapMemory(allocator, tokens[i].allocation);
				require(observed == (uint64_t(i) ^ 0x9e3779b97f4a7c15ull), "mapped payload mismatch");
			}
		}
		start = Clock::now();
		while (!tokens.empty()) {
			auto token = tokens.back(); tokens.pop_back();
			release(token);
		}
		freeNs += elapsed(start);
		if (virtualBlock) { VmaStatistics stats{}; vmaGetVirtualBlockStatistics(virtualBlock, &stats); require(stats.allocationCount == 0, "virtual leak"); }
		else { VmaTotalStatistics stats{}; vmaCalculateStatistics(allocator, &stats); require(stats.total.statistics.allocationCount == 0, "native leak"); }
	}
	if (virtualBlock) vmaDestroyVirtualBlock(virtualBlock);
	else {
		if (pool) vmaDestroyPool(allocator, pool);
		VmaTotalStatistics stats{}; vmaCalculateStatistics(allocator, &stats); require(stats.total.statistics.blockCount == 0, "native backing leak");
		vmaDestroyAllocator(allocator); destroyDevice(device, nullptr); destroyInstance(instance, nullptr);
	}
	std::cout << "{\"schema\":\"vk.sdk.sample.v2\",\"backend\":\"vma\",\"case\":\"" << mode
		<< "\",\"trace\":\"" << trace << "\",\"algorithm\":\"" << algorithm << "\",\"strategy\":\"" << strategy << "\",\"mapping\":\"" << mapping
		<< "\",\"api_requested\":" << api << ",\"device_index\":" << deviceIndex << ",\"memory_type\":" << memoryType
		<< ",\"batches\":" << batches << ",\"batch_size\":" << batchSize << ",\"block_size\":" << BlockSize
		<< ",\"sizes\":[64,256,4096,16384,65536],\"identity\":";
	if (mode == "virtual") std::cout << "null";
	else {
		std::cout << "{\"device_uuid\":"; array(ids.deviceUUID); std::cout << ",\"driver_uuid\":"; array(ids.driverUUID);
		std::cout << ",\"vendor_id\":" << properties.vendorID << ",\"device_id\":" << properties.deviceID
			<< ",\"device_type\":" << properties.deviceType << ",\"api_version\":" << properties.apiVersion
			<< ",\"driver_version\":" << properties.driverVersion << ",\"queue_family\":" << family << ",\"device_count\":" << deviceCount << '}';
	}
	std::cout << ",\"measurements\":{\"allocations\":" << batches * (batchSize + replacements) << ",\"allocate_ns\":" << allocateNs
		<< ",\"free_ns\":" << freeNs << ",\"requested_byte_checksum\":" << checksum
		<< ",\"trace_checksum\":" << traceChecksum << ",\"map_ns\":" << mapNs << ",\"map_cycles\":" << (mapping == "cycle" ? batches * batchSize : 0)
		<< ",\"peak\":{\"live_count\":" << batchSize << ",\"native_count\":" << peakNative << ",\"native_bytes\":" << peakBytes
		<< ",\"allocation_bytes\":" << peakAllocationBytes << "},\"fragmented_peak\":";
	if (replacements) std::cout << "{\"live_count\":" << batchSize << ",\"native_count\":" << peakNative << ",\"native_bytes\":" << peakBytes << ",\"allocation_bytes\":" << peakAllocationBytes << '}';
	else std::cout << "null";
	std::cout << "}}\n";
	return 0;
} catch (const std::exception& error) {
	std::cerr << "VMA SDK failed: " << error.what() << '\n'; return 1;
}
