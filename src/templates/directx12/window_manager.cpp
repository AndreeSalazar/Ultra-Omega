/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 WINDOW MANAGER - Gestión de ventanas
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: window_manager.cpp
 * Descripción: Gestión de ventanas y mensajes de Windows
 * 
 * USO: Este nodo puede ser heredado para gestión de ventanas
 * Ejemplo: Conecta este nodo a main.cpp para usar window_manager con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include <windows.h>
#include <string>

// ═══════════════════════════════════════════════════════════════════════════
// GESTIÓN DE VENTANAS
// ═══════════════════════════════════════════════════════════════════════════

struct Window {
    HWND hwnd;
    HINSTANCE hinstance;
    bool running;
    UINT width;
    UINT height;
    std::wstring title;
    
    Window() : hwnd(nullptr), hinstance(nullptr), running(false), width(WIDTH), height(HEIGHT) {}
};

static Window g_window;
static LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);

// Crear ventana
bool CreateWindowEx(Window& window, HINSTANCE hInstance, const wchar_t* title, int nCmdShow) {
    window.hinstance = hInstance;
    window.title = title;
    window.running = true;
    
    const wchar_t CLASS_NAME[] = L"DirectX12WindowClass";
    
    WNDCLASSEX wc = {};
    wc.cbSize = sizeof(WNDCLASSEX);
    wc.style = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance;
    wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wc.lpszClassName = CLASS_NAME;
    
    RegisterClassEx(&wc);
    
    RECT rect = { 0, 0, (LONG)window.width, (LONG)window.height };
    AdjustWindowRect(&rect, WS_OVERLAPPEDWINDOW, FALSE);
    
    window.hwnd = ::CreateWindowEx(
        0,
        CLASS_NAME,
        title,
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT,
        rect.right - rect.left,
        rect.bottom - rect.top,
        nullptr,
        nullptr,
        hInstance,
        nullptr);
    
    if (window.hwnd == nullptr) {
        return false;
    }
    
    ShowWindow(window.hwnd, nCmdShow);
    return true;
}

// Procesar mensajes
void ProcessWindowMessages(Window& window) {
    MSG msg = {};
    while (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
}

// Obtener ventana
Window& GetWindow() {
    return g_window;
}

// Callback de ventana
LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    switch (uMsg) {
    case WM_DESTROY:
        g_window.running = false;
        PostQuitMessage(0);
        return 0;
        
    case WM_SIZE:
        if (wParam != SIZE_MINIMIZED) {
            g_window.width = LOWORD(lParam);
            g_window.height = HIWORD(lParam);
            // Notificar resize a swapchain
        }
        return 0;
        
    case WM_KEYDOWN:
        if (wParam == VK_ESCAPE) {
            g_window.running = false;
            PostQuitMessage(0);
        }
        return 0;
    }
    
    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

