/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN RENDER LOOP - Loop principal de renderizado
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: render_loop.cpp
 * Descripción: Loop de renderizado frame por frame
 * Hereda: sync.cpp -> commands.cpp -> buffers.cpp -> pipeline.cpp
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>
#include <chrono>

/* Declaraciones externas */
extern void recordCommandBuffer(VulkanContext& ctx, VkCommandBuffer commandBuffer, 
    uint32_t imageIndex);
extern void updateUniformBuffer(VulkanContext& ctx, uint32_t currentImage, float time);
extern void recreateSwapChain(VulkanContext& ctx);

/* ═══════════════════════════════════════════════════════════════════════════
 * DIBUJAR FRAME
 * ═══════════════════════════════════════════════════════════════════════════
 */

void drawFrame(VulkanContext& ctx) {
    static auto startTime = std::chrono::high_resolution_clock::now();
    
    // Esperar a que el frame anterior termine
    vkWaitForFences(ctx.device, 1, &ctx.inFlightFences[ctx.currentFrame], 
        VK_TRUE, UINT64_MAX);

    // Adquirir imagen de la swapchain
    uint32_t imageIndex;
    VkResult result = vkAcquireNextImageKHR(ctx.device, ctx.swapChain, UINT64_MAX,
        ctx.imageAvailableSemaphores[ctx.currentFrame], VK_NULL_HANDLE, &imageIndex);

    if (result == VK_ERROR_OUT_OF_DATE_KHR) {
        recreateSwapChain(ctx);
        return;
    } else if (result != VK_SUCCESS && result != VK_SUBOPTIMAL_KHR) {
        throw std::runtime_error("Error al adquirir imagen de swapchain!");
    }

    // Calcular tiempo
    auto currentTime = std::chrono::high_resolution_clock::now();
    float time = std::chrono::duration<float, std::chrono::seconds::period>(
        currentTime - startTime).count();

    // Actualizar uniform buffer
    updateUniformBuffer(ctx, ctx.currentFrame, time);

    // Reset fence
    vkResetFences(ctx.device, 1, &ctx.inFlightFences[ctx.currentFrame]);

    // Reset y grabar command buffer
    vkResetCommandBuffer(ctx.commandBuffers[ctx.currentFrame], 0);
    recordCommandBuffer(ctx, ctx.commandBuffers[ctx.currentFrame], imageIndex);

    // Submit command buffer
    VkSubmitInfo submitInfo{};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;

    VkSemaphore waitSemaphores[] = {ctx.imageAvailableSemaphores[ctx.currentFrame]};
    VkPipelineStageFlags waitStages[] = {VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT};
    submitInfo.waitSemaphoreCount = 1;
    submitInfo.pWaitSemaphores = waitSemaphores;
    submitInfo.pWaitDstStageMask = waitStages;
    submitInfo.commandBufferCount = 1;
    submitInfo.pCommandBuffers = &ctx.commandBuffers[ctx.currentFrame];

    VkSemaphore signalSemaphores[] = {ctx.renderFinishedSemaphores[ctx.currentFrame]};
    submitInfo.signalSemaphoreCount = 1;
    submitInfo.pSignalSemaphores = signalSemaphores;

    if (vkQueueSubmit(ctx.graphicsQueue, 1, &submitInfo, 
        ctx.inFlightFences[ctx.currentFrame]) != VK_SUCCESS) {
        throw std::runtime_error("Error al enviar command buffer!");
    }

    // Presentar
    VkPresentInfoKHR presentInfo{};
    presentInfo.sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
    presentInfo.waitSemaphoreCount = 1;
    presentInfo.pWaitSemaphores = signalSemaphores;

    VkSwapchainKHR swapChains[] = {ctx.swapChain};
    presentInfo.swapchainCount = 1;
    presentInfo.pSwapchains = swapChains;
    presentInfo.pImageIndices = &imageIndex;

    result = vkQueuePresentKHR(ctx.presentQueue, &presentInfo);

    if (result == VK_ERROR_OUT_OF_DATE_KHR || result == VK_SUBOPTIMAL_KHR || 
        ctx.framebufferResized) {
        ctx.framebufferResized = false;
        recreateSwapChain(ctx);
    } else if (result != VK_SUCCESS) {
        throw std::runtime_error("Error al presentar imagen!");
    }

    // Siguiente frame
    ctx.currentFrame = (ctx.currentFrame + 1) % MAX_FRAMES_IN_FLIGHT;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * LOOP PRINCIPAL
 * ═══════════════════════════════════════════════════════════════════════════
 */

void mainLoop(VulkanContext& ctx) {
    std::cout << "\n[RUN] Iniciando render loop..." << std::endl;
    
    while (!glfwWindowShouldClose(ctx.window)) {
        glfwPollEvents();
        drawFrame(ctx);
    }

    vkDeviceWaitIdle(ctx.device);
    std::cout << "[END] Render loop finalizado" << std::endl;
}

