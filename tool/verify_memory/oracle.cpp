// Development-only VMA virtual allocation oracle. No Vulkan loader or GPU calls.
#include <vulkan/vulkan_core.h>
#define VMA_VULKAN_HEADERS_ALREADY_INCLUDED 1
#define VMA_STATIC_VULKAN_FUNCTIONS 0
#define VMA_DYNAMIC_VULKAN_FUNCTIONS 0
#define VMA_IMPLEMENTATION
#include "vk_mem_alloc.h"
#include <iostream>
#include <string>
#include <sstream>
#include <unordered_map>

int main() {
    VmaVirtualBlock block = nullptr;
    std::unordered_map<uint64_t, VmaVirtualAllocation> live;
    std::string op;
    while (std::cin >> op) {
        if (op == "b") {
            uint64_t size; std::cin >> size;
            std::string rest; std::getline(std::cin, rest);
            unsigned linear = 0; std::istringstream(rest) >> linear;
            VmaVirtualBlockCreateInfo info{}; info.size = size;
            if (linear) info.flags = VMA_VIRTUAL_BLOCK_CREATE_LINEAR_ALGORITHM_BIT;
            if (block || vmaCreateVirtualBlock(&info, &block) != VK_SUCCESS) return 2;
            std::cout << "b " << size;
        } else if (op == "a") {
            uint64_t id, size, alignment; unsigned strategy;
            std::cin >> id >> size >> alignment >> strategy;
            VmaVirtualAllocationCreateInfo info{};
            info.size = size; info.alignment = alignment;
            // Linear detailed statistics uses non-null userData to identify live entries.
            // This sentinel is never dereferenced and is not an algorithm adaptation.
            info.pUserData = reinterpret_cast<void*>(uintptr_t(1));
            std::string rest; std::getline(std::cin, rest);
            unsigned upper = 0; std::istringstream(rest) >> upper;
            switch (strategy) {
                case 0: break;
                case 1: info.flags = VMA_VIRTUAL_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT; break;
                case 2: info.flags = VMA_VIRTUAL_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT; break;
                case 3: info.flags = VMA_VIRTUAL_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT; break;
                default: return 3;
            }
            if (upper) info.flags |= VMA_VIRTUAL_ALLOCATION_CREATE_UPPER_ADDRESS_BIT;
            VkDeviceSize offset = 0;
            VmaVirtualAllocation allocation = VK_NULL_HANDLE;
            bool ok = vmaVirtualAllocate(block, &info, &allocation, &offset) == VK_SUCCESS;
            if (ok) live[id] = allocation;
            std::cout << "a " << id << ' ' << ok << ' ' << (ok ? offset : 0) << ' ' << size;
        } else if (op == "f") {
            uint64_t id; std::cin >> id;
            auto it = live.find(id);
            if (it != live.end()) { vmaVirtualFree(block, it->second); live.erase(it); }
            std::cout << "f " << id;
        } else if (op == "c") {
            vmaClearVirtualBlock(block); live.clear(); std::cout << 'c';
        } else return 4;
        VmaDetailedStatistics stats{};
        vmaCalculateVirtualBlockStatistics(block, &stats);
        std::cout << ' ' << stats.statistics.allocationCount << ' ' << stats.statistics.allocationBytes
                  << ' ' << stats.statistics.blockBytes - stats.statistics.allocationBytes
                  << ' ' << stats.unusedRangeCount << ' ' << (stats.unusedRangeCount ? stats.unusedRangeSizeMax : 0) << '\n';
    }
    if (!block) return 5;
    vmaClearVirtualBlock(block); vmaDestroyVirtualBlock(block);
}
