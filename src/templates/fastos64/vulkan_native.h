/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - VULKAN NATIVE API HEADER
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: vulkan_native.h
 * Descripción: API Vulkan nativa para FastOS con soporte RTX 3060
 * Autor: Eddi Andreé Salazar Matos
 * 
 * Esta es una implementación simplificada de Vulkan que funciona directamente
 * con el hardware cuando se usa GPU passthrough (VFIO).
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef VULKAN_NATIVE_H
#define VULKAN_NATIVE_H

#include "types64.h"
#include "pci.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN TYPES
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef uint32_t VkFlags;
typedef uint32_t VkBool32;
typedef uint64_t VkDeviceSize;

#define VK_TRUE     1
#define VK_FALSE    0
#define VK_NULL_HANDLE  0

typedef enum VkResult {
    VK_SUCCESS = 0,
    VK_NOT_READY = 1,
    VK_TIMEOUT = 2,
    VK_ERROR_OUT_OF_HOST_MEMORY = -1,
    VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
    VK_ERROR_INITIALIZATION_FAILED = -3,
    VK_ERROR_DEVICE_LOST = -4,
    VK_ERROR_EXTENSION_NOT_PRESENT = -7,
    VK_ERROR_FEATURE_NOT_PRESENT = -8,
} VkResult;

typedef enum VkStructureType {
    VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,
    VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,
    VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,
    VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
} VkStructureType;

typedef enum VkFormat {
    VK_FORMAT_UNDEFINED = 0,
    VK_FORMAT_R8G8B8A8_UNORM = 37,
    VK_FORMAT_B8G8R8A8_UNORM = 44,
    VK_FORMAT_B8G8R8A8_SRGB = 50,
} VkFormat;

typedef enum VkColorSpaceKHR {
    VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,
} VkColorSpaceKHR;

typedef enum VkPresentModeKHR {
    VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
    VK_PRESENT_MODE_MAILBOX_KHR = 1,
    VK_PRESENT_MODE_FIFO_KHR = 2,
    VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
} VkPresentModeKHR;

/* ═══════════════════════════════════════════════════════════════════════════
 * HANDLES
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct VkInstance_T* VkInstance;
typedef struct VkPhysicalDevice_T* VkPhysicalDevice;
typedef struct VkDevice_T* VkDevice;
typedef struct VkQueue_T* VkQueue;
typedef struct VkCommandPool_T* VkCommandPool;
typedef struct VkCommandBuffer_T* VkCommandBuffer;
typedef struct VkSemaphore_T* VkSemaphore;
typedef struct VkFence_T* VkFence;
typedef struct VkSwapchainKHR_T* VkSwapchainKHR;
typedef struct VkSurfaceKHR_T* VkSurfaceKHR;
typedef struct VkImage_T* VkImage;
typedef struct VkImageView_T* VkImageView;
typedef struct VkFramebuffer_T* VkFramebuffer;
typedef struct VkRenderPass_T* VkRenderPass;
typedef struct VkPipeline_T* VkPipeline;
typedef struct VkPipelineLayout_T* VkPipelineLayout;
typedef struct VkShaderModule_T* VkShaderModule;
typedef struct VkBuffer_T* VkBuffer;
typedef struct VkDeviceMemory_T* VkDeviceMemory;

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct VkExtent2D {
    uint32_t width;
    uint32_t height;
} VkExtent2D;

typedef struct VkExtent3D {
    uint32_t width;
    uint32_t height;
    uint32_t depth;
} VkExtent3D;

typedef struct VkOffset2D {
    int32_t x;
    int32_t y;
} VkOffset2D;

typedef struct VkRect2D {
    VkOffset2D offset;
    VkExtent2D extent;
} VkRect2D;

typedef struct VkViewport {
    float x;
    float y;
    float width;
    float height;
    float minDepth;
    float maxDepth;
} VkViewport;

typedef struct VkApplicationInfo {
    VkStructureType sType;
    const void* pNext;
    const char* pApplicationName;
    uint32_t applicationVersion;
    const char* pEngineName;
    uint32_t engineVersion;
    uint32_t apiVersion;
} VkApplicationInfo;

typedef struct VkInstanceCreateInfo {
    VkStructureType sType;
    const void* pNext;
    VkFlags flags;
    const VkApplicationInfo* pApplicationInfo;
    uint32_t enabledLayerCount;
    const char* const* ppEnabledLayerNames;
    uint32_t enabledExtensionCount;
    const char* const* ppEnabledExtensionNames;
} VkInstanceCreateInfo;

typedef struct VkPhysicalDeviceProperties {
    uint32_t apiVersion;
    uint32_t driverVersion;
    uint32_t vendorID;
    uint32_t deviceID;
    uint32_t deviceType;
    char deviceName[256];
    uint8_t pipelineCacheUUID[16];
    /* ... más campos ... */
} VkPhysicalDeviceProperties;

typedef struct VkPhysicalDeviceMemoryProperties {
    uint32_t memoryTypeCount;
    struct {
        uint32_t propertyFlags;
        uint32_t heapIndex;
    } memoryTypes[32];
    uint32_t memoryHeapCount;
    struct {
        VkDeviceSize size;
        VkFlags flags;
    } memoryHeaps[16];
} VkPhysicalDeviceMemoryProperties;

/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS VULKAN CONTEXT
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct FastOSVulkanContext {
    /* Instance */
    VkInstance instance;
    VkPhysicalDevice physicalDevice;
    VkDevice device;
    VkQueue graphicsQueue;
    VkQueue presentQueue;
    
    /* Swapchain */
    VkSwapchainKHR swapchain;
    VkFormat swapchainFormat;
    VkExtent2D swapchainExtent;
    uint32_t imageCount;
    VkImage* swapchainImages;
    VkImageView* swapchainImageViews;
    
    /* Render */
    VkRenderPass renderPass;
    VkFramebuffer* framebuffers;
    VkCommandPool commandPool;
    VkCommandBuffer* commandBuffers;
    
    /* Sync */
    VkSemaphore imageAvailableSemaphore;
    VkSemaphore renderFinishedSemaphore;
    VkFence inFlightFence;
    
    /* GPU Info */
    PCIDevice* gpu;
    uint64_t vram_base;
    uint64_t vram_size;
    uint64_t mmio_base;
    
    /* Estado */
    bool initialized;
    uint32_t currentFrame;
} FastOSVulkanContext;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES VULKAN NATIVAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
VkResult vkCreateInstance(
    const VkInstanceCreateInfo* pCreateInfo,
    void* pAllocator,
    VkInstance* pInstance
);

void vkDestroyInstance(
    VkInstance instance,
    void* pAllocator
);

/* Dispositivos físicos */
VkResult vkEnumeratePhysicalDevices(
    VkInstance instance,
    uint32_t* pPhysicalDeviceCount,
    VkPhysicalDevice* pPhysicalDevices
);

void vkGetPhysicalDeviceProperties(
    VkPhysicalDevice physicalDevice,
    VkPhysicalDeviceProperties* pProperties
);

void vkGetPhysicalDeviceMemoryProperties(
    VkPhysicalDevice physicalDevice,
    VkPhysicalDeviceMemoryProperties* pMemoryProperties
);

/* Dispositivo lógico */
VkResult vkCreateDevice(
    VkPhysicalDevice physicalDevice,
    void* pCreateInfo,
    void* pAllocator,
    VkDevice* pDevice
);

void vkDestroyDevice(
    VkDevice device,
    void* pAllocator
);

void vkGetDeviceQueue(
    VkDevice device,
    uint32_t queueFamilyIndex,
    uint32_t queueIndex,
    VkQueue* pQueue
);

/* Swapchain */
VkResult vkCreateSwapchainKHR(
    VkDevice device,
    void* pCreateInfo,
    void* pAllocator,
    VkSwapchainKHR* pSwapchain
);

void vkDestroySwapchainKHR(
    VkDevice device,
    VkSwapchainKHR swapchain,
    void* pAllocator
);

VkResult vkGetSwapchainImagesKHR(
    VkDevice device,
    VkSwapchainKHR swapchain,
    uint32_t* pSwapchainImageCount,
    VkImage* pSwapchainImages
);

VkResult vkAcquireNextImageKHR(
    VkDevice device,
    VkSwapchainKHR swapchain,
    uint64_t timeout,
    VkSemaphore semaphore,
    VkFence fence,
    uint32_t* pImageIndex
);

VkResult vkQueuePresentKHR(
    VkQueue queue,
    void* pPresentInfo
);

/* Command buffers */
VkResult vkCreateCommandPool(
    VkDevice device,
    void* pCreateInfo,
    void* pAllocator,
    VkCommandPool* pCommandPool
);

void vkDestroyCommandPool(
    VkDevice device,
    VkCommandPool commandPool,
    void* pAllocator
);

VkResult vkAllocateCommandBuffers(
    VkDevice device,
    void* pAllocateInfo,
    VkCommandBuffer* pCommandBuffers
);

VkResult vkBeginCommandBuffer(
    VkCommandBuffer commandBuffer,
    void* pBeginInfo
);

VkResult vkEndCommandBuffer(
    VkCommandBuffer commandBuffer
);

VkResult vkQueueSubmit(
    VkQueue queue,
    uint32_t submitCount,
    void* pSubmits,
    VkFence fence
);

/* Render pass */
void vkCmdBeginRenderPass(
    VkCommandBuffer commandBuffer,
    void* pRenderPassBegin,
    uint32_t contents
);

void vkCmdEndRenderPass(
    VkCommandBuffer commandBuffer
);

void vkCmdBindPipeline(
    VkCommandBuffer commandBuffer,
    uint32_t pipelineBindPoint,
    VkPipeline pipeline
);

void vkCmdDraw(
    VkCommandBuffer commandBuffer,
    uint32_t vertexCount,
    uint32_t instanceCount,
    uint32_t firstVertex,
    uint32_t firstInstance
);

void vkCmdSetViewport(
    VkCommandBuffer commandBuffer,
    uint32_t firstViewport,
    uint32_t viewportCount,
    const VkViewport* pViewports
);

void vkCmdSetScissor(
    VkCommandBuffer commandBuffer,
    uint32_t firstScissor,
    uint32_t scissorCount,
    const VkRect2D* pScissors
);

/* Sincronización */
VkResult vkCreateSemaphore(
    VkDevice device,
    void* pCreateInfo,
    void* pAllocator,
    VkSemaphore* pSemaphore
);

void vkDestroySemaphore(
    VkDevice device,
    VkSemaphore semaphore,
    void* pAllocator
);

VkResult vkCreateFence(
    VkDevice device,
    void* pCreateInfo,
    void* pAllocator,
    VkFence* pFence
);

void vkDestroyFence(
    VkDevice device,
    VkFence fence,
    void* pAllocator
);

VkResult vkWaitForFences(
    VkDevice device,
    uint32_t fenceCount,
    const VkFence* pFences,
    VkBool32 waitAll,
    uint64_t timeout
);

VkResult vkResetFences(
    VkDevice device,
    uint32_t fenceCount,
    const VkFence* pFences
);

/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS VULKAN HELPERS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicializar contexto Vulkan completo */
VkResult fastos_vulkan_init(FastOSVulkanContext* ctx, uint32_t width, uint32_t height);

/* Limpiar contexto */
void fastos_vulkan_cleanup(FastOSVulkanContext* ctx);

/* Renderizar frame */
VkResult fastos_vulkan_render_frame(FastOSVulkanContext* ctx);

/* Obtener información de GPU */
void fastos_vulkan_get_gpu_info(FastOSVulkanContext* ctx, char* buffer, uint32_t size);

/* Verificar soporte RTX */
bool fastos_vulkan_has_rtx_support(FastOSVulkanContext* ctx);

/* Verificar soporte Ray Tracing */
bool fastos_vulkan_has_raytracing(FastOSVulkanContext* ctx);

#endif /* VULKAN_NATIVE_H */

