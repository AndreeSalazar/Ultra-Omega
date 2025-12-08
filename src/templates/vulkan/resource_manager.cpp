/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN RESOURCE MANAGER - Gestión de recursos (wrappers)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: resource_manager.cpp
 * Descripción: Wrappers y utilidades para gestión de recursos Vulkan
 * 
 * USO: Este nodo puede ser heredado para usar funciones de gestión de recursos
 * Ejemplo: Conecta este nodo a otros para acceder a resource_manager con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include "buffers.cpp"

// ═══════════════════════════════════════════════════════════════════════════
// WRAPPER PARA BUFFERS
// ═══════════════════════════════════════════════════════════════════════════

struct BufferResource {
    VkBuffer buffer;
    VkDeviceMemory memory;
    VkDeviceSize size;
    void* mapped;
    
    void Release(VkDevice device) {
        if (mapped) {
            vkUnmapMemory(device, memory);
            mapped = nullptr;
        }
        if (buffer) {
            vkDestroyBuffer(device, buffer, nullptr);
            buffer = VK_NULL_HANDLE;
        }
        if (memory) {
            vkFreeMemory(device, memory, nullptr);
            memory = VK_NULL_HANDLE;
        }
    }
    
    void Map(VkDevice device, VkDeviceSize offset = 0, VkDeviceSize mapSize = VK_WHOLE_SIZE) {
        if (!mapped && buffer) {
            vkMapMemory(device, memory, offset, mapSize, 0, &mapped);
        }
    }
    
    void Unmap(VkDevice device) {
        if (mapped) {
            vkUnmapMemory(device, memory);
            mapped = nullptr;
        }
    }
    
    void CopyData(const void* data, size_t dataSize) {
        if (mapped && dataSize <= size) {
            memcpy(mapped, data, dataSize);
        }
    }
};

// Crear vertex buffer
BufferResource CreateVertexBuffer(VulkanContext& ctx, const void* data, VkDeviceSize size) {
    BufferResource buffer = {};
    buffer.size = size;
    
    // Crear staging buffer
    VkBuffer stagingBuffer;
    VkDeviceMemory stagingBufferMemory;
    createBuffer(ctx, size, VK_BUFFER_USAGE_TRANSFER_SRC_BIT,
        VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
        stagingBuffer, stagingBufferMemory);
    
    // Copiar datos al staging buffer
    void* mappedData;
    vkMapMemory(ctx.device, stagingBufferMemory, 0, size, 0, &mappedData);
    memcpy(mappedData, data, size);
    vkUnmapMemory(ctx.device, stagingBufferMemory);
    
    // Crear buffer final
    createBuffer(ctx, size, VK_BUFFER_USAGE_TRANSFER_DST_BIT | VK_BUFFER_USAGE_VERTEX_BUFFER_BIT,
        VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, buffer.buffer, buffer.memory);
    
    // Copiar del staging al buffer final (requiere command buffer)
    // Nota: En producción, usar copyBuffer con command list
    
    // Limpiar staging buffer
    vkDestroyBuffer(ctx.device, stagingBuffer, nullptr);
    vkFreeMemory(ctx.device, stagingBufferMemory, nullptr);
    
    return buffer;
}

// Crear index buffer
BufferResource CreateIndexBuffer(VulkanContext& ctx, const uint16_t* indices, uint32_t count) {
    VkDeviceSize size = sizeof(uint16_t) * count;
    return CreateVertexBuffer(ctx, indices, size);
}

// Crear uniform buffer (mapeable)
BufferResource CreateUniformBuffer(VulkanContext& ctx, VkDeviceSize size) {
    BufferResource buffer = {};
    buffer.size = size;
    
    createBuffer(ctx, size, VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT,
        VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
        buffer.buffer, buffer.memory);
    
    buffer.Map(ctx.device);
    return buffer;
}

// ═══════════════════════════════════════════════════════════════════════════
// WRAPPER PARA DESCRIPTOR SETS
// ═══════════════════════════════════════════════════════════════════════════

struct DescriptorSetManager {
    VkDescriptorPool pool;
    VkDescriptorSetLayout layout;
    std::vector<VkDescriptorSet> sets;
    
    bool Initialize(VkDevice device, const VkDescriptorSetLayoutCreateInfo* layoutInfo,
                   uint32_t maxSets, const VkDescriptorPoolSize* poolSizes, uint32_t poolSizeCount) {
        // Crear layout
        if (vkCreateDescriptorSetLayout(device, layoutInfo, nullptr, &layout) != VK_SUCCESS) {
            return false;
        }
        
        // Crear pool
        VkDescriptorPoolCreateInfo poolInfo{};
        poolInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
        poolInfo.poolSizeCount = poolSizeCount;
        poolInfo.pPoolSizes = poolSizes;
        poolInfo.maxSets = maxSets;
        
        return vkCreateDescriptorPool(device, &poolInfo, nullptr, &pool) == VK_SUCCESS;
    }
    
    bool AllocateSets(VkDevice device, uint32_t count) {
        std::vector<VkDescriptorSetLayout> layouts(count, layout);
        VkDescriptorSetAllocateInfo allocInfo{};
        allocInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
        allocInfo.descriptorPool = pool;
        allocInfo.descriptorSetCount = count;
        allocInfo.pSetLayouts = layouts.data();
        
        sets.resize(count);
        return vkAllocateDescriptorSets(device, &allocInfo, sets.data()) == VK_SUCCESS;
    }
    
    void Cleanup(VkDevice device) {
        if (pool) {
            vkDestroyDescriptorPool(device, pool, nullptr);
            pool = VK_NULL_HANDLE;
        }
        if (layout) {
            vkDestroyDescriptorSetLayout(device, layout, nullptr);
            layout = VK_NULL_HANDLE;
        }
        sets.clear();
    }
};

