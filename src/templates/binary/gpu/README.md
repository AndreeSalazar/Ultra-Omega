# 🎮 Templates GPU - SPIR-V Bytecode (Vulkan)

## Estructura de Archivos

Cada template GPU tiene varios archivos:

1. **`*.json`** - Metadata con inputs/outputs/uniforms
2. **`*.glsl`** - Código fuente GLSL (referencia)
3. **`*.spv.hex`** - SPIR-V en representación hexadecimal
4. **`*.spv`** - SPIR-V compilado (binario)

## Templates Disponibles

### 🔺 vertex_passthrough
Vertex shader básico sin transformación.
- **Stage**: Vertex
- **Inputs**: 
  - `inPosition` (location 0, vec3)
  - `inColor` (location 1, vec3)
- **Outputs**: 
  - `gl_Position` (builtin)
  - `fragColor` (location 0, vec3)

### 🎯 vertex_mvp
Vertex shader con transformación Model-View-Projection.
- **Stage**: Vertex
- **Inputs**: 
  - `inPosition` (location 0, vec3)
  - `inColor` (location 1, vec3)
  - `inTexCoord` (location 2, vec2)
- **Uniforms**: 
  - `ubo.model` (mat4)
  - `ubo.view` (mat4)
  - `ubo.proj` (mat4)
- **Outputs**: 
  - `gl_Position` (builtin)
  - `fragColor` (location 0, vec3)
  - `fragTexCoord` (location 1, vec2)

### 🎨 fragment_color
Fragment shader con color interpolado.
- **Stage**: Fragment
- **Inputs**: 
  - `fragColor` (location 0, vec3)
- **Outputs**: 
  - `outColor` (location 0, vec4)

## Compilar GLSL a SPIR-V

### Usando glslc (Vulkan SDK)
```bash
glslc shader.vert -o shader.vert.spv
glslc shader.frag -o shader.frag.spv
```

### Usando glslangValidator
```bash
glslangValidator -V shader.vert -o shader.vert.spv
glslangValidator -V shader.frag -o shader.frag.spv
```

## Estructura SPIR-V

```
┌─────────────────────────────────────┐
│ Magic Number: 0x07230203            │
│ Version: 1.3.0                      │
│ Generator: Ultra-Omega              │
│ Bound: (max ID + 1)                 │
│ Schema: 0                           │
├─────────────────────────────────────┤
│ OpCapability Shader                 │
│ OpMemoryModel Logical GLSL450       │
│ OpEntryPoint ...                    │
├─────────────────────────────────────┤
│ Decorations (Location, Binding...)  │
├─────────────────────────────────────┤
│ Type declarations                   │
│ Variable declarations               │
├─────────────────────────────────────┤
│ Function definitions                │
└─────────────────────────────────────┘
```

## Integración con Vulkan

```cpp
// Cargar SPIR-V
VkShaderModuleCreateInfo createInfo = {};
createInfo.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
createInfo.codeSize = spirv_code.size();
createInfo.pCode = reinterpret_cast<const uint32_t*>(spirv_code.data());

VkShaderModule shaderModule;
vkCreateShaderModule(device, &createInfo, nullptr, &shaderModule);
```

## Descriptor Set Layout

Para shaders con uniforms (como vertex_mvp):

```cpp
VkDescriptorSetLayoutBinding uboBinding = {};
uboBinding.binding = 0;
uboBinding.descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
uboBinding.descriptorCount = 1;
uboBinding.stageFlags = VK_SHADER_STAGE_VERTEX_BIT;
```

## Vertex Input Description

```cpp
// Para vertex_passthrough / vertex_mvp
VkVertexInputBindingDescription bindingDesc = {};
bindingDesc.binding = 0;
bindingDesc.stride = sizeof(Vertex);
bindingDesc.inputRate = VK_VERTEX_INPUT_RATE_VERTEX;

VkVertexInputAttributeDescription attrDescs[3] = {};
// Position
attrDescs[0].binding = 0;
attrDescs[0].location = 0;
attrDescs[0].format = VK_FORMAT_R32G32B32_SFLOAT;
attrDescs[0].offset = offsetof(Vertex, position);
// Color
attrDescs[1].binding = 0;
attrDescs[1].location = 1;
attrDescs[1].format = VK_FORMAT_R32G32B32_SFLOAT;
attrDescs[1].offset = offsetof(Vertex, color);
// TexCoord (solo vertex_mvp)
attrDescs[2].binding = 0;
attrDescs[2].location = 2;
attrDescs[2].format = VK_FORMAT_R32G32_SFLOAT;
attrDescs[2].offset = offsetof(Vertex, texCoord);
```

