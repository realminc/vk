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

static void require(bool value) {
    if (!value)
        std::abort();
}

struct Native {
    std::vector<uint8_t> bytes;
    uint32_t type;
    bool mapped = false;
};

struct Region {
    VkBuffer buffer;
    VmaAllocation allocation;
};

static std::map<uint64_t, Native> memory;
static std::map<uint64_t, uint64_t> buffers;
static uint64_t nextHandle = 100, atom, requestSize, alignment;
static uint32_t bits, failureType;
static int failureSite = -1, failureCode;
static bool coherent;

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
    output->limits.bufferImageGranularity = 1;
    output->limits.maxMemoryAllocationCount = 4096;
}

static VKAPI_ATTR void VKAPI_CALL memoryProperties(VkPhysicalDevice,
                                                   VkPhysicalDeviceMemoryProperties *output) {
    *output = {};
    output->memoryHeapCount = 1;
    output->memoryHeaps[0].size = 1ull << 30;
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
    require(!info->pNext);
    VkResult result = status(0, info->memoryTypeIndex);
    uint64_t id = result == VK_SUCCESS ? nextHandle++ : 0;
    if (result == VK_SUCCESS) {
        memory.emplace(id,
                       Native{std::vector<uint8_t>(info->allocationSize), info->memoryTypeIndex});
        *output = handle<VkDeviceMemory>(id);
    }
    std::cout << "e allocate " << info->memoryTypeIndex << ' ' << info->allocationSize << ' '
              << result << ' ' << id << '\n';
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

// Mandatory VMA PFNs for unused resource families fail immediately if reached.
static VKAPI_ATTR VkResult VKAPI_CALL createImage(VkDevice,
                                                  const VkImageCreateInfo *,
                                                  const VkAllocationCallbacks *,
                                                  VkImage *) {
    std::abort();
}

static VKAPI_ATTR void VKAPI_CALL destroyImage(VkDevice, VkImage, const VkAllocationCallbacks *) {
    std::abort();
}

static VKAPI_ATTR void VKAPI_CALL imageRequirements(VkDevice, VkImage, VkMemoryRequirements *) {
    std::abort();
}

static VKAPI_ATTR VkResult VKAPI_CALL bindImage(VkDevice, VkImage, VkDeviceMemory, VkDeviceSize) {
    std::abort();
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
    std::string line, op;
    std::getline(std::cin, line);
    std::istringstream config(line);
    int coherence, separate;
    config >> op >> atom >> coherence >> separate;
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
    create.pVulkanFunctions = &functions;
    VmaAllocator allocator = nullptr;
    require(vmaCreateAllocator(&create, &allocator) == VK_SUCCESS);
    VmaPool pool = nullptr;
    if (!separate) {
        VmaPoolCreateInfo p{};
        p.memoryTypeIndex = 0;
        p.blockSize = 65536;
        p.maxBlockCount = 1;
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
            VmaAllocationCreateInfo a{};
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
            ok = vmaCreateBuffer(
                     allocator, &buffer, &a, &region.buffer, &region.allocation, &info) ==
                 VK_SUCCESS;
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
                vmaDestroyBuffer(allocator, region.buffer, region.allocation);
                live.erase(id);
            } else
                std::abort();
        }
        VmaTotalStatistics stats{};
        if (!closed)
            vmaCalculateStatistics(allocator, &stats);
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
