/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN DEVICE - Selección y creación de dispositivo
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: device.cpp
 * Descripción: Seleccionar GPU física y crear dispositivo lógico
 * Hereda: instance.cpp -> vulkan_types.h
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>
#include <set>
#include <string>

/* ═══════════════════════════════════════════════════════════════════════════
 * BUSCAR FAMILIAS DE COLAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

QueueFamilyIndices findQueueFamilies(VkPhysicalDevice device, VkSurfaceKHR surface) {
    QueueFamilyIndices indices;

    uint32_t queueFamilyCount = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, nullptr);

    std::vector<VkQueueFamilyProperties> queueFamilies(queueFamilyCount);
    vkGetPhysicalDeviceQueueFamilyProperties(device, &queueFamilyCount, queueFamilies.data());

    int i = 0;
    for (const auto& queueFamily : queueFamilies) {
        // Cola de gráficos
        if (queueFamily.queueFlags & VK_QUEUE_GRAPHICS_BIT) {
            indices.graphicsFamily = i;
        }

        // Cola de presentación
        VkBool32 presentSupport = false;
        vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface, &presentSupport);
        if (presentSupport) {
            indices.presentFamily = i;
        }

        if (indices.isComplete()) {
            break;
        }

        i++;
    }

    return indices;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * VERIFICAR EXTENSIONES DEL DISPOSITIVO
 * ═══════════════════════════════════════════════════════════════════════════
 */

bool checkDeviceExtensionSupport(VkPhysicalDevice device) {
    uint32_t extensionCount;
    vkEnumerateDeviceExtensionProperties(device, nullptr, &extensionCount, nullptr);

    std::vector<VkExtensionProperties> availableExtensions(extensionCount);
    vkEnumerateDeviceExtensionProperties(device, nullptr, &extensionCount, 
        availableExtensions.data());

    std::set<std::string> requiredExtensions(deviceExtensions.begin(), 
        deviceExtensions.end());

    for (const auto& extension : availableExtensions) {
        requiredExtensions.erase(extension.extensionName);
    }

    return requiredExtensions.empty();
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSULTAR SOPORTE DE SWAPCHAIN
 * ═══════════════════════════════════════════════════════════════════════════
 */

SwapChainSupportDetails querySwapChainSupport(VkPhysicalDevice device, 
    VkSurfaceKHR surface) {
    SwapChainSupportDetails details;

    // Capacidades de la superficie
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR(device, surface, &details.capabilities);

    // Formatos de superficie
    uint32_t formatCount;
    vkGetPhysicalDeviceSurfaceFormatsKHR(device, surface, &formatCount, nullptr);
    if (formatCount != 0) {
        details.formats.resize(formatCount);
        vkGetPhysicalDeviceSurfaceFormatsKHR(device, surface, &formatCount, 
            details.formats.data());
    }

    // Modos de presentación
    uint32_t presentModeCount;
    vkGetPhysicalDeviceSurfacePresentModesKHR(device, surface, &presentModeCount, nullptr);
    if (presentModeCount != 0) {
        details.presentModes.resize(presentModeCount);
        vkGetPhysicalDeviceSurfacePresentModesKHR(device, surface, &presentModeCount, 
            details.presentModes.data());
    }

    return details;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * EVALUAR DISPOSITIVO
 * ═══════════════════════════════════════════════════════════════════════════
 */

int rateDeviceSuitability(VkPhysicalDevice device, VkSurfaceKHR surface) {
    VkPhysicalDeviceProperties deviceProperties;
    VkPhysicalDeviceFeatures deviceFeatures;
    vkGetPhysicalDeviceProperties(device, &deviceProperties);
    vkGetPhysicalDeviceFeatures(device, &deviceFeatures);

    int score = 0;

    // GPU discreta tiene mayor preferencia
    if (deviceProperties.deviceType == VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU) {
        score += 1000;
    }

    // Máximo tamaño de texturas
    score += deviceProperties.limits.maxImageDimension2D;

    // Verificar características requeridas
    QueueFamilyIndices indices = findQueueFamilies(device, surface);
    if (!indices.isComplete()) {
        return 0;
    }

    // Verificar extensiones
    if (!checkDeviceExtensionSupport(device)) {
        return 0;
    }

    // Verificar swapchain
    SwapChainSupportDetails swapChainSupport = querySwapChainSupport(device, surface);
    if (swapChainSupport.formats.empty() || swapChainSupport.presentModes.empty()) {
        return 0;
    }

    // Verificar samplerAnisotropy
    if (!deviceFeatures.samplerAnisotropy) {
        return 0;
    }

    return score;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * SELECCIONAR GPU FÍSICA
 * ═══════════════════════════════════════════════════════════════════════════
 */

void pickPhysicalDevice(VulkanContext& ctx) {
    uint32_t deviceCount = 0;
    vkEnumeratePhysicalDevices(ctx.instance, &deviceCount, nullptr);

    if (deviceCount == 0) {
        throw std::runtime_error("No se encontraron GPUs con soporte Vulkan!");
    }

    std::vector<VkPhysicalDevice> devices(deviceCount);
    vkEnumeratePhysicalDevices(ctx.instance, &deviceCount, devices.data());

    // Evaluar y seleccionar el mejor dispositivo
    int bestScore = 0;
    for (const auto& device : devices) {
        int score = rateDeviceSuitability(device, ctx.surface);
        if (score > bestScore) {
            bestScore = score;
            ctx.physicalDevice = device;
        }
    }

    if (ctx.physicalDevice == VK_NULL_HANDLE) {
        throw std::runtime_error("No se encontró una GPU adecuada!");
    }

    // Mostrar información del dispositivo seleccionado
    VkPhysicalDeviceProperties deviceProperties;
    vkGetPhysicalDeviceProperties(ctx.physicalDevice, &deviceProperties);
    std::cout << "[OK] GPU seleccionada: " << deviceProperties.deviceName << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR DISPOSITIVO LÓGICO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createLogicalDevice(VulkanContext& ctx) {
    QueueFamilyIndices indices = findQueueFamilies(ctx.physicalDevice, ctx.surface);

    std::vector<VkDeviceQueueCreateInfo> queueCreateInfos;
    std::set<uint32_t> uniqueQueueFamilies = {
        indices.graphicsFamily.value(), 
        indices.presentFamily.value()
    };

    float queuePriority = 1.0f;
    for (uint32_t queueFamily : uniqueQueueFamilies) {
        VkDeviceQueueCreateInfo queueCreateInfo{};
        queueCreateInfo.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        queueCreateInfo.queueFamilyIndex = queueFamily;
        queueCreateInfo.queueCount = 1;
        queueCreateInfo.pQueuePriorities = &queuePriority;
        queueCreateInfos.push_back(queueCreateInfo);
    }

    // Características del dispositivo
    VkPhysicalDeviceFeatures deviceFeatures{};
    deviceFeatures.samplerAnisotropy = VK_TRUE;

    // Crear dispositivo lógico
    VkDeviceCreateInfo createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
    createInfo.queueCreateInfoCount = static_cast<uint32_t>(queueCreateInfos.size());
    createInfo.pQueueCreateInfos = queueCreateInfos.data();
    createInfo.pEnabledFeatures = &deviceFeatures;
    createInfo.enabledExtensionCount = static_cast<uint32_t>(deviceExtensions.size());
    createInfo.ppEnabledExtensionNames = deviceExtensions.data();

    if (enableValidationLayers) {
        createInfo.enabledLayerCount = static_cast<uint32_t>(validationLayers.size());
        createInfo.ppEnabledLayerNames = validationLayers.data();
    } else {
        createInfo.enabledLayerCount = 0;
    }

    if (vkCreateDevice(ctx.physicalDevice, &createInfo, nullptr, &ctx.device) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear dispositivo lógico!");
    }

    // Obtener colas
    vkGetDeviceQueue(ctx.device, indices.graphicsFamily.value(), 0, &ctx.graphicsQueue);
    vkGetDeviceQueue(ctx.device, indices.presentFamily.value(), 0, &ctx.presentQueue);

    std::cout << "[OK] Dispositivo lógico creado" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CLEANUP
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanupDevice(VulkanContext& ctx) {
    vkDestroyDevice(ctx.device, nullptr);
    std::cout << "[OK] Dispositivo lógico destruido" << std::endl;
}

