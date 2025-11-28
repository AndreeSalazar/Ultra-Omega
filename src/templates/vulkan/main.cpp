/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN MAIN - Aplicación completa de Vulkan
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: main.cpp
 * Descripción: Punto de entrada que une todos los módulos de Vulkan
 * 
 * ESTRUCTURA DE NODOS (Ultra-Omega):
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                        VULKAN APP - MAPA DE NODOS                       │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │                                                                         │
 * │                        ┌─────────────────────┐                          │
 * │                        │  vulkan_types.h     │ ← Tipos base             │
 * │                        └──────────┬──────────┘                          │
 * │                                   │                                     │
 * │                        ┌──────────▼──────────┐                          │
 * │                        │   instance.cpp      │ ← Crear instancia        │
 * │                        └──────────┬──────────┘                          │
 * │                                   │                                     │
 * │                        ┌──────────▼──────────┐                          │
 * │                        │    device.cpp       │ ← Seleccionar GPU        │
 * │                        └──────────┬──────────┘                          │
 * │                                   │                                     │
 * │                        ┌──────────▼──────────┐                          │
 * │                        │  swapchain.cpp      │ ← Crear swapchain        │
 * │                        └──────────┬──────────┘                          │
 * │                                   │                                     │
 * │                        ┌──────────▼──────────┐                          │
 * │                        │   pipeline.cpp      │ ← Graphics pipeline      │
 * │                        └──────────┬──────────┘                          │
 * │                    ┌──────────────┼──────────────┐                      │
 * │                    │              │              │                      │
 * │         ┌──────────▼────┐  ┌──────▼──────┐  ┌────▼──────────┐           │
 * │         │ shader.vert   │  │ shader.frag │  │ buffers.cpp   │           │
 * │         └───────────────┘  └─────────────┘  └───────┬───────┘           │
 * │                                                     │                   │
 * │                        ┌────────────────────────────▼────┐              │
 * │                        │       commands.cpp              │              │
 * │                        └────────────────┬────────────────┘              │
 * │                                         │                               │
 * │                        ┌────────────────▼────────────────┐              │
 * │                        │         sync.cpp                │              │
 * │                        └────────────────┬────────────────┘              │
 * │                                         │                               │
 * │                        ┌────────────────▼────────────────┐              │
 * │                        │      render_loop.cpp            │              │
 * │                        └────────────────┬────────────────┘              │
 * │                                         │                               │
 * │                        ┌────────────────▼────────────────┐              │
 * │                        │         main.cpp                │              │
 * │                        │    [NODO FINAL - COMBINA TODO]  │              │
 * │                        └─────────────────────────────────┘              │
 * │                                                                         │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * Compilar:
 *   # Compilar shaders
 *   glslc shader.vert -o shaders/vert.spv
 *   glslc shader.frag -o shaders/frag.spv
 *   
 *   # Compilar aplicación
 *   g++ -std=c++17 -o vulkan_app main.cpp instance.cpp device.cpp swapchain.cpp \
 *       pipeline.cpp buffers.cpp commands.cpp sync.cpp render_loop.cpp \
 *       -lglfw -lvulkan -ldl -lpthread -lX11 -lXxf86vm -lXrandr -lXi
 *
 * En Windows (con MSVC):
 *   cl /EHsc /std:c++17 main.cpp *.cpp /link glfw3.lib vulkan-1.lib
 *
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>
#include <cstdlib>

/* ═══════════════════════════════════════════════════════════════════════════
 * DECLARACIONES EXTERNAS (de otros módulos)
 * ═══════════════════════════════════════════════════════════════════════════
 */

// instance.cpp
extern void createInstance(VulkanContext& ctx);
extern void setupDebugMessenger(VulkanContext& ctx);
extern void cleanupInstance(VulkanContext& ctx);

// device.cpp
extern void pickPhysicalDevice(VulkanContext& ctx);
extern void createLogicalDevice(VulkanContext& ctx);
extern void cleanupDevice(VulkanContext& ctx);

// swapchain.cpp
extern void createSwapChain(VulkanContext& ctx);
extern void createImageViews(VulkanContext& ctx);
extern void cleanupSwapChain(VulkanContext& ctx);

// pipeline.cpp
extern void createRenderPass(VulkanContext& ctx);
extern void createDescriptorSetLayout(VulkanContext& ctx);
extern void createGraphicsPipeline(VulkanContext& ctx);
extern void createFramebuffers(VulkanContext& ctx);
extern void cleanupPipeline(VulkanContext& ctx);

// buffers.cpp
extern void createVertexBuffer(VulkanContext& ctx);
extern void createIndexBuffer(VulkanContext& ctx);
extern void createUniformBuffers(VulkanContext& ctx);
extern void cleanupBuffers(VulkanContext& ctx);

// commands.cpp
extern void createCommandPool(VulkanContext& ctx);
extern void createCommandBuffers(VulkanContext& ctx);
extern void cleanupCommands(VulkanContext& ctx);

// sync.cpp
extern void createSyncObjects(VulkanContext& ctx);
extern void cleanupSync(VulkanContext& ctx);

// render_loop.cpp
extern void mainLoop(VulkanContext& ctx);

/* ═══════════════════════════════════════════════════════════════════════════
 * CALLBACKS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void framebufferResizeCallback(GLFWwindow* window, int width, int height) {
    auto ctx = reinterpret_cast<VulkanContext*>(glfwGetWindowUserPointer(window));
    ctx->framebufferResized = true;
}

static void keyCallback(GLFWwindow* window, int key, int scancode, int action, int mods) {
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, GLFW_TRUE);
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR VENTANA
 * ═══════════════════════════════════════════════════════════════════════════
 */

void initWindow(VulkanContext& ctx) {
    glfwInit();

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);

    ctx.window = glfwCreateWindow(WIDTH, HEIGHT, "Vulkan App - Ultra-Omega", 
        nullptr, nullptr);
    
    glfwSetWindowUserPointer(ctx.window, &ctx);
    glfwSetFramebufferSizeCallback(ctx.window, framebufferResizeCallback);
    glfwSetKeyCallback(ctx.window, keyCallback);

    std::cout << "[OK] Ventana GLFW creada (" << WIDTH << "x" << HEIGHT << ")" 
              << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR SUPERFICIE
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createSurface(VulkanContext& ctx) {
    if (glfwCreateWindowSurface(ctx.instance, ctx.window, nullptr, 
        &ctx.surface) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear superficie de ventana!");
    }
    std::cout << "[OK] Superficie Vulkan creada" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR DESCRIPTOR POOL Y SETS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createDescriptorPool(VulkanContext& ctx) {
    std::array<VkDescriptorPoolSize, 2> poolSizes{};
    poolSizes[0].type = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
    poolSizes[0].descriptorCount = static_cast<uint32_t>(MAX_FRAMES_IN_FLIGHT);
    poolSizes[1].type = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
    poolSizes[1].descriptorCount = static_cast<uint32_t>(MAX_FRAMES_IN_FLIGHT);

    VkDescriptorPoolCreateInfo poolInfo{};
    poolInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
    poolInfo.poolSizeCount = static_cast<uint32_t>(poolSizes.size());
    poolInfo.pPoolSizes = poolSizes.data();
    poolInfo.maxSets = static_cast<uint32_t>(MAX_FRAMES_IN_FLIGHT);

    if (vkCreateDescriptorPool(ctx.device, &poolInfo, nullptr, 
        &ctx.descriptorPool) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear descriptor pool!");
    }
    std::cout << "[OK] Descriptor pool creado" << std::endl;
}

void createDescriptorSets(VulkanContext& ctx) {
    std::vector<VkDescriptorSetLayout> layouts(MAX_FRAMES_IN_FLIGHT, 
        ctx.descriptorSetLayout);
    
    VkDescriptorSetAllocateInfo allocInfo{};
    allocInfo.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
    allocInfo.descriptorPool = ctx.descriptorPool;
    allocInfo.descriptorSetCount = static_cast<uint32_t>(MAX_FRAMES_IN_FLIGHT);
    allocInfo.pSetLayouts = layouts.data();

    ctx.descriptorSets.resize(MAX_FRAMES_IN_FLIGHT);
    if (vkAllocateDescriptorSets(ctx.device, &allocInfo, 
        ctx.descriptorSets.data()) != VK_SUCCESS) {
        throw std::runtime_error("Error al asignar descriptor sets!");
    }

    for (size_t i = 0; i < MAX_FRAMES_IN_FLIGHT; i++) {
        VkDescriptorBufferInfo bufferInfo{};
        bufferInfo.buffer = ctx.uniformBuffers[i];
        bufferInfo.offset = 0;
        bufferInfo.range = sizeof(UniformBufferObject);

        VkWriteDescriptorSet descriptorWrite{};
        descriptorWrite.sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        descriptorWrite.dstSet = ctx.descriptorSets[i];
        descriptorWrite.dstBinding = 0;
        descriptorWrite.dstArrayElement = 0;
        descriptorWrite.descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        descriptorWrite.descriptorCount = 1;
        descriptorWrite.pBufferInfo = &bufferInfo;

        vkUpdateDescriptorSets(ctx.device, 1, &descriptorWrite, 0, nullptr);
    }
    std::cout << "[OK] Descriptor sets creados" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZAR VULKAN (ORDEN CORRECTO)
 * ═══════════════════════════════════════════════════════════════════════════
 */

void initVulkan(VulkanContext& ctx) {
    std::cout << "\n═══════════════════════════════════════════════════════════════" 
              << std::endl;
    std::cout << "             INICIALIZANDO VULKAN                              " 
              << std::endl;
    std::cout << "═══════════════════════════════════════════════════════════════\n" 
              << std::endl;

    createInstance(ctx);           // 1. Instancia
    setupDebugMessenger(ctx);      // 2. Debug
    createSurface(ctx);            // 3. Superficie
    pickPhysicalDevice(ctx);       // 4. GPU física
    createLogicalDevice(ctx);      // 5. Dispositivo lógico
    createSwapChain(ctx);          // 6. Swapchain
    createImageViews(ctx);         // 7. Image views
    createRenderPass(ctx);         // 8. Render pass
    createDescriptorSetLayout(ctx);// 9. Descriptor layout
    createGraphicsPipeline(ctx);   // 10. Pipeline
    createFramebuffers(ctx);       // 11. Framebuffers
    createCommandPool(ctx);        // 12. Command pool
    createVertexBuffer(ctx);       // 13. Vertex buffer
    createIndexBuffer(ctx);        // 14. Index buffer
    createUniformBuffers(ctx);     // 15. Uniform buffers
    createDescriptorPool(ctx);     // 16. Descriptor pool
    createDescriptorSets(ctx);     // 17. Descriptor sets
    createCommandBuffers(ctx);     // 18. Command buffers
    createSyncObjects(ctx);        // 19. Sincronización

    ctx.currentFrame = 0;
    ctx.framebufferResized = false;

    std::cout << "\n═══════════════════════════════════════════════════════════════" 
              << std::endl;
    std::cout << "             VULKAN INICIALIZADO CORRECTAMENTE                 " 
              << std::endl;
    std::cout << "═══════════════════════════════════════════════════════════════\n" 
              << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CLEANUP (ORDEN INVERSO)
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanup(VulkanContext& ctx) {
    std::cout << "\n═══════════════════════════════════════════════════════════════" 
              << std::endl;
    std::cout << "             LIMPIANDO RECURSOS                                " 
              << std::endl;
    std::cout << "═══════════════════════════════════════════════════════════════\n" 
              << std::endl;

    cleanupSwapChain(ctx);
    
    vkDestroyDescriptorPool(ctx.device, ctx.descriptorPool, nullptr);
    
    cleanupBuffers(ctx);
    cleanupSync(ctx);
    cleanupCommands(ctx);
    cleanupPipeline(ctx);
    
    vkDestroySurfaceKHR(ctx.instance, ctx.surface, nullptr);
    
    cleanupDevice(ctx);
    cleanupInstance(ctx);

    glfwDestroyWindow(ctx.window);
    glfwTerminate();

    std::cout << "\n[OK] Limpieza completa" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * MAIN
 * ═══════════════════════════════════════════════════════════════════════════
 */

int main() {
    VulkanContext ctx{};

    try {
        std::cout << R"(
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║   ██╗   ██╗██╗   ██╗██╗     ██╗  ██╗ █████╗ ███╗   ██╗                    ║
║   ██║   ██║██║   ██║██║     ██║ ██╔╝██╔══██╗████╗  ██║                    ║
║   ██║   ██║██║   ██║██║     █████╔╝ ███████║██╔██╗ ██║                    ║
║   ╚██╗ ██╔╝██║   ██║██║     ██╔═██╗ ██╔══██║██║╚██╗██║                    ║
║    ╚████╔╝ ╚██████╔╝███████╗██║  ██╗██║  ██║██║ ╚████║                    ║
║     ╚═══╝   ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝                    ║
║                                                                           ║
║   Ultra-Omega Vulkan Template                                             ║
║   Desarrollado por: Eddi Andreé Salazar Matos                             ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
)" << std::endl;

        initWindow(ctx);
        initVulkan(ctx);
        mainLoop(ctx);
        cleanup(ctx);

    } catch (const std::exception& e) {
        std::cerr << "\n[ERROR] " << e.what() << std::endl;
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}

