/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 MAIN - Aplicación completa de DirectX 12
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: main.cpp
 * Descripción: Punto de entrada que une todos los módulos de DirectX 12
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "adapter.cpp"
#include "device.cpp"
#include "swapchain.cpp"
#include "command_allocator.cpp"
#include "root_signature.cpp"
#include "pipeline_state.cpp"
#include "command_list.cpp"
#include "buffers.cpp"
#include "render_loop.cpp"
#include <windows.h>
#include <iostream>

// ═══════════════════════════════════════════════════════════════════════════
// VENTANA
// ═══════════════════════════════════════════════════════════════════════════

HWND g_hwnd = nullptr;
bool g_running = true;

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    switch (uMsg) {
    case WM_DESTROY:
        g_running = false;
        PostQuitMessage(0);
        return 0;
    case WM_SIZE:
        // Resize swapchain si es necesario
        // ResizeSwapChain(LOWORD(lParam), HIWORD(lParam));
        return 0;
    }
    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

HWND CreateWindow(HINSTANCE hInstance, int nCmdShow) {
    const wchar_t CLASS_NAME[] = L"DirectX12WindowClass";
    
    WNDCLASSEX wc = {};
    wc.cbSize = sizeof(WNDCLASSEX);
    wc.style = CS_HREDRAW | CS_VREDRAW;
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance;
    wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wc.lpszClassName = CLASS_NAME;
    
    RegisterClassEx(&wc);
    
    RECT rect = { 0, 0, (LONG)WIDTH, (LONG)HEIGHT };
    AdjustWindowRect(&rect, WS_OVERLAPPEDWINDOW, FALSE);
    
    HWND hwnd = CreateWindowEx(
        0,
        CLASS_NAME,
        L"DirectX 12 Application",
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT,
        rect.right - rect.left,
        rect.bottom - rect.top,
        nullptr,
        nullptr,
        hInstance,
        nullptr);
    
    if (hwnd == nullptr) {
        return nullptr;
    }
    
    ShowWindow(hwnd, nCmdShow);
    return hwnd;
}

// ═══════════════════════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════════════════════

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    try {
        // Crear ventana
        g_hwnd = CreateWindow(hInstance, nCmdShow);
        if (!g_hwnd) {
            throw std::runtime_error("No se pudo crear la ventana");
        }
        
        // Inicializar DirectX 12
        std::cout << "=== Inicializando DirectX 12 ===" << std::endl;
        
        // 1. Enumerar adaptadores
        PrintAdapterInfo();
        
        // 2. Crear dispositivo
        CreateDevice();
        CreateCommandQueue();
        
        // 3. Crear swapchain
        CreateSwapChain(g_hwnd);
        CreateRenderTargetViewHeap();
        CreateRenderTargetViews();
        
        // 4. Crear command allocators
        CreateCommandAllocators();
        
        // 5. Crear root signature
        CreateRootSignature();
        
        // 6. Crear pipeline state
        CreatePipelineState();
        
        // 7. Crear command list
        CreateCommandList();
        
        // 8. Crear buffers
        Vertex vertices[] = {
            { { -0.5f, -0.5f, 0.0f }, { 1.0f, 0.0f, 0.0f, 1.0f }, { 0.0f, 0.0f } },
            { {  0.5f, -0.5f, 0.0f }, { 0.0f, 1.0f, 0.0f, 1.0f }, { 1.0f, 0.0f } },
            { {  0.0f,  0.5f, 0.0f }, { 0.0f, 0.0f, 1.0f, 1.0f }, { 0.5f, 1.0f } },
        };
        CreateVertexBuffer(vertices, 3);
        
        UINT indices[] = { 0, 1, 2 };
        CreateIndexBuffer(indices, 3);
        
        CreateConstantBuffers();
        
        // 9. Crear fences
        CreateFences();
        
        std::cout << "=== Inicialización completa ===" << std::endl;
        
        // Loop principal
        MSG msg = {};
        while (g_running) {
            while (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE)) {
                TranslateMessage(&msg);
                DispatchMessage(&msg);
            }
            
            RenderFrame();
        }
        
        // Limpiar
        WaitForPreviousFrame(GetCurrentFrameIndex());
        CloseHandle(g_fenceEvent);
        
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
}

