/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 HELPERS - Utilidades y funciones helper
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: helpers.cpp
 * Descripción: Funciones helper reutilizables para DirectX12
 * 
 * USO: Este nodo puede ser heredado por otros nodos para usar estas funciones
 * Ejemplo de herencia: Conecta este nodo a otro para acceder a helpers con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include <iostream>
#include <iomanip>
#include <cmath>
#include <cstring>

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE MATEMÁTICAS
// ═══════════════════════════════════════════════════════════════════════════

// Crear matriz de proyección
void CreateProjectionMatrix(float* out, float fov, float aspect, float nearZ, float farZ) {
    float f = 1.0f / tanf(fov * 0.5f);
    float range = farZ - nearZ;
    
    memset(out, 0, sizeof(float) * 16);
    out[0] = f / aspect;
    out[5] = f;
    out[10] = -(farZ + nearZ) / range;
    out[11] = -1.0f;
    out[14] = -(2.0f * farZ * nearZ) / range;
}

// Crear matriz de vista (look-at)
void CreateViewMatrix(float* out, const float* eye, const float* target, const float* up) {
    float zaxis[3], xaxis[3], yaxis[3];
    
    // zaxis = normalize(eye - target)
    zaxis[0] = eye[0] - target[0];
    zaxis[1] = eye[1] - target[1];
    zaxis[2] = eye[2] - target[2];
    float len = sqrtf(zaxis[0]*zaxis[0] + zaxis[1]*zaxis[1] + zaxis[2]*zaxis[2]);
    if (len > 0.0f) {
        zaxis[0] /= len; zaxis[1] /= len; zaxis[2] /= len;
    }
    
    // xaxis = normalize(cross(up, zaxis))
    xaxis[0] = up[1] * zaxis[2] - up[2] * zaxis[1];
    xaxis[1] = up[2] * zaxis[0] - up[0] * zaxis[2];
    xaxis[2] = up[0] * zaxis[1] - up[1] * zaxis[0];
    len = sqrtf(xaxis[0]*xaxis[0] + xaxis[1]*xaxis[1] + xaxis[2]*xaxis[2]);
    if (len > 0.0f) {
        xaxis[0] /= len; xaxis[1] /= len; xaxis[2] /= len;
    }
    
    // yaxis = cross(zaxis, xaxis)
    yaxis[0] = zaxis[1] * xaxis[2] - zaxis[2] * xaxis[1];
    yaxis[1] = zaxis[2] * xaxis[0] - zaxis[0] * xaxis[2];
    yaxis[2] = zaxis[0] * xaxis[1] - zaxis[1] * xaxis[0];
    
    memset(out, 0, sizeof(float) * 16);
    out[0] = xaxis[0]; out[1] = yaxis[0]; out[2] = zaxis[0];
    out[4] = xaxis[1]; out[5] = yaxis[1]; out[6] = zaxis[1];
    out[8] = xaxis[2]; out[9] = yaxis[2]; out[10] = zaxis[2];
    out[12] = -(xaxis[0]*eye[0] + xaxis[1]*eye[1] + xaxis[2]*eye[2]);
    out[13] = -(yaxis[0]*eye[0] + yaxis[1]*eye[1] + yaxis[2]*eye[2]);
    out[14] = -(zaxis[0]*eye[0] + zaxis[1]*eye[1] + zaxis[2]*eye[2]);
    out[15] = 1.0f;
}

// Crear matriz de mundo (identidad)
void CreateIdentityMatrix(float* out) {
    memset(out, 0, sizeof(float) * 16);
    out[0] = out[5] = out[10] = out[15] = 1.0f;
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE DEBUG
// ═══════════════════════════════════════════════════════════════════════════

// Imprimir HRESULT como mensaje legible
void PrintHRESULT(HRESULT hr, const char* context) {
    std::cout << "[" << context << "] HRESULT: 0x" << std::hex << std::uppercase << hr << std::dec;
    if (FAILED(hr)) {
        std::cout << " (FAILED)";
    }
    std::cout << std::endl;
}

// Imprimir información del adaptador
void PrintAdapterInfo(const AdapterInfo& info) {
    std::wcout << L"Adaptador: " << info.desc.Description << std::endl;
    std::cout << "  - Hardware: " << (info.isHardware ? "Sí" : "No") << std::endl;
    std::cout << "  - Memoria VRAM: " << (info.desc.DedicatedVideoMemory / (1024 * 1024)) << " MB" << std::endl;
    std::cout << "  - Memoria Sistema: " << (info.desc.SharedSystemMemory / (1024 * 1024)) << " MB" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE MEMORIA
// ═══════════════════════════════════════════════════════════════════════════

// Alinear tamaño a múltiplo
UINT AlignSize(UINT size, UINT alignment) {
    return (size + alignment - 1) & ~(alignment - 1);
}

// Obtener tamaño alineado para constant buffer
UINT GetConstantBufferAlignedSize(UINT size) {
    return AlignSize(size, CONSTANT_BUFFER_ALIGNMENT);
}

// Obtener tamaño alineado para textura
UINT GetTextureAlignedSize(UINT size) {
    return AlignSize(size, TEXTURE_ALIGNMENT);
}

