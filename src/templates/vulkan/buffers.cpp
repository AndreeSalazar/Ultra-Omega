/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN BUFFERS - Vertex, Index y Uniform Buffers
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: buffers.cpp
 * Descripción: Crear y manejar buffers de GPU
 * Hereda: pipeline.cpp -> swapchain.cpp -> device.cpp -> vulkan_types.h
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>
#include <cstring>

/* ═══════════════════════════════════════════════════════════════════════════
 * DATOS DE GEOMETRÍA (Cubo ejemplo)
 * ═══════════════════════════════════════════════════════════════════════════
 */

const std::vector<Vertex> vertices = {
    // Posición           Color               TexCoord
    {{-0.5f, -0.5f, 0.0f}, {1.0f, 0.0f, 0.0f}, {0.0f, 0.0f}},
    {{ 0.5f, -0.5f, 0.0f}, {0.0f, 1.0f, 0.0f}, {1.0f, 0.0f}},
    {{ 0.5f,  0.5f, 0.0f}, {0.0f, 0.0f, 1.0f}, {1.0f, 1.0f}},
    {{-0.5f,  0.5f, 0.0f}, {1.0f, 1.0f, 1.0f}, {0.0f, 1.0f}},
    
    {{-0.5f, -0.5f, -0.5f}, {1.0f, 0.0f, 0.0f}, {0.0f, 0.0f}},
    {{ 0.5f, -0.5f, -0.5f}, {0.0f, 1.0f, 0.0f}, {1.0f, 0.0f}},
    {{ 0.5f,  0.5f, -0.5f}, {0.0f, 0.0f, 1.0f}, {1.0f, 1.0f}},
    {{-0.5f,  0.5f, -0.5f}, {1.0f, 1.0f, 1.0f}, {0.0f, 1.0f}}
};

const std::vector<uint16_t> indices = {
    0, 1, 2, 2, 3, 0,
    4, 5, 6, 6, 7, 4
};

/* ═══════════════════════════════════════════════════════════════════════════
 * BUSCAR TIPO DE MEMORIA
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t findMemoryType(VulkanContext& ctx, uint32_t typeFilter, 
    VkMemoryPropertyFlags properties) {
    
    VkPhysicalDeviceMemoryProperties memProperties;
    vkGetPhysicalDeviceMemoryProperties(ctx.physicalDevice, &memProperties);

    for (uint32_t i = 0; i < memProperties.memoryTypeCount; i++) {
        if ((typeFilter & (1 << i)) && 
            (memProperties.memoryTypes[i].propertyFlags & properties) == properties) {
            return i;
        }
    }

    throw std::runtime_error("No se encontró tipo de memoria adecuado!");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR BUFFER GENÉRICO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createBuffer(VulkanContext& ctx, VkDeviceSize size, VkBufferUsageFlags usage,
    VkMemoryPropertyFlags properties, VkBuffer& buffer, VkDeviceMemory& bufferMemory) {
    
    VkBufferCreateInfo bufferInfo{};
    bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    bufferInfo.size = size;
    bufferInfo.usage = usage;
    bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;

    if (vkCreateBuffer(ctx.device, &bufferInfo, nullptr, &buffer) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear buffer!");
    }

    VkMemoryRequirements memRequirements;
    vkGetBufferMemoryRequirements(ctx.device, buffer, &memRequirements);

    VkMemoryAllocateInfo allocInfo{};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(ctx, memRequirements.memoryTypeBits, 
        properties);

    if (vkAllocateMemory(ctx.device, &allocInfo, nullptr, &bufferMemory) != VK_SUCCESS) {
        throw std::runtime_error("Error al asignar memoria de buffer!");
    }

    vkBindBufferMemory(ctx.device, buffer, bufferMemory, 0);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * COPIAR BUFFER
 * ═══════════════════════════════════════════════════════════════════════════
 */

void copyBuffer(VulkanContext& ctx, VkBuffer srcBuffer, VkBuffer dstBuffer, 
    VkDeviceSize size) {
    
    VkCommandBufferAllocateInfo allocInfo{};
    allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    allocInfo.commandPool = ctx.commandPool;
    allocInfo.commandBufferCount = 1;

    VkCommandBuffer commandBuffer;
    vkAllocateCommandBuffers(ctx.device, &allocInfo, &commandBuffer);

    VkCommandBufferBeginInfo beginInfo{};
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    beginInfo.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;

    vkBeginCommandBuffer(commandBuffer, &beginInfo);

    VkBufferCopy copyRegion{};
    copyRegion.size = size;
    vkCmdCopyBuffer(commandBuffer, srcBuffer, dstBuffer, 1, &copyRegion);

    vkEndCommandBuffer(commandBuffer);

    VkSubmitInfo submitInfo{};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
    submitInfo.commandBufferCount = 1;
    submitInfo.pCommandBuffers = &commandBuffer;

    vkQueueSubmit(ctx.graphicsQueue, 1, &submitInfo, VK_NULL_HANDLE);
    vkQueueWaitIdle(ctx.graphicsQueue);

    vkFreeCommandBuffers(ctx.device, ctx.commandPool, 1, &commandBuffer);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR VERTEX BUFFER
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createVertexBuffer(VulkanContext& ctx) {
    VkDeviceSize bufferSize = sizeof(vertices[0]) * vertices.size();

    // Staging buffer (CPU visible)
    VkBuffer stagingBuffer;
    VkDeviceMemory stagingBufferMemory;
    createBuffer(ctx, bufferSize, VK_BUFFER_USAGE_TRANSFER_SRC_BIT,
        VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
        stagingBuffer, stagingBufferMemory);

    // Copiar datos a staging
    void* data;
    vkMapMemory(ctx.device, stagingBufferMemory, 0, bufferSize, 0, &data);
    memcpy(data, vertices.data(), (size_t)bufferSize);
    vkUnmapMemory(ctx.device, stagingBufferMemory);

    // Crear buffer en GPU
    createBuffer(ctx, bufferSize,
        VK_BUFFER_USAGE_TRANSFER_DST_BIT | VK_BUFFER_USAGE_VERTEX_BUFFER_BIT,
        VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
        ctx.vertexBuffer, ctx.vertexBufferMemory);

    // Copiar de staging a GPU
    copyBuffer(ctx, stagingBuffer, ctx.vertexBuffer, bufferSize);

    // Cleanup staging
    vkDestroyBuffer(ctx.device, stagingBuffer, nullptr);
    vkFreeMemory(ctx.device, stagingBufferMemory, nullptr);

    std::cout << "[OK] Vertex buffer creado" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR INDEX BUFFER
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createIndexBuffer(VulkanContext& ctx) {
    VkDeviceSize bufferSize = sizeof(indices[0]) * indices.size();

    VkBuffer stagingBuffer;
    VkDeviceMemory stagingBufferMemory;
    createBuffer(ctx, bufferSize, VK_BUFFER_USAGE_TRANSFER_SRC_BIT,
        VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
        stagingBuffer, stagingBufferMemory);

    void* data;
    vkMapMemory(ctx.device, stagingBufferMemory, 0, bufferSize, 0, &data);
    memcpy(data, indices.data(), (size_t)bufferSize);
    vkUnmapMemory(ctx.device, stagingBufferMemory);

    createBuffer(ctx, bufferSize,
        VK_BUFFER_USAGE_TRANSFER_DST_BIT | VK_BUFFER_USAGE_INDEX_BUFFER_BIT,
        VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
        ctx.indexBuffer, ctx.indexBufferMemory);

    copyBuffer(ctx, stagingBuffer, ctx.indexBuffer, bufferSize);

    vkDestroyBuffer(ctx.device, stagingBuffer, nullptr);
    vkFreeMemory(ctx.device, stagingBufferMemory, nullptr);

    std::cout << "[OK] Index buffer creado" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR UNIFORM BUFFERS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createUniformBuffers(VulkanContext& ctx) {
    VkDeviceSize bufferSize = sizeof(UniformBufferObject);

    ctx.uniformBuffers.resize(MAX_FRAMES_IN_FLIGHT);
    ctx.uniformBuffersMemory.resize(MAX_FRAMES_IN_FLIGHT);
    ctx.uniformBuffersMapped.resize(MAX_FRAMES_IN_FLIGHT);

    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT; i++) {
        createBuffer(ctx, bufferSize, VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT,
            VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            ctx.uniformBuffers[i], ctx.uniformBuffersMemory[i]);

        vkMapMemory(ctx.device, ctx.uniformBuffersMemory[i], 0, bufferSize, 0, 
            &ctx.uniformBuffersMapped[i]);
    }

    std::cout << "[OK] Uniform buffers creados" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * ACTUALIZAR UNIFORM BUFFER (Llamar cada frame)
 * ═══════════════════════════════════════════════════════════════════════════
 */

void updateUniformBuffer(VulkanContext& ctx, uint32_t currentImage, float time) {
    UniformBufferObject ubo{};
    
    // Matriz de modelo (rotación)
    float angle = time * 1.5708f; // 90 grados por segundo
    float c = cosf(angle);
    float s = sinf(angle);
    
    // Rotación en Y
    ubo.model[0] = c;    ubo.model[1] = 0.0f; ubo.model[2] = s;    ubo.model[3] = 0.0f;
    ubo.model[4] = 0.0f; ubo.model[5] = 1.0f; ubo.model[6] = 0.0f; ubo.model[7] = 0.0f;
    ubo.model[8] = -s;   ubo.model[9] = 0.0f; ubo.model[10] = c;   ubo.model[11] = 0.0f;
    ubo.model[12] = 0.0f;ubo.model[13] = 0.0f;ubo.model[14] = 0.0f;ubo.model[15] = 1.0f;
    
    // Matriz de vista (lookAt simplificado)
    ubo.view[0] = 1.0f;  ubo.view[1] = 0.0f;  ubo.view[2] = 0.0f;  ubo.view[3] = 0.0f;
    ubo.view[4] = 0.0f;  ubo.view[5] = 1.0f;  ubo.view[6] = 0.0f;  ubo.view[7] = 0.0f;
    ubo.view[8] = 0.0f;  ubo.view[9] = 0.0f;  ubo.view[10] = 1.0f; ubo.view[11] = 0.0f;
    ubo.view[12] = 0.0f; ubo.view[13] = 0.0f; ubo.view[14] = -2.0f;ubo.view[15] = 1.0f;
    
    // Matriz de proyección (perspectiva)
    float aspect = (float)ctx.swapChainExtent.width / (float)ctx.swapChainExtent.height;
    float fov = 0.785398f; // 45 grados
    float near_plane = 0.1f;
    float far_plane = 10.0f;
    float tanHalfFov = tanf(fov / 2.0f);
    
    ubo.proj[0] = 1.0f / (aspect * tanHalfFov);
    ubo.proj[5] = -1.0f / tanHalfFov; // Invertido para Vulkan
    ubo.proj[10] = far_plane / (near_plane - far_plane);
    ubo.proj[11] = -1.0f;
    ubo.proj[14] = -(far_plane * near_plane) / (far_plane - near_plane);
    
    memcpy(ctx.uniformBuffersMapped[currentImage], &ubo, sizeof(ubo));
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CLEANUP
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanupBuffers(VulkanContext& ctx) {
    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT; i++) {
        vkDestroyBuffer(ctx.device, ctx.uniformBuffers[i], nullptr);
        vkFreeMemory(ctx.device, ctx.uniformBuffersMemory[i], nullptr);
    }
    
    vkDestroyBuffer(ctx.device, ctx.indexBuffer, nullptr);
    vkFreeMemory(ctx.device, ctx.indexBufferMemory, nullptr);
    
    vkDestroyBuffer(ctx.device, ctx.vertexBuffer, nullptr);
    vkFreeMemory(ctx.device, ctx.vertexBufferMemory, nullptr);
    
    std::cout << "[OK] Buffers destruidos" << std::endl;
}

