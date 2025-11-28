/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN SYNC - Sincronización (Semáforos y Fences)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: sync.cpp
 * Descripción: Crear objetos de sincronización para renderizado
 * Hereda: commands.cpp -> buffers.cpp -> pipeline.cpp -> vulkan_types.h
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR OBJETOS DE SINCRONIZACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createSyncObjects(VulkanContext& ctx) {
    ctx.imageAvailableSemaphores.resize(MAX_FRAMES_IN_FLIGHT);
    ctx.renderFinishedSemaphores.resize(MAX_FRAMES_IN_FLIGHT);
    ctx.inFlightFences.resize(MAX_FRAMES_IN_FLIGHT);

    VkSemaphoreCreateInfo semaphoreInfo{};
    semaphoreInfo.sType = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;

    VkFenceCreateInfo fenceInfo{};
    fenceInfo.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
    fenceInfo.flags = VK_FENCE_CREATE_SIGNALED_BIT;

    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT; i++) {
        if (vkCreateSemaphore(ctx.device, &semaphoreInfo, nullptr, 
            &ctx.imageAvailableSemaphores[i]) != VK_SUCCESS ||
            vkCreateSemaphore(ctx.device, &semaphoreInfo, nullptr, 
            &ctx.renderFinishedSemaphores[i]) != VK_SUCCESS ||
            vkCreateFence(ctx.device, &fenceInfo, nullptr, 
            &ctx.inFlightFences[i]) != VK_SUCCESS) {
            throw std::runtime_error("Error al crear objetos de sincronización!");
        }
    }

    std::cout << "[OK] Objetos de sincronización creados" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CLEANUP
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanupSync(VulkanContext& ctx) {
    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT; i++) {
        vkDestroySemaphore(ctx.device, ctx.renderFinishedSemaphores[i], nullptr);
        vkDestroySemaphore(ctx.device, ctx.imageAvailableSemaphores[i], nullptr);
        vkDestroyFence(ctx.device, ctx.inFlightFences[i], nullptr);
    }
    std::cout << "[OK] Objetos de sincronización destruidos" << std::endl;
}

