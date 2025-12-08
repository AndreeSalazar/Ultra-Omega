/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN HELPERS - Utilidades y funciones helper
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: helpers.cpp
 * Descripción: Funciones helper reutilizables para Vulkan
 * 
 * USO: Este nodo puede ser heredado por otros nodos para usar estas funciones
 * Ejemplo de herencia: Conecta este nodo a otro para acceder a helpers con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <iomanip>
#include <cmath>
#include <cstring>

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE MATEMÁTICAS
// ═══════════════════════════════════════════════════════════════════════════

// Crear matriz de proyección (perspectiva)
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

// Imprimir resultado VkResult
void PrintVkResult(VkResult result, const char* context) {
    std::cout << "[" << context << "] VkResult: ";
    switch (result) {
        case VK_SUCCESS:
            std::cout << "VK_SUCCESS";
            break;
        case VK_NOT_READY:
            std::cout << "VK_NOT_READY";
            break;
        case VK_TIMEOUT:
            std::cout << "VK_TIMEOUT";
            break;
        case VK_EVENT_SET:
            std::cout << "VK_EVENT_SET";
            break;
        case VK_EVENT_RESET:
            std::cout << "VK_EVENT_RESET";
            break;
        case VK_INCOMPLETE:
            std::cout << "VK_INCOMPLETE";
            break;
        case VK_ERROR_OUT_OF_HOST_MEMORY:
            std::cout << "VK_ERROR_OUT_OF_HOST_MEMORY";
            break;
        case VK_ERROR_OUT_OF_DEVICE_MEMORY:
            std::cout << "VK_ERROR_OUT_OF_DEVICE_MEMORY";
            break;
        case VK_ERROR_INITIALIZATION_FAILED:
            std::cout << "VK_ERROR_INITIALIZATION_FAILED";
            break;
        case VK_ERROR_DEVICE_LOST:
            std::cout << "VK_ERROR_DEVICE_LOST";
            break;
        case VK_ERROR_MEMORY_MAP_FAILED:
            std::cout << "VK_ERROR_MEMORY_MAP_FAILED";
            break;
        case VK_ERROR_LAYER_NOT_PRESENT:
            std::cout << "VK_ERROR_LAYER_NOT_PRESENT";
            break;
        case VK_ERROR_EXTENSION_NOT_PRESENT:
            std::cout << "VK_ERROR_EXTENSION_NOT_PRESENT";
            break;
        case VK_ERROR_FEATURE_NOT_PRESENT:
            std::cout << "VK_ERROR_FEATURE_NOT_PRESENT";
            break;
        case VK_ERROR_INCOMPATIBLE_DRIVER:
            std::cout << "VK_ERROR_INCOMPATIBLE_DRIVER";
            break;
        case VK_ERROR_TOO_MANY_OBJECTS:
            std::cout << "VK_ERROR_TOO_MANY_OBJECTS";
            break;
        case VK_ERROR_FORMAT_NOT_SUPPORTED:
            std::cout << "VK_ERROR_FORMAT_NOT_SUPPORTED";
            break;
        default:
            std::cout << "UNKNOWN (0x" << std::hex << result << ")";
            break;
    }
    std::cout << std::dec << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE MEMORIA
// ═══════════════════════════════════════════════════════════════════════════

// Alinear tamaño a múltiplo
VkDeviceSize AlignSize(VkDeviceSize size, VkDeviceSize alignment) {
    return (size + alignment - 1) & ~(alignment - 1);
}

// Obtener tamaño alineado para uniform buffer
VkDeviceSize GetUniformBufferAlignedSize(VkDeviceSize size, VkDeviceSize minAlignment) {
    return AlignSize(size, minAlignment);
}

