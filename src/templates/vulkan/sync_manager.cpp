/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN SYNC MANAGER - Gestión de sincronización (semáforos y fences)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: sync_manager.cpp
 * Descripción: Gestión de semáforos y fences para sincronización CPU-GPU
 * 
 * USO: Este nodo puede ser heredado para gestión de sincronización
 * Ejemplo: Conecta este nodo a render_loop.cpp para usar sync_manager con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <stdexcept>

// ═══════════════════════════════════════════════════════════════════════════
// GESTIÓN DE SEMÁFOROS Y FENCES
// ═══════════════════════════════════════════════════════════════════════════

struct SyncManager {
    std::vector<VkSemaphore> imageAvailableSemaphores;
    std::vector<VkSemaphore> renderFinishedSemaphores;
    std::vector<VkFence> inFlightFences;
    std::vector<VkFence> imagesInFlight;
    size_t currentFrame;
    
    bool Initialize(VkDevice device, uint32_t imageCount, uint32_t maxFramesInFlight) {
        imageAvailableSemaphores.resize(maxFramesInFlight);
        renderFinishedSemaphores.resize(maxFramesInFlight);
        inFlightFences.resize(maxFramesInFlight);
        imagesInFlight.resize(imageCount, VK_NULL_HANDLE);
        currentFrame = 0;
        
        VkSemaphoreCreateInfo semaphoreInfo{};
        semaphoreInfo.sType = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;
        
        VkFenceCreateInfo fenceInfo{};
        fenceInfo.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
        fenceInfo.flags = VK_FENCE_CREATE_SIGNALED_BIT; // Empezar señalizado
        
        for (size_t i = 0; i < maxFramesInFlight; i++) {
            if (vkCreateSemaphore(device, &semaphoreInfo, nullptr, &imageAvailableSemaphores[i]) != VK_SUCCESS ||
                vkCreateSemaphore(device, &semaphoreInfo, nullptr, &renderFinishedSemaphores[i]) != VK_SUCCESS ||
                vkCreateFence(device, &fenceInfo, nullptr, &inFlightFences[i]) != VK_SUCCESS) {
                return false;
            }
        }
        
        return true;
    }
    
    uint32_t GetCurrentFrame() const {
        return currentFrame;
    }
    
    VkSemaphore GetImageAvailableSemaphore() const {
        return imageAvailableSemaphores[currentFrame];
    }
    
    VkSemaphore GetRenderFinishedSemaphore() const {
        return renderFinishedSemaphores[currentFrame];
    }
    
    VkFence GetInFlightFence() const {
        return inFlightFences[currentFrame];
    }
    
    void WaitForFrame(VkDevice device) {
        vkWaitForFences(device, 1, &inFlightFences[currentFrame], VK_TRUE, UINT64_MAX);
    }
    
    void ResetFence(VkDevice device) {
        vkResetFences(device, 1, &inFlightFences[currentFrame]);
    }
    
    void AdvanceFrame(uint32_t maxFramesInFlight) {
        currentFrame = (currentFrame + 1) % maxFramesInFlight;
    }
    
    void SetImageInFlight(uint32_t imageIndex, VkFence fence) {
        imagesInFlight[imageIndex] = fence;
    }
    
    VkFence GetImageInFlight(uint32_t imageIndex) const {
        return imagesInFlight[imageIndex];
    }
    
    void Cleanup(VkDevice device) {
        for (size_t i = 0; i < imageAvailableSemaphores.size(); i++) {
            vkDestroySemaphore(device, imageAvailableSemaphores[i], nullptr);
            vkDestroySemaphore(device, renderFinishedSemaphores[i], nullptr);
            vkDestroyFence(device, inFlightFences[i], nullptr);
        }
        imageAvailableSemaphores.clear();
        renderFinishedSemaphores.clear();
        inFlightFences.clear();
        imagesInFlight.clear();
    }
};

