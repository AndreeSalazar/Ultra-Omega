/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN WINDOW MANAGER - Gestión de ventanas GLFW
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: window_manager.cpp
 * Descripción: Gestión de ventanas y mensajes con GLFW para Vulkan
 * 
 * USO: Este nodo puede ser heredado para gestión de ventanas
 * Ejemplo: Conecta este nodo a main.cpp para usar window_manager con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <stdexcept>

// ═══════════════════════════════════════════════════════════════════════════
// GESTIÓN DE VENTANAS GLFW
// ═══════════════════════════════════════════════════════════════════════════

struct Window {
    GLFWwindow* window;
    int width;
    int height;
    bool minimized;
    bool shouldClose;
    
    Window() : window(nullptr), width(WIDTH), height(HEIGHT), minimized(false), shouldClose(false) {}
};

static Window g_window;
static void framebufferResizeCallback(GLFWwindow* window, int width, int height);

// Crear ventana GLFW
bool CreateWindow(Window& win, const char* title) {
    glfwInit();
    
    // GLFW no debe crear contexto OpenGL (Vulkan lo maneja)
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);
    
    win.window = glfwCreateWindow(win.width, win.height, title, nullptr, nullptr);
    if (!win.window) {
        glfwTerminate();
        return false;
    }
    
    glfwSetWindowUserPointer(win.window, &win);
    glfwSetFramebufferSizeCallback(win.window, framebufferResizeCallback);
    
    return true;
}

// Procesar eventos de ventana
void ProcessWindowEvents(Window& win) {
    glfwPollEvents();
    win.shouldClose = glfwWindowShouldClose(win.window);
    win.minimized = (win.width == 0 || win.height == 0);
}

// Obtener extensiones requeridas por GLFW
std::vector<const char*> GetRequiredWindowExtensions() {
    uint32_t glfwExtensionCount = 0;
    const char** glfwExtensions;
    glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);
    
    std::vector<const char*> extensions(glfwExtensions, glfwExtensions + glfwExtensionCount);
    return extensions;
}

// Crear surface de Vulkan desde ventana GLFW
VkSurfaceKHR CreateWindowSurface(VkInstance instance, Window& win) {
    VkSurfaceKHR surface;
    if (glfwCreateWindowSurface(instance, win.window, nullptr, &surface) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear surface de ventana!");
    }
    return surface;
}

// Obtener ventana global
Window& GetWindow() {
    return g_window;
}

// Callback de resize
void framebufferResizeCallback(GLFWwindow* window, int width, int height) {
    auto* win = reinterpret_cast<Window*>(glfwGetWindowUserPointer(window));
    if (win) {
        win->width = width;
        win->height = height;
    }
}

// Limpiar ventana
void CleanupWindow(Window& win) {
    if (win.window) {
        glfwDestroyWindow(win.window);
        win.window = nullptr;
    }
    glfwTerminate();
}

