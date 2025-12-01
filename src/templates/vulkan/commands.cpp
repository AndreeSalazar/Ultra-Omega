/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN COMMANDS - Command Pool y Command Buffers
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: commands.cpp
 * Descripción: Crear y grabar command buffers
 * Hereda: buffers.cpp -> pipeline.cpp -> swapchain.cpp -> vulkan_types.h
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>

/* Declaración externa */
extern QueueFamilyIndices findQueueFamilies(VkPhysicalDevice device, VkSurfaceKHR surface);
extern const std::vector<uint16_t> indices;

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR COMMAND POOL
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createCommandPool(VulkanContext& ctx) {
    QueueFamilyIndices queueFamilyIndices = findQueueFamilies(ctx.physicalDevice, 
        ctx.surface);

    VkCommandPoolCreateInfo poolInfo{};
    poolInfo.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
    poolInfo.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
    poolInfo.queueFamilyIndex = queueFamilyIndices.graphicsFamily.value();

    if (vkCreateCommandPool(ctx.device, &poolInfo, nullptr, 
        &ctx.commandPool) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear command pool!");
    }

    std::cout << "[OK] Command pool creado" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR COMMAND BUFFERS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createCommandBuffers(VulkanContext& ctx) {
    ctx.commandBuffers.resize(MAX_FRAMES_IN_FLIGHT);

    VkCommandBufferAllocateInfo allocInfo{};
    allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    allocInfo.commandPool = ctx.commandPool;
    allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    allocInfo.commandBufferCount = static_cast<uint32_t>(ctx.commandBuffers.size());

    if (vkAllocateCommandBuffers(ctx.device, &allocInfo, 
        ctx.commandBuffers.data()) != VK_SUCCESS) {
        throw std::runtime_error("Error al asignar command buffers!");
    }

    std::cout << "[OK] Command buffers creados" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * GRABAR COMMAND BUFFER
 * ═══════════════════════════════════════════════════════════════════════════
 */

void recordCommandBuffer(VulkanContext& ctx, VkCommandBuffer commandBuffer, 
    uint32_t imageIndex) {
    
    VkCommandBufferBeginInfo beginInfo{};
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;

    if (vkBeginCommandBuffer(commandBuffer, &beginInfo) != VK_SUCCESS) {
        throw std::runtime_error("Error al comenzar grabación de command buffer!");
    }

    // Iniciar render pass
    VkRenderPassBeginInfo renderPassInfo{};
    renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
    renderPassInfo.renderPass = ctx.renderPass;
    renderPassInfo.framebuffer = ctx.swapChainFramebuffers[imageIndex];
    renderPassInfo.renderArea.offset = {0, 0};
    renderPassInfo.renderArea.extent = ctx.swapChainExtent;

    VkClearValue clearColor = {{{0.02f, 0.02f, 0.05f, 1.0f}}}; // Fondo oscuro
    renderPassInfo.clearValueCount = 1;
    renderPassInfo.pClearValues = &clearColor;

    vkCmdBeginRenderPass(commandBuffer, &renderPassInfo, VK_SUBPASS_CONTENTS_INLINE);

    // Bind pipeline
    vkCmdBindPipeline(commandBuffer, VK_PIPELINE_BIND_POINT_GRAPHICS, 
        ctx.graphicsPipeline);

    // Configurar viewport
    VkViewport viewport{};
    viewport.x = 0.0f;
    viewport.y = 0.0f;
    viewport.width = static_cast<float>(ctx.swapChainExtent.width);
    viewport.height = static_cast<float>(ctx.swapChainExtent.height);
    viewport.minDepth = 0.0f;
    viewport.maxDepth = 1.0f;
    vkCmdSetViewport(commandBuffer, 0, 1, &viewport);

    // Configurar scissor
    VkRect2D scissor{};
    scissor.offset = {0, 0};
    scissor.extent = ctx.swapChainExtent;
    vkCmdSetScissor(commandBuffer, 0, 1, &scissor);

    // Bind vertex buffer
    VkBuffer vertexBuffers[] = {ctx.vertexBuffer};
    VkDeviceSize offsets[] = {0};
    vkCmdBindVertexBuffers(commandBuffer, 0, 1, vertexBuffers, offsets);

    // Bind index buffer
    vkCmdBindIndexBuffer(commandBuffer, ctx.indexBuffer, 0, VK_INDEX_TYPE_UINT16);

    // Bind descriptor sets
    vkCmdBindDescriptorSets(commandBuffer, VK_PIPELINE_BIND_POINT_GRAPHICS,
        ctx.pipelineLayout, 0, 1, &ctx.descriptorSets[ctx.currentFrame], 
        0, nullptr);

    // Dibujar
    vkCmdDrawIndexed(commandBuffer, static_cast<uint32_t>(indices.size()), 
        1, 0, 0, 0);

    vkCmdEndRenderPass(commandBuffer);

    if (vkEndCommandBuffer(commandBuffer) != VK_SUCCESS) {
        throw std::runtime_error("Error al finalizar grabación de command buffer!");
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CLEANUP
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanupCommands(VulkanContext& ctx) {
    vkDestroyCommandPool(ctx.device, ctx.commandPool, nullptr);
    std::cout << "[OK] Command pool destruido" << std::endl;
}

