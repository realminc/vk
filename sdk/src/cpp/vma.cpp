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
	VmaAllocation allocation{};
	VmaVirtualAllocation virtualAllocation{};
	VkDeviceSize offset{}, size{};
};

int main(int argc, char** argv) try {
	std::string mode = "virtual";
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
		else if (key == "--api") {
			require(value.size() == 3 && value[0] == '1' && value[1] == '.' && value[2] >= '1' && value[2] <= '4', "API must be 1.1..1.4");
			api = VK_MAKE_API_VERSION(0, 1, value[2] - '0', 0);
		} else throw std::runtime_error("unknown argument");
	}
	require(batches > 0 && batches <= 4096 && batchSize > 0 && batchSize <= 256 && memoryType < 32, "invalid workload bounds");
	require(mode == "virtual" || mode == "buffer-pool" || mode == "buffer-separate", "unknown VMA workload");
	VkInstance instance{};
	VkDevice device{};
	VkPhysicalDevice physical{};
	VkPhysicalDeviceProperties properties{};
	VkPhysicalDeviceIDProperties ids{VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES};
	uint32_t deviceCount = 0, family = 0;
	PFN_vkGetInstanceProcAddr get{};
	PFN_vkDestroyInstance destroyInstance{};
	PFN_vkDestroyDevice destroyDevice{};
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
		VmaVulkanFunctions functions{}; functions.vkGetInstanceProcAddr = get; functions.vkGetDeviceProcAddr = getDevice;
		VmaAllocatorCreateInfo allocatorInfo{}; allocatorInfo.instance = instance; allocatorInfo.physicalDevice = physical;
		allocatorInfo.device = device; allocatorInfo.vulkanApiVersion = api; allocatorInfo.pVulkanFunctions = &functions;
		check(vmaCreateAllocator(&allocatorInfo, &allocator));
		VkPhysicalDeviceMemoryProperties memory{};
		resolve<PFN_vkGetPhysicalDeviceMemoryProperties>(get, instance, "vkGetPhysicalDeviceMemoryProperties")(physical, &memory);
		require(memoryType < memory.memoryTypeCount && (memory.memoryTypes[memoryType].propertyFlags & VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT), "host-visible memory type required");
		if (mode == "buffer-pool") {
			VmaPoolCreateInfo info{}; info.memoryTypeIndex = memoryType; info.blockSize = BlockSize;
			info.minBlockCount = 1; info.maxBlockCount = 1;
			check(vmaCreatePool(allocator, &info, &pool));
		}
	} else {
		VmaVirtualBlockCreateInfo info{}; info.size = BlockSize;
		check(vmaCreateVirtualBlock(&info, &virtualBlock));
	}
	std::vector<Token> tokens; tokens.reserve(batchSize);
	uint64_t allocateNs = 0, freeNs = 0, checksum = 0, peakNative = 0, peakBytes = 0, peakAllocationBytes = 0;
	for (size_t batch = 0; batch < batches; ++batch) {
		auto start = Clock::now();
		for (size_t i = 0; i < batchSize; ++i) {
			Token token{}; token.size = Sizes[i % 5];
			opaque(token.size);
			if (virtualBlock) {
				VmaVirtualAllocationCreateInfo info{}; info.size = token.size; info.alignment = 64;
				check(vmaVirtualAllocate(virtualBlock, &info, &token.virtualAllocation, &token.offset));
			} else {
				VkBufferCreateInfo info{VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO}; info.size = token.size;
				info.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT; info.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
				VmaAllocationCreateInfo options{}; options.requiredFlags = VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT;
				options.memoryTypeBits = 1u << memoryType; options.pool = pool;
				if (!pool) options.flags = VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT;
				check(vmaCreateBuffer(allocator, &info, &options, &token.buffer, &token.allocation, nullptr));
			}
			tokens.push_back(token);
		}
		allocateNs += elapsed(start);
		if (batch == 0 && virtualBlock) {
			auto sorted = tokens; std::sort(sorted.begin(), sorted.end(), [](auto a, auto b) { return a.offset < b.offset; });
			uint64_t end = 0;
			for (auto token : sorted) { require(token.offset >= end && token.offset % 64 == 0, "virtual overlap/alignment"); end = token.offset + token.size; require(end <= BlockSize, "virtual extent"); }
			VmaStatistics stats{}; vmaGetVirtualBlockStatistics(virtualBlock, &stats); require(stats.allocationCount == batchSize, "virtual count"); peakAllocationBytes = stats.allocationBytes;
		} else if (batch == 0) {
			VmaTotalStatistics stats{}; vmaCalculateStatistics(allocator, &stats); require(stats.total.statistics.allocationCount == batchSize, "native count");
			peakNative = stats.total.statistics.blockCount; peakBytes = stats.total.statistics.blockBytes; peakAllocationBytes = stats.total.statistics.allocationBytes;
		}
		for (size_t i = 0; i < batchSize; ++i) checksum += Sizes[i % 5];
		start = Clock::now();
		while (!tokens.empty()) {
			auto token = tokens.back(); tokens.pop_back();
			opaque(token);
			if (virtualBlock) vmaVirtualFree(virtualBlock, token.virtualAllocation);
			else vmaDestroyBuffer(allocator, token.buffer, token.allocation);
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
	std::cout << "{\"schema\":\"vk.sdk.sample.v1\",\"backend\":\"vma\",\"case\":\"" << mode
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
	std::cout << ",\"measurements\":{\"allocations\":" << batches * batchSize << ",\"allocate_ns\":" << allocateNs
		<< ",\"free_ns\":" << freeNs << ",\"requested_byte_checksum\":" << checksum
		<< ",\"peak\":{\"live_count\":" << batchSize << ",\"native_count\":" << peakNative << ",\"native_bytes\":" << peakBytes
		<< ",\"allocation_bytes\":" << peakAllocationBytes << "}}}\n";
	return 0;
} catch (const std::exception& error) {
	std::cerr << "VMA SDK failed: " << error.what() << '\n'; return 1;
}
