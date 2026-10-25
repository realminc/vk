// Development-only oracle: execute the actual hash-pinned VMA preference function.
#include <vulkan/vulkan_core.h>
#define VMA_VULKAN_HEADERS_ALREADY_INCLUDED 1
#define VMA_IMPLEMENTATION
#define VMA_STATIC_VULKAN_FUNCTIONS 0
#define VMA_DYNAMIC_VULKAN_FUNCTIONS 0
#include <vk_mem_alloc.h>
#include <cstdio>

int main() {
	const VmaMemoryUsage usages[] = { VMA_MEMORY_USAGE_UNKNOWN, VMA_MEMORY_USAGE_AUTO,
		VMA_MEMORY_USAGE_AUTO_PREFER_DEVICE, VMA_MEMORY_USAGE_AUTO_PREFER_HOST };
	const unsigned properties[] = { 0, 2, 8, 64, 128 };
	for (unsigned usage = 0; usage < 4; ++usage)
	for (unsigned host = 0; host < 3; ++host)
	for (unsigned transfer = 0; transfer < 2; ++transfer)
	for (unsigned device = 0; device < 2; ++device)
	for (unsigned integrated = 0; integrated < 2; ++integrated)
	for (unsigned required : properties)
	for (unsigned preferred : properties) {
		if ((usage == 0 && (host != 0 || transfer != 0)) || (host == 0 && transfer != 0)) continue;
		VmaAllocationCreateInfo info = {};
		info.usage = usages[usage];
		info.requiredFlags = required;
		info.preferredFlags = preferred;
		if (host == 1) info.flags |= VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT;
		if (host == 2) info.flags |= VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT;
		if (transfer) info.flags |= VMA_ALLOCATION_CREATE_HOST_ACCESS_ALLOW_TRANSFER_INSTEAD_BIT;
		VkMemoryPropertyFlags resultRequired = 0, resultPreferred = 0, resultAvoided = 0;
		if (!FindMemoryPreferences(integrated != 0, info,
			VmaBufferImageUsage(device ? VK_BUFFER_USAGE_STORAGE_BUFFER_BIT : VK_BUFFER_USAGE_TRANSFER_SRC_BIT),
			resultRequired, resultPreferred, resultAvoided)) return 1;
		std::printf("%u\t%u\t%u\t%u\t%u\t%u\t%u\t%u\t%u\t%u\n",
			usage, host, transfer, device, integrated, required, preferred,
			resultRequired, resultPreferred, resultAvoided);
	}
}
