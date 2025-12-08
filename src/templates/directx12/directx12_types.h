/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 TYPES - Tipos y estructuras fundamentales
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: directx12_types.h
 * Descripción: Definiciones de tipos y estructuras para DirectX 12
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef DIRECTX12_TYPES_H
#define DIRECTX12_TYPES_H

#include <d3d12.h>
#include <dxgi1_6.h>
#include <wrl/client.h>
#include <vector>
#include <memory>
#include <string>
#include <array>

using Microsoft::WRL::ComPtr;

#ifdef _DEBUG
#include <dxgidebug.h>
#endif

/* ═══════════════════════════════════════════════════════════════════════════
 * CONFIGURACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

const UINT WIDTH = 1280;
const UINT HEIGHT = 720;
const UINT BACK_BUFFER_COUNT = 2;
const UINT MAX_FRAMES_IN_FLIGHT = 2;

// Feature levels soportados
const D3D_FEATURE_LEVEL FEATURE_LEVELS[] = {
    D3D_FEATURE_LEVEL_12_1,
    D3D_FEATURE_LEVEL_12_0,
    D3D_FEATURE_LEVEL_11_1,
    D3D_FEATURE_LEVEL_11_0,
};

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS DE DATOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

// Estructura de vértice
struct Vertex {
    float position[3];
    float color[4];
    float texCoord[2];
    
    static D3D12_INPUT_ELEMENT_DESC* GetInputLayout(UINT& numElements) {
        static D3D12_INPUT_ELEMENT_DESC inputLayout[] = {
            { "POSITION", 0, DXGI_FORMAT_R32G32B32_FLOAT, 0, 0, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0 },
            { "COLOR", 0, DXGI_FORMAT_R32G32B32A32_FLOAT, 0, 12, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0 },
            { "TEXCOORD", 0, DXGI_FORMAT_R32G32_FLOAT, 0, 28, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0 },
        };
        numElements = _countof(inputLayout);
        return inputLayout;
    }
};

// Constantes del frame
struct FrameConstants {
    float projection[16];
    float view[16];
    float world[16];
    float time;
    float padding[3];
};

// Información del adaptador
struct AdapterInfo {
    ComPtr<IDXGIAdapter4> adapter;
    DXGI_ADAPTER_DESC3 desc;
    bool isHardware;
    bool supportsDx12;
};

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

// Helper para conversión de HRESULT a string
inline std::string HrToString(HRESULT hr) {
    char s_str[64] = {};
    sprintf_s(s_str, "HRESULT de 0x%08X", static_cast<UINT>(hr));
    return std::string(s_str);
}

// Macro para comprobar errores DX
#define DX_CHECK(hr) do { \
    HRESULT __hr = (hr); \
    if (FAILED(__hr)) { \
        throw std::runtime_error(HrToString(__hr)); \
    } \
} while(0)

// Alineamiento de constantes
const UINT CONSTANT_BUFFER_ALIGNMENT = 256;

// Alineamiento de texturas
const UINT TEXTURE_ALIGNMENT = 512;

#endif // DIRECTX12_TYPES_H

