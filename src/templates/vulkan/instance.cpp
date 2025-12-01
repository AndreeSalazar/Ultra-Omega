/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN INSTANCE - Creación de instancia Vulkan
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: instance.cpp
 * Descripción: Funciones para crear y destruir la instancia de Vulkan
 * Hereda: vulkan_types.h
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>
#include <cstring>

/* ═══════════════════════════════════════════════════════════════════════════
 * CALLBACK DE DEBUG
 * ═══════════════════════════════════════════════════════════════════════════
 */

static VKAPI_ATTR VkBool32 VKAPI_CALL debugCallback(
    VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity,
    VkDebugUtilsMessageTypeFlagsEXT messageType,
    const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData,
    void* pUserData) {
    
    if (messageSeverity >= VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT) {
        std::cerr << "[VULKAN] " << pCallbackData->pMessage << std::endl;
    }
    
    return VK_FALSE;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * VERIFICAR CAPAS DE VALIDACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

bool checkValidationLayerSupport() {
    uint32_t layerCount;
    vkEnumerateInstanceLayerProperties(&layerCount, nullptr);

    std::vector<VkLayerProperties> availableLayers(layerCount);
    vkEnumerateInstanceLayerProperties(&layerCount, availableLayers.data());

    for (const char* layerName : validationLayers) {
        bool layerFound = false;

        for (const auto& layerProperties : availableLayers) {
            if (strcmp(layerName, layerProperties.layerName) == 0) {
                layerFound = true;
                break;
            }
        }

        if (!layerFound) {
            return false;
        }
    }

    return true;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * OBTENER EXTENSIONES REQUERIDAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

std::vector<const char*> getRequiredExtensions() {
    uint32_t glfwExtensionCount = 0;
    const char** glfwExtensions;
    glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);

    std::vector<const char*> extensions(glfwExtensions, glfwExtensions + glfwExtensionCount);

    if (enableValidationLayers) {
        extensions.push_back(VK_EXT_DEBUG_UTILS_EXTENSION_NAME);
    }

    return extensions;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR DEBUG MESSENGER
 * ═══════════════════════════════════════════════════════════════════════════
 */

void populateDebugMessengerCreateInfo(VkDebugUtilsMessengerCreateInfoEXT& createInfo) {
    createInfo = {};
    createInfo.sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
    createInfo.messageSeverity = VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT |
                                  VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT |
                                  VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
    createInfo.messageType = VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT |
                              VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT |
                              VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
    createInfo.pfnUserCallback = debugCallback;
}

VkResult CreateDebugUtilsMessengerEXT(VkInstance instance,
    const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo,
    const VkAllocationCallbacks* pAllocator,
    VkDebugUtilsMessengerEXT* pDebugMessenger) {
    
    auto func = (PFN_vkCreateDebugUtilsMessengerEXT)
        vkGetInstanceProcAddr(instance, "vkCreateDebugUtilsMessengerEXT");
    
    if (func != nullptr) {
        return func(instance, pCreateInfo, pAllocator, pDebugMessenger);
    } else {
        return VK_ERROR_EXTENSION_NOT_PRESENT;
    }
}

void DestroyDebugUtilsMessengerEXT(VkInstance instance,
    VkDebugUtilsMessengerEXT debugMessenger,
    const VkAllocationCallbacks* pAllocator) {
    
    auto func = (PFN_vkDestroyDebugUtilsMessengerEXT)
        vkGetInstanceProcAddr(instance, "vkDestroyDebugUtilsMessengerEXT");
    
    if (func != nullptr) {
        func(instance, debugMessenger, pAllocator);
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR INSTANCIA VULKAN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createInstance(VulkanContext& ctx) {
    if (enableValidationLayers && !checkValidationLayerSupport()) {
        throw std::runtime_error("Capas de validación solicitadas no disponibles!");
    }

    // Información de la aplicación
    VkApplicationInfo appInfo{};
    appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appInfo.pApplicationName = "Vulkan App";
    appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.pEngineName = "Ultra-Omega Engine";
    appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.apiVersion = VK_API_VERSION_1_3;

    // Información de creación de instancia
    VkInstanceCreateInfo createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pApplicationInfo = &appInfo;

    // Extensiones
    auto extensions = getRequiredExtensions();
    createInfo.enabledExtensionCount = static_cast<uint32_t>(extensions.size());
    createInfo.ppEnabledExtensionNames = extensions.data();

    // Debug messenger para creación/destrucción de instancia
    VkDebugUtilsMessengerCreateInfoEXT debugCreateInfo{};
    if (enableValidationLayers) {
        createInfo.enabledLayerCount = static_cast<uint32_t>(validationLayers.size());
        createInfo.ppEnabledLayerNames = validationLayers.data();

        populateDebugMessengerCreateInfo(debugCreateInfo);
        createInfo.pNext = (VkDebugUtilsMessengerCreateInfoEXT*)&debugCreateInfo;
    } else {
        createInfo.enabledLayerCount = 0;
        createInfo.pNext = nullptr;
    }

    // Crear instancia
    if (vkCreateInstance(&createInfo, nullptr, &ctx.instance) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear instancia Vulkan!");
    }

    std::cout << "[OK] Instancia Vulkan creada" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CONFIGURAR DEBUG MESSENGER
 * ═══════════════════════════════════════════════════════════════════════════
 */

void setupDebugMessenger(VulkanContext& ctx) {
    if (!enableValidationLayers) return;

    VkDebugUtilsMessengerCreateInfoEXT createInfo;
    populateDebugMessengerCreateInfo(createInfo);

    if (CreateDebugUtilsMessengerEXT(ctx.instance, &createInfo, nullptr, 
        &ctx.debugMessenger) != VK_SUCCESS) {
        throw std::runtime_error("Error al configurar debug messenger!");
    }

    std::cout << "[OK] Debug messenger configurado" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CLEANUP
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanupInstance(VulkanContext& ctx) {
    if (enableValidationLayers) {
        DestroyDebugUtilsMessengerEXT(ctx.instance, ctx.debugMessenger, nullptr);
    }
    
    vkDestroyInstance(ctx.instance, nullptr);
    std::cout << "[OK] Instancia Vulkan destruida" << std::endl;
}

