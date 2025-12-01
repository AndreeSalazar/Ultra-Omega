/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN SWAPCHAIN - Cadena de intercambio
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: swapchain.cpp
 * Descripción: Crear y manejar la swapchain para presentación
 * Hereda: device.cpp -> instance.cpp -> vulkan_types.h
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>
#include <algorithm>

/* Declaraciones externas */
extern SwapChainSupportDetails querySwapChainSupport(VkPhysicalDevice device, 
    VkSurfaceKHR surface);
extern QueueFamilyIndices findQueueFamilies(VkPhysicalDevice device, VkSurfaceKHR surface);

/* ═══════════════════════════════════════════════════════════════════════════
 * SELECCIÓN DE FORMATO DE SUPERFICIE
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkSurfaceFormatKHR chooseSwapSurfaceFormat(
    const std::vector<VkSurfaceFormatKHR>& availableFormats) {
    
    // Preferir SRGB con formato BGRA
    for (const auto& availableFormat : availableFormats) {
        if (availableFormat.format == VK_FORMAT_B8G8R8A8_SRGB &&
            availableFormat.colorSpace == VK_COLOR_SPACE_SRGB_NONLINEAR_KHR) {
            return availableFormat;
        }
    }

    return availableFormats[0];
}

/* ═══════════════════════════════════════════════════════════════════════════
 * SELECCIÓN DE MODO DE PRESENTACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkPresentModeKHR chooseSwapPresentMode(
    const std::vector<VkPresentModeKHR>& availablePresentModes) {
    
    // Preferir triple buffering (Mailbox)
    for (const auto& availablePresentMode : availablePresentModes) {
        if (availablePresentMode == VK_PRESENT_MODE_MAILBOX_KHR) {
            return availablePresentMode;
        }
    }

    // FIFO garantizado
    return VK_PRESENT_MODE_FIFO_KHR;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * SELECCIÓN DE EXTENSIÓN (RESOLUCIÓN)
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkExtent2D chooseSwapExtent(const VkSurfaceCapabilitiesKHR& capabilities, 
    GLFWwindow* window) {
    
    if (capabilities.currentExtent.width != std::numeric_limits<uint32_t>::max()) {
        return capabilities.currentExtent;
    } else {
        int width, height;
        glfwGetFramebufferSize(window, &width, &height);

        VkExtent2D actualExtent = {
            static_cast<uint32_t>(width),
            static_cast<uint32_t>(height)
        };

        actualExtent.width = std::clamp(actualExtent.width, 
            capabilities.minImageExtent.width, 
            capabilities.maxImageExtent.width);
        actualExtent.height = std::clamp(actualExtent.height, 
            capabilities.minImageExtent.height, 
            capabilities.maxImageExtent.height);

        return actualExtent;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR SWAPCHAIN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createSwapChain(VulkanContext& ctx) {
    SwapChainSupportDetails swapChainSupport = 
        querySwapChainSupport(ctx.physicalDevice, ctx.surface);

    VkSurfaceFormatKHR surfaceFormat = chooseSwapSurfaceFormat(swapChainSupport.formats);
    VkPresentModeKHR presentMode = chooseSwapPresentMode(swapChainSupport.presentModes);
    VkExtent2D extent = chooseSwapExtent(swapChainSupport.capabilities, ctx.window);

    // Número de imágenes (mínimo + 1 para triple buffering)
    uint32_t imageCount = swapChainSupport.capabilities.minImageCount + 1;
    if (swapChainSupport.capabilities.maxImageCount > 0 && 
        imageCount > swapChainSupport.capabilities.maxImageCount) {
        imageCount = swapChainSupport.capabilities.maxImageCount;
    }

    VkSwapchainCreateInfoKHR createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
    createInfo.surface = ctx.surface;
    createInfo.minImageCount = imageCount;
    createInfo.imageFormat = surfaceFormat.format;
    createInfo.imageColorSpace = surfaceFormat.colorSpace;
    createInfo.imageExtent = extent;
    createInfo.imageArrayLayers = 1;
    createInfo.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

    QueueFamilyIndices indices = findQueueFamilies(ctx.physicalDevice, ctx.surface);
    uint32_t queueFamilyIndices[] = {
        indices.graphicsFamily.value(), 
        indices.presentFamily.value()
    };

    if (indices.graphicsFamily != indices.presentFamily) {
        createInfo.imageSharingMode = VK_SHARING_MODE_CONCURRENT;
        createInfo.queueFamilyIndexCount = 2;
        createInfo.pQueueFamilyIndices = queueFamilyIndices;
    } else {
        createInfo.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;
    }

    createInfo.preTransform = swapChainSupport.capabilities.currentTransform;
    createInfo.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
    createInfo.presentMode = presentMode;
    createInfo.clipped = VK_TRUE;
    createInfo.oldSwapchain = VK_NULL_HANDLE;

    if (vkCreateSwapchainKHR(ctx.device, &createInfo, nullptr, &ctx.swapChain) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear swapchain!");
    }

    // Obtener imágenes de la swapchain
    vkGetSwapchainImagesKHR(ctx.device, ctx.swapChain, &imageCount, nullptr);
    ctx.swapChainImages.resize(imageCount);
    vkGetSwapchainImagesKHR(ctx.device, ctx.swapChain, &imageCount, 
        ctx.swapChainImages.data());

    ctx.swapChainImageFormat = surfaceFormat.format;
    ctx.swapChainExtent = extent;

    std::cout << "[OK] Swapchain creada (" << extent.width << "x" << extent.height 
              << ", " << imageCount << " imagenes)" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR IMAGE VIEWS
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkImageView createImageView(VulkanContext& ctx, VkImage image, VkFormat format, 
    VkImageAspectFlags aspectFlags) {
    
    VkImageViewCreateInfo viewInfo{};
    viewInfo.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
    viewInfo.image = image;
    viewInfo.viewType = VK_IMAGE_VIEW_TYPE_2D;
    viewInfo.format = format;
    viewInfo.subresourceRange.aspectMask = aspectFlags;
    viewInfo.subresourceRange.baseMipLevel = 0;
    viewInfo.subresourceRange.levelCount = 1;
    viewInfo.subresourceRange.baseArrayLayer = 0;
    viewInfo.subresourceRange.layerCount = 1;

    VkImageView imageView;
    if (vkCreateImageView(ctx.device, &viewInfo, nullptr, &imageView) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear image view!");
    }

    return imageView;
}

void createImageViews(VulkanContext& ctx) {
    ctx.swapChainImageViews.resize(ctx.swapChainImages.size());

    for (size_t i = 0; i < ctx.swapChainImages.size(); i++) {
        ctx.swapChainImageViews[i] = createImageView(ctx, ctx.swapChainImages[i], 
            ctx.swapChainImageFormat, VK_IMAGE_ASPECT_COLOR_BIT);
    }

    std::cout << "[OK] Image views creadas" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * RECREAR SWAPCHAIN (Para resize de ventana)
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanupSwapChain(VulkanContext& ctx);
void createFramebuffers(VulkanContext& ctx);

void recreateSwapChain(VulkanContext& ctx) {
    int width = 0, height = 0;
    glfwGetFramebufferSize(ctx.window, &width, &height);
    while (width == 0 || height == 0) {
        glfwGetFramebufferSize(ctx.window, &width, &height);
        glfwWaitEvents();
    }

    vkDeviceWaitIdle(ctx.device);

    cleanupSwapChain(ctx);

    createSwapChain(ctx);
    createImageViews(ctx);
    createFramebuffers(ctx);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CLEANUP
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanupSwapChain(VulkanContext& ctx) {
    // Destruir framebuffers
    for (auto framebuffer : ctx.swapChainFramebuffers) {
        vkDestroyFramebuffer(ctx.device, framebuffer, nullptr);
    }

    // Destruir image views
    for (auto imageView : ctx.swapChainImageViews) {
        vkDestroyImageView(ctx.device, imageView, nullptr);
    }

    // Destruir swapchain
    vkDestroySwapchainKHR(ctx.device, ctx.swapChain, nullptr);
}

