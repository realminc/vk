// Development-only native VMA oracle over a synthetic exact-PFN Vulkan driver.
// The real pinned allocator supplies placement, maps, cache translation and retries.
#include <vulkan/vulkan_core.h>
#define VMA_VULKAN_HEADERS_ALREADY_INCLUDED 1
#define VMA_STATIC_VULKAN_FUNCTIONS 0
#define VMA_DYNAMIC_VULKAN_FUNCTIONS 0
#define VMA_MAPPING_HYSTERESIS_ENABLED 0
#define VMA_IMPLEMENTATION
#include "vk_mem_alloc.h"
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>
#include <cstdlib>

static void require(bool value, unsigned line = __builtin_LINE()) {
    if (!value) {
        std::cerr << "native oracle requirement failed at line " << line << '\n';
        std::abort();
    }
}

struct Native {
    std::vector<uint8_t> bytes;
    uint32_t type;
    bool mapped = false;
};

struct Region {
    VkBuffer buffer;
    VmaAllocation allocation;
    unsigned resourceClass;
};

static std::map<uint64_t, Native> memory;
static std::map<uint64_t, uint64_t> buffers;
static uint64_t nextHandle = 100, atom, requestSize, alignment, granularity = 1;
static uint32_t bits, failureType;
static int failureSite = -1, failureCode;
static bool coherent, address, automaticPool;

template <class T> static uint64_t number(T handle) {
    return (uint64_t)(uintptr_t)handle;
}

template <class T> static T handle(uint64_t value) {
    return (T)(uintptr_t)value;
}

static VkResult status(int site, uint32_t type = 0) {
    return failureSite == site && (site != 0 || failureType == type) ? (VkResult)failureCode
                                                                     : VK_SUCCESS;
}

static VKAPI_ATTR void VKAPI_CALL properties(VkPhysicalDevice, VkPhysicalDeviceProperties *output) {
    *output = {};
    output->apiVersion = VK_API_VERSION_1_0;
    output->limits.nonCoherentAtomSize = atom;
    output->limits.bufferImageGranularity = granularity;
    output->limits.maxMemoryAllocationCount = 4096;
}

static VKAPI_ATTR void VKAPI_CALL memoryProperties(VkPhysicalDevice,
                                                   VkPhysicalDeviceMemoryProperties *output) {
    *output = {};
    output->memoryHeapCount = 1;
    output->memoryHeaps[0].size = automaticPool ? 512 * 1024 : 1ull << 30;
    output->memoryHeaps[0].flags = VK_MEMORY_HEAP_DEVICE_LOCAL_BIT;
    output->memoryTypeCount = 3;
    output->memoryTypes[0].propertyFlags = 2 | (coherent ? 4 : 0);
    output->memoryTypes[1].propertyFlags = 2 | 8 | (coherent ? 4 : 0);
    output->memoryTypes[2].propertyFlags = 1;
}

static VKAPI_ATTR VkResult VKAPI_CALL allocateMemory(VkDevice,
                                                     const VkMemoryAllocateInfo *info,
                                                     const VkAllocationCallbacks *,
                                                     VkDeviceMemory *output) {
    uint32_t flags = 0;
    if (info->pNext) {
        const auto* value = static_cast<const VkMemoryAllocateFlagsInfo*>(info->pNext);
        require(value->sType == VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO);
        require(!value->pNext && value->deviceMask == 0);
        flags = value->flags;
    }
    VkResult result = status(0, info->memoryTypeIndex);
    uint64_t id = result == VK_SUCCESS ? nextHandle++ : 0;
    if (result == VK_SUCCESS) {
        memory.emplace(id,
                       Native{std::vector<uint8_t>(info->allocationSize), info->memoryTypeIndex});
        *output = handle<VkDeviceMemory>(id);
    }
    std::cout << "e allocate " << info->memoryTypeIndex << ' ' << info->allocationSize << ' '
              << result << ' ' << id;
    if (address) std::cout << ' ' << flags;
    else require(flags == 0);
    std::cout << '\n';
    return result;
}

static VKAPI_ATTR void VKAPI_CALL freeMemory(VkDevice,
                                             VkDeviceMemory value,
                                             const VkAllocationCallbacks *) {
    // vkFreeMemory implicitly unmaps; VMA uses this for persistent dedicated memory.
    auto it = memory.find(number(value));
    require(it != memory.end());
    memory.erase(it);
    std::cout << "e free " << number(value) << '\n';
}

static VKAPI_ATTR VkResult VKAPI_CALL createBuffer(VkDevice,
                                                   const VkBufferCreateInfo *info,
                                                   const VkAllocationCallbacks *,
                                                   VkBuffer *output) {
    VkResult result = status(3);
    uint64_t id = result == VK_SUCCESS ? nextHandle++ : 0;
    if (result == VK_SUCCESS) {
        buffers.emplace(id, info->size);
        *output = handle<VkBuffer>(id);
    }
    std::cout << "e create " << id << ' ' << result << '\n';
    return result;
}

static VKAPI_ATTR void VKAPI_CALL destroyBuffer(VkDevice,
                                                VkBuffer buffer,
                                                const VkAllocationCallbacks *) {
    require(buffers.erase(number(buffer)) == 1);
    std::cout << "e destroy " << number(buffer) << '\n';
}

static VKAPI_ATTR void VKAPI_CALL requirements(VkDevice, VkBuffer, VkMemoryRequirements *output) {
    *output = {requestSize, alignment, bits};
}

static VKAPI_ATTR VkResult VKAPI_CALL bindBuffer(VkDevice,
                                                 VkBuffer buffer,
                                                 VkDeviceMemory value,
                                                 VkDeviceSize offset) {
    auto &native = memory.at(number(value));
    require(buffers.count(number(buffer)) && offset % alignment == 0);
    require(offset + requestSize <= native.bytes.size() && (bits & (1u << native.type)));
    VkResult result = status(2);
    std::cout << "e bind " << number(buffer) << ' ' << number(value) << ' ' << offset << ' '
              << result << '\n';
    return result;
}

static VKAPI_ATTR VkResult VKAPI_CALL mapMemory(VkDevice,
                                                VkDeviceMemory value,
                                                VkDeviceSize offset,
                                                VkDeviceSize size,
                                                VkMemoryMapFlags,
                                                void **output) {
    auto &native = memory.at(number(value));
    require(!native.mapped && offset == 0);
    require(size == VK_WHOLE_SIZE || size == native.bytes.size());
    VkResult result = status(1);
    if (result == VK_SUCCESS) {
        native.mapped = true;
        *output = native.bytes.data();
    }
    std::cout << "e map " << number(value) << ' ' << offset << ' ' << size << ' ' << result << '\n';
    return result;
}

static VKAPI_ATTR void VKAPI_CALL unmapMemory(VkDevice, VkDeviceMemory value) {
    auto &native = memory.at(number(value));
    require(native.mapped);
    native.mapped = false;
    std::cout << "e unmap " << number(value) << '\n';
}

static VkResult ranges(const char *op, uint32_t count, const VkMappedMemoryRange *data) {
    require(count == 1);
    std::cout << "e " << op << ' ' << number(data->memory) << ' ' << data->offset << ' '
              << data->size << '\n';
    return VK_SUCCESS;
}

static VKAPI_ATTR VkResult VKAPI_CALL flush(VkDevice,
                                            uint32_t count,
                                            const VkMappedMemoryRange *data) {
    return ranges("flush", count, data);
}

static VKAPI_ATTR VkResult VKAPI_CALL invalidate(VkDevice,
                                                 uint32_t count,
                                                 const VkMappedMemoryRange *data) {
    return ranges("invalidate", count, data);
}

// Exact image PFNs share only the synthetic driver's resource bookkeeping.
static VKAPI_ATTR VkResult VKAPI_CALL createImage(VkDevice device,
                                                  const VkImageCreateInfo *,
                                                  const VkAllocationCallbacks *callbacks,
                                                  VkImage *output) {
    VkBufferCreateInfo info{}; info.size = requestSize;
    VkBuffer buffer = VK_NULL_HANDLE;
    VkResult result = createBuffer(device, &info, callbacks, &buffer);
    if (result == VK_SUCCESS) *output = handle<VkImage>(number(buffer));
    return result;
}

static VKAPI_ATTR void VKAPI_CALL destroyImage(VkDevice device, VkImage image, const VkAllocationCallbacks *callbacks) {
    destroyBuffer(device, handle<VkBuffer>(number(image)), callbacks);
}

static VKAPI_ATTR void VKAPI_CALL imageRequirements(VkDevice device, VkImage image, VkMemoryRequirements *output) {
    requirements(device, handle<VkBuffer>(number(image)), output);
}

static VKAPI_ATTR VkResult VKAPI_CALL bindImage(VkDevice device, VkImage image, VkDeviceMemory memory, VkDeviceSize offset) {
    return bindBuffer(device, handle<VkBuffer>(number(image)), memory, offset);
}

static VKAPI_ATTR void VKAPI_CALL
copyBuffer(VkCommandBuffer, VkBuffer, VkBuffer, uint32_t, const VkBufferCopy *) {
    std::abort();
}

static void checkPointer(const VmaAllocationInfo &info, void *pointer) {
    auto &native = memory.at(number(info.deviceMemory));
    require(native.mapped && pointer == native.bytes.data() + info.offset);
}

int main() {
    std::cout << std::unitbuf;
    std::string line, op;
    std::getline(std::cin, line);
    std::istringstream config(line);
    int coherence, separate;
    config >> op >> atom >> coherence >> separate;
    int customPool = 0;
    config >> customPool;
    automaticPool = customPool == 2;
    int addressMode = 0;
    config >> addressMode;
    address = addressMode != 0;
    config >> granularity;
    VkDeviceSize heapLimit = 0;
    size_t minimum = 0, maximum = automaticPool ? 2 : 1;
    config >> heapLimit >> minimum >> maximum;
    require(op == "b");
    coherent = coherence != 0;
    VmaVulkanFunctions functions{};
    functions.vkGetPhysicalDeviceProperties = properties;
    functions.vkGetPhysicalDeviceMemoryProperties = memoryProperties;
    functions.vkAllocateMemory = allocateMemory;
    functions.vkFreeMemory = freeMemory;
    functions.vkMapMemory = mapMemory;
    functions.vkUnmapMemory = unmapMemory;
    functions.vkFlushMappedMemoryRanges = flush;
    functions.vkInvalidateMappedMemoryRanges = invalidate;
    functions.vkCreateBuffer = createBuffer;
    functions.vkDestroyBuffer = destroyBuffer;
    functions.vkGetBufferMemoryRequirements = requirements;
    functions.vkBindBufferMemory = bindBuffer;
    functions.vkCreateImage = createImage;
    functions.vkDestroyImage = destroyImage;
    functions.vkGetImageMemoryRequirements = imageRequirements;
    functions.vkBindImageMemory = bindImage;
    functions.vkCmdCopyBuffer = copyBuffer;
    VmaAllocatorCreateInfo create{};
    create.instance = handle<VkInstance>(0x1000);
    create.physicalDevice = handle<VkPhysicalDevice>(0x3000);
    create.device = handle<VkDevice>(0x2000);
    create.vulkanApiVersion = VK_API_VERSION_1_0;
    if (address) create.flags |= VMA_ALLOCATOR_CREATE_BUFFER_DEVICE_ADDRESS_BIT;
    create.pVulkanFunctions = &functions;
    VkDeviceSize heapLimits[VK_MAX_MEMORY_HEAPS];
    for (auto& value : heapLimits) value = VK_WHOLE_SIZE;
    heapLimits[0] = heapLimit;
    if (heapLimit) create.pHeapSizeLimit = heapLimits;
    VmaAllocator allocator = nullptr;
    require(vmaCreateAllocator(&create, &allocator) == VK_SUCCESS);
    VmaPool pool = nullptr;
    if (!separate) {
        VmaPoolCreateInfo p{};
        p.memoryTypeIndex = 0;
        if (customPool == 3) p.flags = VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT;
        p.blockSize = automaticPool ? 0 : 65536;
        p.minBlockCount = minimum;
        p.maxBlockCount = maximum;
        if (customPool)
            p.minAllocationAlignment = 256;
        require(vmaCreatePool(allocator, &p, &pool) == VK_SUCCESS);
    }
    std::map<uint64_t, Region> live;
    bool closed = false;
    while (std::getline(std::cin, line)) {
        std::istringstream input(line);
        input >> op;
        uint64_t id = 0, output[5]{};
        bool ok = true;
        if (op == "a") {
            uint32_t required, preferred, persistent, strategy;
            input >> id >> requestSize >> alignment >> bits >> required >> preferred >>
                persistent >> strategy;
            VkBufferCreateInfo buffer{};
            buffer.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
            buffer.size = requestSize;
            buffer.usage = VK_BUFFER_USAGE_STORAGE_BUFFER_BIT;
            if (address && id % 2 == 0) buffer.usage |= VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT;
            unsigned upper = 0, resourceClass = 0, existing = 0; input >> upper >> resourceClass >> existing;
            VmaAllocationCreateInfo a{};
            if (upper) a.flags |= VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT;
            a.requiredFlags = required;
            a.preferredFlags = preferred;
            a.memoryTypeBits = bits;
            a.pool = pool;
            if (separate)
                a.flags |= VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT;
            if (persistent)
                a.flags |= VMA_ALLOCATION_CREATE_MAPPED_BIT;
            switch (strategy) {
            case 0:
                break;
            case 1:
                a.flags |= VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT;
                break;
            case 2:
                a.flags |= VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT;
                break;
            case 3:
                a.flags |= VMA_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT;
                break;
            default:
                std::abort();
            }
            Region region{};
            VmaAllocationInfo info{};
            region.resourceClass = resourceClass;
            if (resourceClass == 0) {
                if (!existing) ok = vmaCreateBuffer(allocator, &buffer, &a, &region.buffer, &region.allocation, &info) == VK_SUCCESS;
                else {
                    VkResult status = createBuffer(create.device, &buffer, nullptr, &region.buffer);
                    const bool created = status == VK_SUCCESS;
                    bool allocated = false;
                    if (created) {
                        VkMemoryRequirements req{};
                        requirements(create.device, region.buffer, &req);
                        status = existing == 2 ? vmaAllocateMemory(allocator, &req, &a, &region.allocation, &info) : vmaAllocateMemoryForBuffer(allocator, region.buffer, &a, &region.allocation, &info);
                        allocated = status == VK_SUCCESS;
                    }
                    if (allocated) status = vmaBindBufferMemory(allocator, region.allocation, region.buffer);
                    ok = status == VK_SUCCESS;
                    if (!ok && created) {
                        require(buffers.count(number(region.buffer)));
                        destroyBuffer(create.device, region.buffer, nullptr);
                        if (allocated) vmaFreeMemory(allocator, region.allocation);
                    }
                }
            } else {
                require(resourceClass == 1 || resourceClass == 2);
                VkImageCreateInfo image{};
                image.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
                image.imageType = VK_IMAGE_TYPE_2D; image.format = VK_FORMAT_R8G8B8A8_UNORM;
                image.extent = {7, 9, 1}; image.mipLevels = 1; image.arrayLayers = 1;
                image.samples = VK_SAMPLE_COUNT_1_BIT;
                image.tiling = resourceClass == 1 ? VK_IMAGE_TILING_LINEAR : VK_IMAGE_TILING_OPTIMAL;
                image.usage = VK_IMAGE_USAGE_TRANSFER_DST_BIT;
                VkImage outputImage = VK_NULL_HANDLE;
                if (!existing) ok = vmaCreateImage(allocator, &image, &a, &outputImage, &region.allocation, &info) == VK_SUCCESS;
                else {
                    VkResult status = createImage(create.device, &image, nullptr, &outputImage);
                    const bool created = status == VK_SUCCESS;
                    bool allocated = false;
                    if (created) {
                        VkMemoryRequirements req{};
                        imageRequirements(create.device, outputImage, &req);
                        status = existing == 2 ? vmaAllocateMemory(allocator, &req, &a, &region.allocation, &info) : vmaAllocateMemoryForImage(allocator, outputImage, &a, &region.allocation, &info);
                        allocated = status == VK_SUCCESS;
                    }
                    if (allocated) status = vmaBindImageMemory(allocator, region.allocation, outputImage);
                    ok = status == VK_SUCCESS;
                    if (!ok && created) {
                        require(buffers.count(number(outputImage)));
                        destroyImage(create.device, outputImage, nullptr);
                        if (allocated) vmaFreeMemory(allocator, region.allocation);
                    }
                }
                region.buffer = handle<VkBuffer>(number(outputImage));
            }
            if (ok) {
                output[0] = number(region.buffer);
                output[1] = number(info.deviceMemory);
                output[2] = info.offset;
                output[3] = info.size;
                output[4] = info.memoryType;
                if (persistent)
                    checkPointer(info, info.pMappedData);
                require(live.emplace(id, region).second);
            }
        } else if (op == "x") {
            input >> failureSite >> failureType >> failureCode;
        } else if (op == "c") {
            require(live.empty() && !closed);
            if (pool)
                vmaDestroyPool(allocator, pool);
            vmaDestroyAllocator(allocator);
            closed = true;
        } else {
            input >> id;
            auto &region = live.at(id);
            if (op == "m") {
                void *pointer = nullptr;
                ok = vmaMapMemory(allocator, region.allocation, &pointer) == VK_SUCCESS;
                if (ok) {
                    VmaAllocationInfo info{};
                    vmaGetAllocationInfo(allocator, region.allocation, &info);
                    checkPointer(info, pointer);
                }
            } else if (op == "u")
                vmaUnmapMemory(allocator, region.allocation);
            else if (op == "h" || op == "i") {
                uint64_t offset, size;
                input >> offset >> size;
                VkResult result =
                    op == "h" ? vmaFlushAllocation(allocator, region.allocation, offset, size)
                              : vmaInvalidateAllocation(allocator, region.allocation, offset, size);
                require(result == VK_SUCCESS);
            } else if (op == "f") {
                if (region.resourceClass == 0) vmaDestroyBuffer(allocator, region.buffer, region.allocation);
                else vmaDestroyImage(allocator, handle<VkImage>(number(region.buffer)), region.allocation);
                live.erase(id);
            } else
                std::abort();
        }
        VmaTotalStatistics stats{};
        if (!closed)
            vmaCalculateStatistics(allocator, &stats);
        if (!closed && pool) {
            VmaStatistics poolStats{};
            vmaGetPoolStatistics(allocator, pool, &poolStats);
            if (customPool == 3) {
                // Pinned AddStatistics starts the second ring vector at the first
                // vector's null-prefix count. Keep this known undercount visible;
                // the independent model still checks detailed/live counts exactly.
                VmaDetailedStatistics detailedPool{};
                vmaCalculatePoolStatistics(allocator, pool, &detailedPool);
                require(detailedPool.statistics.allocationCount == stats.total.statistics.allocationCount);
                require(poolStats.allocationCount <= detailedPool.statistics.allocationCount);
                if (poolStats.allocationCount != detailedPool.statistics.allocationCount)
                    std::cout << "e linear_pool_count " << poolStats.allocationCount << ' '
                              << detailedPool.statistics.allocationCount << '\n';
            } else require(poolStats.allocationCount == stats.total.statistics.allocationCount);
            require(poolStats.allocationBytes == stats.total.statistics.allocationBytes);
            require(poolStats.blockCount == stats.total.statistics.blockCount);
            require(poolStats.blockBytes == stats.total.statistics.blockBytes);
        }
        size_t mapped = 0;
        for (const auto &entry : memory)
            mapped += entry.second.mapped;
        std::cout << "r " << op << ' ' << id << ' ' << ok;
        for (uint64_t value : output)
            std::cout << ' ' << value;
        std::cout << ' ' << stats.total.statistics.allocationCount << ' '
                  << stats.total.statistics.allocationBytes << ' '
                  << stats.total.statistics.blockCount << ' ' << stats.total.statistics.blockBytes
                  << ' ' << mapped << '\n';
    }
    require(closed && memory.empty() && buffers.empty() && live.empty());
}
