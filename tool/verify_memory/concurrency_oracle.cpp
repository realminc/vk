// Development-only VMA 3.4.0 concurrency oracle. No native driver or GPU calls.
// One independently locked driver ledger checks backing exclusion and cleanup.
#include <vulkan/vulkan_core.h>
#define VMA_VULKAN_HEADERS_ALREADY_INCLUDED 1
#define VMA_STATIC_VULKAN_FUNCTIONS 0
#define VMA_DYNAMIC_VULKAN_FUNCTIONS 0
#define VMA_MAPPING_HYSTERESIS_ENABLED 0
#define VMA_IMPLEMENTATION
#include "vk_mem_alloc.h"
#include <chrono>
#include <condition_variable>
#include <cstdlib>
#include <future>
#include <iostream>
#include <map>
#include <mutex>
#include <thread>
#include <vector>

static void require(bool value, unsigned line = __builtin_LINE()) {
    if (!value) {
        std::cerr << "concurrency oracle failed at line " << line << '\n';
        std::abort();
    }
}

template <class T> static uint64_t number(T value) { return (uint64_t)(uintptr_t)value; }
template <class T> static T handle(uint64_t value) { return (T)(uintptr_t)value; }

struct Memory { std::vector<uint8_t> bytes; bool mapped = false; unsigned users = 0; };
struct Buffer { uint64_t memory = 0; };
static std::mutex ledger;
static std::map<uint64_t, Memory> memories;
static std::map<uint64_t, Buffer> buffers;
static uint64_t nextHandle = 100;
static unsigned maps = 0, unmaps = 0;

struct Use {
    uint64_t memory;
    explicit Use(VkDeviceMemory value) : memory(number(value)) {
        std::lock_guard<std::mutex> lock(ledger);
        require(memories.at(memory).users++ == 0);
    }
    ~Use() {
        std::lock_guard<std::mutex> lock(ledger);
        require(memories.at(memory).users-- == 1);
    }
};

struct Gate {
    std::mutex mutex;
    std::condition_variable changed;
    uint64_t memory = 0;
    bool binding = false, entered = false, released = false;
    bool wait(VkDeviceMemory value, bool bind) {
        std::unique_lock<std::mutex> lock(mutex);
        if (memory != number(value) || binding != bind) return true;
        entered = true;
        changed.notify_all();
        return changed.wait_for(lock, std::chrono::seconds(10), [&] { return released; });
    }
    void arm(VkDeviceMemory value, bool bind) {
        std::lock_guard<std::mutex> lock(mutex);
        memory = number(value); binding = bind; entered = released = false;
    }
    void await() {
        std::unique_lock<std::mutex> lock(mutex);
        require(changed.wait_for(lock, std::chrono::seconds(10), [&] { return entered; }));
    }
    void release() {
        std::lock_guard<std::mutex> lock(mutex);
        released = true; changed.notify_all();
    }
} gate;

static VKAPI_ATTR void VKAPI_CALL properties(VkPhysicalDevice, VkPhysicalDeviceProperties* out) {
    *out = {}; out->apiVersion = VK_API_VERSION_1_0;
    out->limits.nonCoherentAtomSize = 64; out->limits.bufferImageGranularity = 1;
    out->limits.maxMemoryAllocationCount = 4096;
}
static VKAPI_ATTR void VKAPI_CALL memoryProperties(VkPhysicalDevice, VkPhysicalDeviceMemoryProperties* out) {
    *out = {}; out->memoryHeapCount = out->memoryTypeCount = 1;
    out->memoryHeaps[0].size = 1ull << 30;
    out->memoryTypes[0].propertyFlags = VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;
}
static VKAPI_ATTR VkResult VKAPI_CALL allocateMemory(VkDevice, const VkMemoryAllocateInfo* info,
                                                    const VkAllocationCallbacks*, VkDeviceMemory* out) {
    std::lock_guard<std::mutex> lock(ledger);
    require(info->memoryTypeIndex == 0);
    uint64_t id = nextHandle++;
    memories.emplace(id, Memory{std::vector<uint8_t>(info->allocationSize)});
    *out = handle<VkDeviceMemory>(id); return VK_SUCCESS;
}
static VKAPI_ATTR void VKAPI_CALL freeMemory(VkDevice, VkDeviceMemory value, const VkAllocationCallbacks*) {
    std::lock_guard<std::mutex> lock(ledger);
    const auto& memory = memories.at(number(value));
    require(!memory.mapped && memory.users == 0);
    for (const auto& entry : buffers) require(entry.second.memory != number(value));
    require(memories.erase(number(value)) == 1);
}
static VKAPI_ATTR VkResult VKAPI_CALL createBuffer(VkDevice, const VkBufferCreateInfo*,
                                                  const VkAllocationCallbacks*, VkBuffer* out) {
    std::lock_guard<std::mutex> lock(ledger);
    uint64_t id = nextHandle++; buffers.emplace(id, Buffer{});
    *out = handle<VkBuffer>(id); return VK_SUCCESS;
}
static VKAPI_ATTR void VKAPI_CALL destroyBuffer(VkDevice, VkBuffer buffer, const VkAllocationCallbacks*) {
    std::lock_guard<std::mutex> lock(ledger); require(buffers.erase(number(buffer)) == 1);
}
static VKAPI_ATTR void VKAPI_CALL requirements(VkDevice, VkBuffer buffer, VkMemoryRequirements* out) {
    std::lock_guard<std::mutex> lock(ledger); require(buffers.count(number(buffer)) == 1);
    *out = {64, 64, 1};
}
static VKAPI_ATTR VkResult VKAPI_CALL bindBuffer(VkDevice, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize offset) {
    Use use(memory);
    require(gate.wait(memory, true));
    std::lock_guard<std::mutex> lock(ledger);
    require(offset % 64 == 0 && offset + 64 <= memories.at(number(memory)).bytes.size());
    auto& target = buffers.at(number(buffer)); require(target.memory == 0); target.memory = number(memory);
    return VK_SUCCESS;
}
static VKAPI_ATTR VkResult VKAPI_CALL mapMemory(VkDevice, VkDeviceMemory memory, VkDeviceSize offset,
                                               VkDeviceSize size, VkMemoryMapFlags, void** out) {
    Use use(memory);
    require(gate.wait(memory, false));
    std::lock_guard<std::mutex> lock(ledger);
    auto& native = memories.at(number(memory));
    require(!native.mapped && offset == 0 && (size == VK_WHOLE_SIZE || size == native.bytes.size()));
    native.mapped = true; *out = native.bytes.data(); ++maps; return VK_SUCCESS;
}
static VKAPI_ATTR void VKAPI_CALL unmapMemory(VkDevice, VkDeviceMemory memory) {
    Use use(memory);
    std::lock_guard<std::mutex> lock(ledger);
    auto& native = memories.at(number(memory)); require(native.mapped); native.mapped = false; ++unmaps;
}
static VKAPI_ATTR VkResult VKAPI_CALL cache(VkDevice, uint32_t, const VkMappedMemoryRange*) { return VK_SUCCESS; }
static VKAPI_ATTR VkResult VKAPI_CALL createImage(VkDevice, const VkImageCreateInfo*, const VkAllocationCallbacks*, VkImage*) { std::abort(); }
static VKAPI_ATTR void VKAPI_CALL destroyImage(VkDevice, VkImage, const VkAllocationCallbacks*) { std::abort(); }
static VKAPI_ATTR void VKAPI_CALL imageRequirements(VkDevice, VkImage, VkMemoryRequirements*) { std::abort(); }
static VKAPI_ATTR VkResult VKAPI_CALL bindImage(VkDevice, VkImage, VkDeviceMemory, VkDeviceSize) { std::abort(); }
static VKAPI_ATTR void VKAPI_CALL copyBuffer(VkCommandBuffer, VkBuffer, VkBuffer, uint32_t, const VkBufferCopy*) { std::abort(); }

struct Fixture {
    VmaAllocator allocator = nullptr;
    VmaPool pool = nullptr;
    VmaAllocationCreateInfo options{};
    VkBufferCreateInfo info{};
    Fixture() {
        VmaVulkanFunctions f{};
        f.vkGetPhysicalDeviceProperties = properties; f.vkGetPhysicalDeviceMemoryProperties = memoryProperties;
        f.vkAllocateMemory = allocateMemory; f.vkFreeMemory = freeMemory;
        f.vkMapMemory = mapMemory; f.vkUnmapMemory = unmapMemory;
        f.vkFlushMappedMemoryRanges = f.vkInvalidateMappedMemoryRanges = cache;
        f.vkCreateBuffer = createBuffer; f.vkDestroyBuffer = destroyBuffer;
        f.vkGetBufferMemoryRequirements = requirements; f.vkBindBufferMemory = bindBuffer;
        f.vkCreateImage = createImage; f.vkDestroyImage = destroyImage;
        f.vkGetImageMemoryRequirements = imageRequirements; f.vkBindImageMemory = bindImage;
        f.vkCmdCopyBuffer = copyBuffer;
        VmaAllocatorCreateInfo a{}; a.pVulkanFunctions = &f; a.vulkanApiVersion = VK_API_VERSION_1_0;
        a.instance = handle<VkInstance>(0x1000); a.physicalDevice = handle<VkPhysicalDevice>(0x3000);
        a.device = handle<VkDevice>(0x2000);
        require(vmaCreateAllocator(&a, &allocator) == VK_SUCCESS);
        VmaPoolCreateInfo p{}; p.blockSize = 128;
        require(vmaCreatePool(allocator, &p, &pool) == VK_SUCCESS);
        options.pool = pool;
        info.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO; info.size = 64;
        info.usage = VK_BUFFER_USAGE_STORAGE_BUFFER_BIT;
    }
    void create(VkBuffer& buffer, VmaAllocation& allocation) {
        require(vmaCreateBuffer(allocator, &info, &options, &buffer, &allocation, nullptr) == VK_SUCCESS);
    }
    VkDeviceMemory memory(VmaAllocation allocation) {
        VmaAllocationInfo a{}; vmaGetAllocationInfo(allocator, allocation, &a); return a.deviceMemory;
    }
    ~Fixture() {
        vmaDestroyPool(allocator, pool); vmaDestroyAllocator(allocator);
        std::lock_guard<std::mutex> lock(ledger); require(memories.empty() && buffers.empty());
    }
};

static void independent(bool binding) {
    {
        Fixture f;
        VkBuffer first, neighbor, second, spare;
        VmaAllocation a, b, c, d;
        require(createBuffer(VK_NULL_HANDLE, &f.info, nullptr, &first) == VK_SUCCESS);
        require(vmaAllocateMemoryForBuffer(f.allocator, first, &f.options, &a, nullptr) == VK_SUCCESS);
        f.create(neighbor, b); f.create(second, c); f.create(spare, d);
        VkDeviceMemory firstMemory = f.memory(a), secondMemory = f.memory(c);
        require(firstMemory == f.memory(b) && secondMemory == f.memory(d) && firstMemory != secondMemory);
        vmaDestroyBuffer(f.allocator, spare, d);
        gate.arm(firstMemory, binding);
        std::thread native([&] {
            if (binding) require(vmaBindBufferMemory(f.allocator, a, first) == VK_SUCCESS);
            else { void* pointer = nullptr; require(vmaMapMemory(f.allocator, a, &pointer) == VK_SUCCESS); }
        });
        gate.await();
        auto placement = std::async(std::launch::async, [&] {
            VkBuffer buffer; VmaAllocation allocation; f.create(buffer, allocation);
            require(f.memory(allocation) == secondMemory);
            vmaDestroyBuffer(f.allocator, buffer, allocation);
        });
        bool progressed = placement.wait_for(std::chrono::seconds(3)) == std::future_status::ready;
        gate.release(); native.join(); placement.get(); require(progressed);
        if (!binding) vmaUnmapMemory(f.allocator, a);
        vmaDestroyBuffer(f.allocator, first, a); vmaDestroyBuffer(f.allocator, neighbor, b);
        vmaDestroyBuffer(f.allocator, second, c);
    }
    std::cout << "CONCURRENCY independent-" << (binding ? "bind" : "map") << " 1 0 0\n";
}

static void sameBacking() {
    {
        Fixture f; maps = unmaps = 0;
        VkBuffer first, second; VmaAllocation a, b;
        f.create(first, a);
        require(createBuffer(VK_NULL_HANDLE, &f.info, nullptr, &second) == VK_SUCCESS);
        require(vmaAllocateMemoryForBuffer(f.allocator, second, &f.options, &b, nullptr) == VK_SUCCESS);
        require(f.memory(a) == f.memory(b));
        gate.arm(f.memory(a), false);
        std::thread native([&] { void* pointer = nullptr; require(vmaMapMemory(f.allocator, a, &pointer) == VK_SUCCESS); });
        gate.await();
        std::promise<void> started;
        auto ready = started.get_future();
        auto binding = std::async(std::launch::async, [&] {
            started.set_value(); require(vmaBindBufferMemory(f.allocator, b, second) == VK_SUCCESS);
        });
        ready.get();
        bool excluded = binding.wait_for(std::chrono::milliseconds(100)) != std::future_status::ready;
        gate.release(); native.join(); binding.get(); require(excluded);
        vmaUnmapMemory(f.allocator, a);
        // One allocation is externally synchronized, as required by VMA. Four
        // explicit holds share one native map and only the last hold unmaps it.
        for (unsigned i = 0; i != 4; ++i) { void* pointer = nullptr; require(vmaMapMemory(f.allocator, a, &pointer) == VK_SUCCESS); }
        for (unsigned i = 0; i != 4; ++i) vmaUnmapMemory(f.allocator, a);
        require(maps == 2 && unmaps == 2);
        vmaDestroyBuffer(f.allocator, first, a); vmaDestroyBuffer(f.allocator, second, b);
    }
    std::cout << "CONCURRENCY same-backing 1 2 2\n";
}

int main() {
    independent(false); independent(true); sameBacking();
}
