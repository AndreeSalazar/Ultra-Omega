/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT + VULKAN - VULKAN ABSTRACTION LAYER
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: vulkan_fastos.h
 * Descripción: Capa de abstracción Vulkan-like para FastOS
 * 
 * NOTA: Esta es una implementación simplificada de conceptos Vulkan
 *       para FastOS. No es Vulkan completo, pero usa la misma filosofía.
 * 
 * Hardware objetivo: NVIDIA RTX 3060 12GB
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef VULKAN_FASTOS_H
#define VULKAN_FASTOS_H

#include "types64.h"
#include "gpu_nvidia.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS VULKAN-LIKE
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef uint32_t VkResult;
typedef uint64_t VkDeviceSize;
typedef uint32_t VkFlags;
typedef uint32_t VkBool32;

#define VK_TRUE                     1
#define VK_FALSE                    0

/* Resultados */
#define VK_SUCCESS                  0
#define VK_NOT_READY                1
#define VK_TIMEOUT                  2
#define VK_ERROR_OUT_OF_HOST_MEMORY -1
#define VK_ERROR_OUT_OF_DEVICE_MEMORY -2
#define VK_ERROR_INITIALIZATION_FAILED -3
#define VK_ERROR_DEVICE_LOST        -4
#define VK_ERROR_FEATURE_NOT_PRESENT -8

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Información de la aplicación */
typedef struct {
    const char*     app_name;
    uint32_t        app_version;
    const char*     engine_name;
    uint32_t        engine_version;
    uint32_t        api_version;
} VkApplicationInfo;

/* Propiedades físicas del dispositivo */
typedef struct {
    uint32_t        api_version;
    uint32_t        driver_version;
    uint32_t        vendor_id;
    uint32_t        device_id;
    uint32_t        device_type;
    char            device_name[256];
    
    /* Límites */
    uint32_t        max_image_dimension_2d;
    uint32_t        max_uniform_buffer_range;
    uint32_t        max_storage_buffer_range;
    uint32_t        max_push_constants_size;
    uint32_t        max_memory_allocation_count;
    uint32_t        max_bound_descriptor_sets;
    uint32_t        max_compute_work_group_count[3];
    uint32_t        max_compute_work_group_size[3];
    
    /* RTX 3060 específico */
    uint32_t        cuda_cores;
    uint32_t        sm_count;
    uint32_t        vram_size_mb;
    uint32_t        memory_bus_width;
    
} VkPhysicalDeviceProperties;

/* Características del dispositivo */
typedef struct {
    VkBool32        geometry_shader;
    VkBool32        tessellation_shader;
    VkBool32        multi_viewport;
    VkBool32        sampler_anisotropy;
    VkBool32        texture_compression_bc;
    VkBool32        shader_float64;
    VkBool32        shader_int64;
    
    /* RTX específico */
    VkBool32        ray_tracing;
    VkBool32        mesh_shaders;
    VkBool32        variable_rate_shading;
    
} VkPhysicalDeviceFeatures;

/* Propiedades de memoria */
typedef struct {
    uint32_t        memory_type_count;
    struct {
        uint32_t    property_flags;
        uint32_t    heap_index;
    } memory_types[32];
    
    uint32_t        memory_heap_count;
    struct {
        VkDeviceSize    size;
        uint32_t        flags;
    } memory_heaps[16];
    
} VkPhysicalDeviceMemoryProperties;

/* Handles */
typedef struct VkInstance_T*        VkInstance;
typedef struct VkPhysicalDevice_T*  VkPhysicalDevice;
typedef struct VkDevice_T*          VkDevice;
typedef struct VkQueue_T*           VkQueue;
typedef struct VkCommandPool_T*     VkCommandPool;
typedef struct VkCommandBuffer_T*   VkCommandBuffer;
typedef struct VkBuffer_T*          VkBuffer;
typedef struct VkDeviceMemory_T*    VkDeviceMemory;
typedef struct VkImage_T*           VkImage;
typedef struct VkImageView_T*       VkImageView;
typedef struct VkSampler_T*         VkSampler;
typedef struct VkFramebuffer_T*     VkFramebuffer;
typedef struct VkRenderPass_T*      VkRenderPass;
typedef struct VkPipeline_T*        VkPipeline;

/* ═══════════════════════════════════════════════════════════════════════════
 * CONTEXTO FASTOS-VULKAN
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    /* Estado */
    bool                            initialized;
    
    /* GPU */
    NvidiaGPU*                      gpu;
    VkPhysicalDeviceProperties      properties;
    VkPhysicalDeviceFeatures        features;
    VkPhysicalDeviceMemoryProperties memory_props;
    
    /* Framebuffer */
    uint32_t*                       framebuffer;
    uint32_t                        fb_width;
    uint32_t                        fb_height;
    uint32_t                        fb_pitch;
    
    /* Estadísticas */
    uint64_t                        frames_rendered;
    uint64_t                        draw_calls;
    uint64_t                        triangles_drawn;
    
} VulkanFastOSContext;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE INSTANCIA
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkResult vkf_create_instance(const VkApplicationInfo* app_info);
void vkf_destroy_instance(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE DISPOSITIVO FÍSICO
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkResult vkf_enumerate_physical_devices(uint32_t* count);
VkResult vkf_get_physical_device_properties(VkPhysicalDeviceProperties* props);
VkResult vkf_get_physical_device_features(VkPhysicalDeviceFeatures* features);
VkResult vkf_get_physical_device_memory_properties(VkPhysicalDeviceMemoryProperties* props);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE DISPOSITIVO LÓGICO
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkResult vkf_create_device(void);
void vkf_destroy_device(void);
void vkf_device_wait_idle(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE RENDERIZADO
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkResult vkf_begin_frame(void);
VkResult vkf_end_frame(void);
void vkf_clear_color(float r, float g, float b, float a);

/* Primitivas */
void vkf_draw_pixel(int32_t x, int32_t y, uint32_t color);
void vkf_draw_line(int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint32_t color);
void vkf_draw_rect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color);
void vkf_fill_rect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color);
void vkf_draw_triangle(int32_t x0, int32_t y0, int32_t x1, int32_t y1, 
                       int32_t x2, int32_t y2, uint32_t color);
void vkf_fill_triangle(int32_t x0, int32_t y0, int32_t x1, int32_t y1,
                       int32_t x2, int32_t y2, uint32_t color);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE INFORMACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

VulkanFastOSContext* vkf_get_context(void);
void vkf_print_info(void);
void vkf_print_stats(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t vkf_rgb(uint8_t r, uint8_t g, uint8_t b);
uint32_t vkf_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a);

#endif /* VULKAN_FASTOS_H */

