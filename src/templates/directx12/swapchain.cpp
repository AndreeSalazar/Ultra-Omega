/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 SWAPCHAIN - Cadena de intercambio (Swap Chain)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: swapchain.cpp
 * Descripción: Creación y manejo de la cadena de intercambio para presentación
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"
#include <iostream>

// Variables globales de swapchain
ComPtr<IDXGISwapChain4> g_swapChain;
ComPtr<ID3D12DescriptorHeap> g_rtvHeap;
ComPtr<ID3D12Resource> g_renderTargets[BACK_BUFFER_COUNT];
UINT g_rtvDescriptorSize;
UINT g_frameIndex = 0;
HWND g_hwnd = nullptr;

// ═══════════════════════════════════════════════════════════════════════════
// CREAR DESCRIPTOR HEAP PARA RENDER TARGETS
// ═══════════════════════════════════════════════════════════════════════════

void CreateRenderTargetViewHeap() {
    D3D12_DESCRIPTOR_HEAP_DESC rtvHeapDesc = {};
    rtvHeapDesc.NumDescriptors = BACK_BUFFER_COUNT;
    rtvHeapDesc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
    rtvHeapDesc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
    
    DX_CHECK(GetDevice()->CreateDescriptorHeap(&rtvHeapDesc, IID_PPV_ARGS(&g_rtvHeap)));
    
    g_rtvDescriptorSize = GetDevice()->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
    
    std::cout << "Render Target View Heap creado" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// CREAR SWAPCHAIN
// ═══════════════════════════════════════════════════════════════════════════

void CreateSwapChain(HWND hwnd) {
    g_hwnd = hwnd;
    
    ComPtr<IDXGIFactory7> factory;
    DX_CHECK(CreateDXGIFactory2(
#ifdef _DEBUG
        DXGI_CREATE_FACTORY_DEBUG,
#else
        0,
#endif
        IID_PPV_ARGS(&factory)));
    
    // Descripción del swapchain
    DXGI_SWAP_CHAIN_DESC1 swapChainDesc = {};
    swapChainDesc.Width = WIDTH;
    swapChainDesc.Height = HEIGHT;
    swapChainDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    swapChainDesc.SampleDesc.Count = 1;
    swapChainDesc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    swapChainDesc.BufferCount = BACK_BUFFER_COUNT;
    swapChainDesc.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
    swapChainDesc.Flags = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH;
    
    ComPtr<IDXGISwapChain1> swapChain1;
    DX_CHECK(factory->CreateSwapChainForHwnd(
        GetCommandQueue(),
        hwnd,
        &swapChainDesc,
        nullptr,
        nullptr,
        &swapChain1));
    
    DX_CHECK(swapChain1.As(&g_swapChain));
    
    // Prevenir cambio de modo completo automático
    DX_CHECK(factory->MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER));
    
    g_frameIndex = g_swapChain->GetCurrentBackBufferIndex();
    
    std::cout << "Swapchain creada (" << WIDTH << "x" << HEIGHT << ")" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// CREAR RENDER TARGET VIEWS
// ═══════════════════════════════════════════════════════════════════════════

void CreateRenderTargetViews() {
    CD3DX12_CPU_DESCRIPTOR_HANDLE rtvHandle(g_rtvHeap->GetCPUDescriptorHandleForHeapStart());
    
    for (UINT n = 0; n < BACK_BUFFER_COUNT; n++) {
        DX_CHECK(g_swapChain->GetBuffer(n, IID_PPV_ARGS(&g_renderTargets[n])));
        GetDevice()->CreateRenderTargetView(g_renderTargets[n].Get(), nullptr, rtvHandle);
        rtvHandle.Offset(1, g_rtvDescriptorSize);
    }
    
    std::cout << "Render Target Views creados" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// OBTENER RECURSOS Y HANDLES
// ═══════════════════════════════════════════════════════════════════════════

IDXGISwapChain4* GetSwapChain() {
    return g_swapChain.Get();
}

ID3D12Resource* GetCurrentRenderTarget() {
    return g_renderTargets[g_frameIndex].Get();
}

CD3DX12_CPU_DESCRIPTOR_HANDLE GetCurrentRTVHandle() {
    return CD3DX12_CPU_DESCRIPTOR_HANDLE(
        g_rtvHeap->GetCPUDescriptorHandleForHeapStart(),
        g_frameIndex,
        g_rtvDescriptorSize);
}

UINT GetCurrentFrameIndex() {
    return g_frameIndex;
}

// ═══════════════════════════════════════════════════════════════════════════
// PRESENTAR FRAME
// ═══════════════════════════════════════════════════════════════════════════

void PresentSwapChain() {
    DX_CHECK(g_swapChain->Present(1, 0));
    g_frameIndex = g_swapChain->GetCurrentBackBufferIndex();
}

// ═══════════════════════════════════════════════════════════════════════════
// RESIZE SWAPCHAIN (para cambio de tamaño de ventana)
// ═══════════════════════════════════════════════════════════════════════════

void ResizeSwapChain(UINT width, UINT height) {
    // Esperar a que termine el frame actual
    // (se necesita implementar fences para esto correctamente)
    
    // Liberar render targets existentes
    for (UINT i = 0; i < BACK_BUFFER_COUNT; i++) {
        g_renderTargets[i].Reset();
    }
    
    // Resize
    DX_CHECK(g_swapChain->ResizeBuffers(
        BACK_BUFFER_COUNT,
        width,
        height,
        DXGI_FORMAT_R8G8B8A8_UNORM,
        0));
    
    g_frameIndex = g_swapChain->GetCurrentBackBufferIndex();
    
    // Recrear render targets
    CreateRenderTargetViews();
    
    std::cout << "Swapchain redimensionada a " << width << "x" << height << std::endl;
}

