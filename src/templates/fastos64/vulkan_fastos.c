/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT + VULKAN - VULKAN ABSTRACTION IMPLEMENTATION
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: vulkan_fastos.c
 * Descripción: Implementación de la capa Vulkan-like para FastOS
 * Hardware: NVIDIA RTX 3060 12GB + AMD Ryzen 5 5600X
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_fastos.h"
#include "framebuffer.h"
#include "memory64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONTEXTO GLOBAL
 * ═══════════════════════════════════════════════════════════════════════════
 */

static VulkanFastOSContext ctx = {0};

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR INSTANCIA
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkResult vkf_create_instance(const VkApplicationInfo* app_info) {
    if (ctx.initialized) {
        return VK_SUCCESS;
    }
    
    /* Inicializar GPU NVIDIA */
    if (!nvidia_init()) {
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    
    ctx.gpu = nvidia_get_gpu();
    if (!ctx.gpu) {
        return VK_ERROR_DEVICE_LOST;
    }
    
    /* Llenar propiedades del dispositivo */
    ctx.properties.vendor_id = ctx.gpu->vendor_id;
    ctx.properties.device_id = ctx.gpu->device_id;
    ctx.properties.device_type = 2; /* VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU */
    
    /* Copiar nombre */
    const char* name = ctx.gpu->chip_name;
    int i = 0;
    while (name[i] && i < 255) {
        ctx.properties.device_name[i] = name[i];
        i++;
    }
    ctx.properties.device_name[i] = '\0';
    
    /* RTX 3060 específico */
    ctx.properties.cuda_cores = RTX3060_CUDA_CORES;
    ctx.properties.sm_count = RTX3060_SM_COUNT;
    ctx.properties.vram_size_mb = RTX3060_VRAM_SIZE_GB * 1024;
    ctx.properties.memory_bus_width = RTX3060_MEMORY_BUS;
    
    /* Límites */
    ctx.properties.max_image_dimension_2d = 16384;
    ctx.properties.max_uniform_buffer_range = 65536;
    ctx.properties.max_storage_buffer_range = 134217728;
    ctx.properties.max_push_constants_size = 256;
    ctx.properties.max_memory_allocation_count = 4096;
    ctx.properties.max_bound_descriptor_sets = 8;
    ctx.properties.max_compute_work_group_count[0] = 65535;
    ctx.properties.max_compute_work_group_count[1] = 65535;
    ctx.properties.max_compute_work_group_count[2] = 65535;
    ctx.properties.max_compute_work_group_size[0] = 1024;
    ctx.properties.max_compute_work_group_size[1] = 1024;
    ctx.properties.max_compute_work_group_size[2] = 64;
    
    /* Características */
    ctx.features.geometry_shader = VK_TRUE;
    ctx.features.tessellation_shader = VK_TRUE;
    ctx.features.multi_viewport = VK_TRUE;
    ctx.features.sampler_anisotropy = VK_TRUE;
    ctx.features.texture_compression_bc = VK_TRUE;
    ctx.features.shader_float64 = VK_TRUE;
    ctx.features.shader_int64 = VK_TRUE;
    
    /* RTX Features */
    ctx.features.ray_tracing = VK_TRUE;         /* RTX 3060 soporta RT */
    ctx.features.mesh_shaders = VK_TRUE;
    ctx.features.variable_rate_shading = VK_TRUE;
    
    /* Memoria */
    ctx.memory_props.memory_type_count = 2;
    
    /* Tipo 0: Device local (VRAM) */
    ctx.memory_props.memory_types[0].property_flags = 0x01; /* DEVICE_LOCAL */
    ctx.memory_props.memory_types[0].heap_index = 0;
    
    /* Tipo 1: Host visible */
    ctx.memory_props.memory_types[1].property_flags = 0x06; /* HOST_VISIBLE | HOST_COHERENT */
    ctx.memory_props.memory_types[1].heap_index = 1;
    
    ctx.memory_props.memory_heap_count = 2;
    ctx.memory_props.memory_heaps[0].size = RTX3060_VRAM_SIZE;
    ctx.memory_props.memory_heaps[0].flags = 0x01; /* DEVICE_LOCAL */
    ctx.memory_props.memory_heaps[1].size = 16ULL * 1024 * 1024 * 1024; /* 16GB RAM */
    ctx.memory_props.memory_heaps[1].flags = 0;
    
    ctx.initialized = true;
    return VK_SUCCESS;
}

void vkf_destroy_instance(void) {
    ctx.initialized = false;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE DISPOSITIVO FÍSICO
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkResult vkf_enumerate_physical_devices(uint32_t* count) {
    if (!ctx.initialized) {
        *count = 0;
        return VK_ERROR_INITIALIZATION_FAILED;
    }
    *count = 1; /* Solo tenemos la RTX 3060 */
    return VK_SUCCESS;
}

VkResult vkf_get_physical_device_properties(VkPhysicalDeviceProperties* props) {
    if (!ctx.initialized) return VK_ERROR_INITIALIZATION_FAILED;
    *props = ctx.properties;
    return VK_SUCCESS;
}

VkResult vkf_get_physical_device_features(VkPhysicalDeviceFeatures* features) {
    if (!ctx.initialized) return VK_ERROR_INITIALIZATION_FAILED;
    *features = ctx.features;
    return VK_SUCCESS;
}

VkResult vkf_get_physical_device_memory_properties(VkPhysicalDeviceMemoryProperties* props) {
    if (!ctx.initialized) return VK_ERROR_INITIALIZATION_FAILED;
    *props = ctx.memory_props;
    return VK_SUCCESS;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE DISPOSITIVO LÓGICO
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkResult vkf_create_device(void) {
    if (!ctx.initialized) return VK_ERROR_INITIALIZATION_FAILED;
    
    /* Obtener framebuffer del sistema */
    fb_get_size(&ctx.fb_width, &ctx.fb_height);
    ctx.fb_pitch = ctx.fb_width * 4;
    
    return VK_SUCCESS;
}

void vkf_destroy_device(void) {
    /* Nada que hacer por ahora */
}

void vkf_device_wait_idle(void) {
    /* Sincronizar */
    mfence();
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE RENDERIZADO
 * ═══════════════════════════════════════════════════════════════════════════
 */

VkResult vkf_begin_frame(void) {
    if (!ctx.initialized) return VK_ERROR_INITIALIZATION_FAILED;
    return VK_SUCCESS;
}

VkResult vkf_end_frame(void) {
    if (!ctx.initialized) return VK_ERROR_INITIALIZATION_FAILED;
    ctx.frames_rendered++;
    return VK_SUCCESS;
}

void vkf_clear_color(float r, float g, float b, float a) {
    uint8_t ri = (uint8_t)(r * 255.0f);
    uint8_t gi = (uint8_t)(g * 255.0f);
    uint8_t bi = (uint8_t)(b * 255.0f);
    uint32_t color = vkf_rgba(ri, gi, bi, 255);
    fb_clear(color);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * PRIMITIVAS DE DIBUJO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void vkf_draw_pixel(int32_t x, int32_t y, uint32_t color) {
    if (x < 0 || y < 0 || (uint32_t)x >= ctx.fb_width || (uint32_t)y >= ctx.fb_height) return;
    fb_put_pixel(x, y, color);
    ctx.draw_calls++;
}

void vkf_draw_line(int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint32_t color) {
    fb_draw_line(x0, y0, x1, y1, color);
    ctx.draw_calls++;
}

void vkf_draw_rect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color) {
    fb_draw_rect(x, y, w, h, color);
    ctx.draw_calls++;
}

void vkf_fill_rect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color) {
    fb_fill_rect(x, y, w, h, color);
    ctx.draw_calls++;
}

void vkf_draw_triangle(int32_t x0, int32_t y0, int32_t x1, int32_t y1, 
                       int32_t x2, int32_t y2, uint32_t color) {
    vkf_draw_line(x0, y0, x1, y1, color);
    vkf_draw_line(x1, y1, x2, y2, color);
    vkf_draw_line(x2, y2, x0, y0, color);
    ctx.triangles_drawn++;
}

/* Rasterización de triángulo usando scanline */
void vkf_fill_triangle(int32_t x0, int32_t y0, int32_t x1, int32_t y1,
                       int32_t x2, int32_t y2, uint32_t color) {
    /* Ordenar vértices por Y */
    if (y0 > y1) { int32_t t = y0; y0 = y1; y1 = t; t = x0; x0 = x1; x1 = t; }
    if (y0 > y2) { int32_t t = y0; y0 = y2; y2 = t; t = x0; x0 = x2; x2 = t; }
    if (y1 > y2) { int32_t t = y1; y1 = y2; y2 = t; t = x1; x1 = x2; x2 = t; }
    
    int32_t total_height = y2 - y0;
    if (total_height == 0) return;
    
    for (int32_t y = y0; y <= y2; y++) {
        bool second_half = (y > y1) || (y1 == y0);
        int32_t segment_height = second_half ? (y2 - y1) : (y1 - y0);
        if (segment_height == 0) continue;
        
        float alpha = (float)(y - y0) / total_height;
        float beta = second_half ? 
            (float)(y - y1) / segment_height :
            (float)(y - y0) / segment_height;
        
        int32_t ax = x0 + (int32_t)((x2 - x0) * alpha);
        int32_t bx = second_half ?
            x1 + (int32_t)((x2 - x1) * beta) :
            x0 + (int32_t)((x1 - x0) * beta);
        
        if (ax > bx) { int32_t t = ax; ax = bx; bx = t; }
        
        for (int32_t x = ax; x <= bx; x++) {
            vkf_draw_pixel(x, y, color);
        }
    }
    
    ctx.triangles_drawn++;
    ctx.draw_calls++;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INFORMACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

VulkanFastOSContext* vkf_get_context(void) {
    return ctx.initialized ? &ctx : NULL;
}

void vkf_print_info(void) {
    fb_set_colors(FB_COLOR_MAGENTA, FB_COLOR_TERM_BG);
    fb_print("\n╔═══════════════════════════════════════════════════════════════╗\n");
    fb_print("║            FASTOS + VULKAN - SISTEMA DE EDDI                   ║\n");
    fb_print("╠═══════════════════════════════════════════════════════════════╣\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ API: ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("FastOS-Vulkan 1.0\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ GPU: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print(ctx.properties.device_name);
    fb_print("\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ VRAM: ");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("12288 MB (12 GB GDDR6)\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ CUDA Cores: ");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("3584\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ Ray Tracing: ");
    fb_set_colors(ctx.features.ray_tracing ? FB_COLOR_GREEN : FB_COLOR_RED, FB_COLOR_TERM_BG);
    fb_print(ctx.features.ray_tracing ? "Soportado (RT Cores)\n" : "No soportado\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ Mesh Shaders: ");
    fb_set_colors(ctx.features.mesh_shaders ? FB_COLOR_GREEN : FB_COLOR_RED, FB_COLOR_TERM_BG);
    fb_print(ctx.features.mesh_shaders ? "Soportado\n" : "No soportado\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_set_colors(FB_COLOR_MAGENTA, FB_COLOR_TERM_BG);
    fb_print("╚═══════════════════════════════════════════════════════════════╝\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
}

void vkf_print_stats(void) {
    fb_print("Frames: ");
    /* TODO: Convertir a string */
    fb_print("(ver ctx.frames_rendered)\n");
    fb_print("Draw calls: ");
    fb_print("(ver ctx.draw_calls)\n");
    fb_print("Triangles: ");
    fb_print("(ver ctx.triangles_drawn)\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t vkf_rgb(uint8_t r, uint8_t g, uint8_t b) {
    return 0xFF000000 | ((uint32_t)r << 16) | ((uint32_t)g << 8) | b;
}

uint32_t vkf_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    return ((uint32_t)a << 24) | ((uint32_t)r << 16) | ((uint32_t)g << 8) | b;
}

