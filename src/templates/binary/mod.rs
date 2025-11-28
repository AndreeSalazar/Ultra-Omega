// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Sistema de Templates Binarios
// Backend de ejecución para nodos con código máquina directo
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

// ══════════════════════════════════════════
// TIPOS Y ESTRUCTURAS
// ══════════════════════════════════════════

/// Tipo de template binario
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryTemplateType {
    /// CPU x86_64 machine code
    Cpu,
    /// GPU SPIR-V bytecode
    GpuSpirv,
}

/// Parámetro inyectable en un template
#[derive(Debug, Clone)]
pub struct TemplateParameter {
    pub name: String,
    pub param_type: String,  // "uint32", "uint64", "float", etc.
    pub offset: usize,
    pub size: usize,
    pub default: Option<u64>,
    pub description: String,
}

/// Metadata de un template binario
#[derive(Debug, Clone)]
pub struct BinaryTemplateInfo {
    pub name: String,
    pub template_type: BinaryTemplateType,
    pub architecture: String,
    pub description: String,
    pub size_bytes: usize,
    pub parameters: Vec<TemplateParameter>,
    pub hex_representation: String,
}

/// Template binario cargado en memoria
#[derive(Debug, Clone)]
pub struct BinaryTemplate {
    pub info: BinaryTemplateInfo,
    pub binary_data: Vec<u8>,
}

/// Instancia de un nodo compilado
#[derive(Debug)]
pub struct CompiledNode {
    pub node_id: u32,
    pub template_name: String,
    pub compiled_code: Vec<u8>,
    pub parameter_values: HashMap<String, u64>,
}

// ══════════════════════════════════════════
// TEMPLATES CPU EMBEBIDOS
// ══════════════════════════════════════════

pub mod cpu {
    /// Hello World! - Programa completo que imprime y termina
    /// Bytecode x86_64 Linux (55 bytes)
    pub const HELLO_WORLD: &[u8] = &[
        // mov rax, 1 (syscall: write)
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00,
        // mov rdi, 1 (fd: stdout)
        0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00,
        // lea rsi, [rip+0x13] (buffer address)
        0x48, 0x8D, 0x35, 0x13, 0x00, 0x00, 0x00,
        // mov rdx, 13 (length)
        0x48, 0xC7, 0xC2, 0x0D, 0x00, 0x00, 0x00,
        // syscall
        0x0F, 0x05,
        // mov rax, 60 (syscall: exit)
        0x48, 0xC7, 0xC0, 0x3C, 0x00, 0x00, 0x00,
        // xor rdi, rdi (exit code: 0)
        0x48, 0x31, 0xFF,
        // syscall
        0x0F, 0x05,
        // "Hello World!\n"
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20,  // "Hello "
        0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21,  // "World!"
        0x0A,                                  // '\n'
    ];

    /// Syscall write - Template con parámetros inyectables
    /// Parámetros: fd (offset 10), buffer_addr (offset 17), length (offset 27)
    pub const SYSCALL_WRITE: &[u8] = &[
        // mov rax, 1 (syscall: write)
        0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00,
        // mov rdi, [FD] - placeholder 0xFFFFFFFF
        0x48, 0xC7, 0xC7, 0xFF, 0xFF, 0xFF, 0xFF,
        // movabs rsi, [BUFFER_ADDR] - placeholder 0xFFFFFFFFFFFFFFFF
        0x48, 0xBE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        // mov rdx, [LENGTH] - placeholder 0xFFFFFFFF
        0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF,
        // syscall
        0x0F, 0x05,
        // ret
        0xC3,
    ];

    /// Syscall exit - Template con código de salida inyectable
    /// Parámetros: exit_code (offset 10)
    pub const SYSCALL_EXIT: &[u8] = &[
        // mov rax, 60 (syscall: exit)
        0x48, 0xC7, 0xC0, 0x3C, 0x00, 0x00, 0x00,
        // mov rdi, [EXIT_CODE] - placeholder 0xFFFFFFFF
        0x48, 0xC7, 0xC7, 0xFF, 0xFF, 0xFF, 0xFF,
        // syscall
        0x0F, 0x05,
    ];

    /// Memory alloc (mmap) - Alocar memoria ejecutable
    /// Parámetros: size (offset 10)
    pub const MEMORY_ALLOC: &[u8] = &[
        // mov rax, 9 (syscall: mmap)
        0x48, 0xC7, 0xC0, 0x09, 0x00, 0x00, 0x00,
        // xor rdi, rdi (addr = NULL)
        0x48, 0x31, 0xFF,
        // movabs rsi, [SIZE] - placeholder
        0x48, 0xBE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        // mov rdx, 7 (PROT_READ|WRITE|EXEC)
        0x48, 0xC7, 0xC2, 0x07, 0x00, 0x00, 0x00,
        // mov r10, 0x22 (MAP_PRIVATE|MAP_ANON)
        0x49, 0xC7, 0xC2, 0x22, 0x00, 0x00, 0x00,
        // mov r8, -1 (fd = -1)
        0x49, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF,
        // xor r9, r9 (offset = 0)
        0x4D, 0x31, 0xC9,
        // syscall
        0x0F, 0x05,
        // ret
        0xC3,
    ];

    /// Math add64 - Suma de dos valores 64-bit
    /// Entrada: rdi=a, rsi=b | Salida: rax=a+b
    pub const MATH_ADD64: &[u8] = &[
        // mov rax, rdi
        0x48, 0x89, 0xF8,
        // add rax, rsi
        0x48, 0x01, 0xF0,
        // ret
        0xC3,
    ];

    /// Math mul64 - Multiplicación de dos valores 64-bit
    /// Entrada: rdi=a, rsi=b | Salida: rax=a*b
    pub const MATH_MUL64: &[u8] = &[
        // mov rax, rdi
        0x48, 0x89, 0xF8,
        // imul rax, rsi
        0x48, 0x0F, 0xAF, 0xC6,
        // ret
        0xC3,
    ];

    /// Math sub64 - Resta de dos valores 64-bit
    /// Entrada: rdi=a, rsi=b | Salida: rax=a-b
    pub const MATH_SUB64: &[u8] = &[
        // mov rax, rdi
        0x48, 0x89, 0xF8,
        // sub rax, rsi
        0x48, 0x29, 0xF0,
        // ret
        0xC3,
    ];

    /// Math div64 - División de dos valores 64-bit
    /// Entrada: rdi=a, rsi=b | Salida: rax=a/b, rdx=a%b
    pub const MATH_DIV64: &[u8] = &[
        // mov rax, rdi
        0x48, 0x89, 0xF8,
        // xor rdx, rdx (clear high bits for division)
        0x48, 0x31, 0xD2,
        // div rsi (unsigned: rax=rax/rsi, rdx=rax%rsi)
        0x48, 0xF7, 0xF6,
        // ret
        0xC3,
    ];

    /// NOP sled - Para alineación o padding
    pub const NOP_SLED_16: &[u8] = &[
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
    ];
}

// ══════════════════════════════════════════
// FUNCIONES DE UTILIDAD
// ══════════════════════════════════════════

/// Convierte bytes a representación hexadecimal
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convierte string hexadecimal a bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.replace(" ", "").replace("\n", "").replace("\r", "");
    
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }
    
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i+2], 16)
                .map_err(|e| format!("Invalid hex at position {}: {}", i, e))
        })
        .collect()
}

/// Inyecta un valor en un template binario en el offset especificado
pub fn inject_parameter(template: &mut [u8], offset: usize, value: u64, size: usize) -> Result<(), String> {
    if offset + size > template.len() {
        return Err(format!("Parameter injection out of bounds: offset {} + size {} > template size {}", 
            offset, size, template.len()));
    }
    
    let value_bytes = value.to_le_bytes();
    template[offset..offset+size].copy_from_slice(&value_bytes[..size]);
    
    Ok(())
}

/// Crea una instancia compilada de un template
pub fn compile_template(
    template: &[u8],
    node_id: u32,
    template_name: &str,
    parameters: &[(usize, u64, usize)],  // (offset, value, size)
) -> Result<CompiledNode, String> {
    let mut compiled = template.to_vec();
    let mut param_values = HashMap::new();
    
    for (offset, value, size) in parameters {
        inject_parameter(&mut compiled, *offset, *value, *size)?;
        param_values.insert(format!("param_{}", offset), *value);
    }
    
    Ok(CompiledNode {
        node_id,
        template_name: template_name.to_string(),
        compiled_code: compiled,
        parameter_values: param_values,
    })
}

// ══════════════════════════════════════════
// INFORMACIÓN DE TEMPLATES
// ══════════════════════════════════════════

/// Obtiene información de todos los templates CPU disponibles
pub fn get_cpu_templates() -> Vec<BinaryTemplateInfo> {
    vec![
        BinaryTemplateInfo {
            name: "hello_world".to_string(),
            template_type: BinaryTemplateType::Cpu,
            architecture: "x86_64".to_string(),
            description: "Programa completo que imprime 'Hello World!' y termina".to_string(),
            size_bytes: cpu::HELLO_WORLD.len(),
            parameters: vec![],
            hex_representation: bytes_to_hex(cpu::HELLO_WORLD),
        },
        BinaryTemplateInfo {
            name: "syscall_write".to_string(),
            template_type: BinaryTemplateType::Cpu,
            architecture: "x86_64".to_string(),
            description: "Syscall write con parámetros inyectables".to_string(),
            size_bytes: cpu::SYSCALL_WRITE.len(),
            parameters: vec![
                TemplateParameter {
                    name: "fd".to_string(),
                    param_type: "uint32".to_string(),
                    offset: 10,
                    size: 4,
                    default: Some(1),
                    description: "File descriptor".to_string(),
                },
                TemplateParameter {
                    name: "buffer_addr".to_string(),
                    param_type: "uint64".to_string(),
                    offset: 17,
                    size: 8,
                    default: None,
                    description: "Buffer address".to_string(),
                },
                TemplateParameter {
                    name: "length".to_string(),
                    param_type: "uint32".to_string(),
                    offset: 27,
                    size: 4,
                    default: None,
                    description: "Length to write".to_string(),
                },
            ],
            hex_representation: bytes_to_hex(cpu::SYSCALL_WRITE),
        },
        BinaryTemplateInfo {
            name: "syscall_exit".to_string(),
            template_type: BinaryTemplateType::Cpu,
            architecture: "x86_64".to_string(),
            description: "Syscall exit con código de salida inyectable".to_string(),
            size_bytes: cpu::SYSCALL_EXIT.len(),
            parameters: vec![
                TemplateParameter {
                    name: "exit_code".to_string(),
                    param_type: "uint32".to_string(),
                    offset: 10,
                    size: 4,
                    default: Some(0),
                    description: "Exit code".to_string(),
                },
            ],
            hex_representation: bytes_to_hex(cpu::SYSCALL_EXIT),
        },
        BinaryTemplateInfo {
            name: "memory_alloc".to_string(),
            template_type: BinaryTemplateType::Cpu,
            architecture: "x86_64".to_string(),
            description: "Alocar memoria con mmap".to_string(),
            size_bytes: cpu::MEMORY_ALLOC.len(),
            parameters: vec![
                TemplateParameter {
                    name: "size".to_string(),
                    param_type: "uint64".to_string(),
                    offset: 12,
                    size: 8,
                    default: Some(4096),
                    description: "Size in bytes".to_string(),
                },
            ],
            hex_representation: bytes_to_hex(cpu::MEMORY_ALLOC),
        },
        BinaryTemplateInfo {
            name: "math_add64".to_string(),
            template_type: BinaryTemplateType::Cpu,
            architecture: "x86_64".to_string(),
            description: "Suma dos valores de 64 bits (rdi + rsi -> rax)".to_string(),
            size_bytes: cpu::MATH_ADD64.len(),
            parameters: vec![],
            hex_representation: bytes_to_hex(cpu::MATH_ADD64),
        },
        BinaryTemplateInfo {
            name: "math_mul64".to_string(),
            template_type: BinaryTemplateType::Cpu,
            architecture: "x86_64".to_string(),
            description: "Multiplica dos valores de 64 bits (rdi * rsi -> rax)".to_string(),
            size_bytes: cpu::MATH_MUL64.len(),
            parameters: vec![],
            hex_representation: bytes_to_hex(cpu::MATH_MUL64),
        },
        BinaryTemplateInfo {
            name: "math_sub64".to_string(),
            template_type: BinaryTemplateType::Cpu,
            architecture: "x86_64".to_string(),
            description: "Resta dos valores de 64 bits (rdi - rsi -> rax)".to_string(),
            size_bytes: cpu::MATH_SUB64.len(),
            parameters: vec![],
            hex_representation: bytes_to_hex(cpu::MATH_SUB64),
        },
        BinaryTemplateInfo {
            name: "math_div64".to_string(),
            template_type: BinaryTemplateType::Cpu,
            architecture: "x86_64".to_string(),
            description: "Divide dos valores de 64 bits (rdi / rsi -> rax, rdi % rsi -> rdx)".to_string(),
            size_bytes: cpu::MATH_DIV64.len(),
            parameters: vec![],
            hex_representation: bytes_to_hex(cpu::MATH_DIV64),
        },
    ]
}

// ══════════════════════════════════════════
// TESTS
// ══════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world_size() {
        assert_eq!(cpu::HELLO_WORLD.len(), 55);
    }

    #[test]
    fn test_bytes_to_hex() {
        let bytes = &[0x48, 0xC7, 0xC0, 0x01];
        assert_eq!(bytes_to_hex(bytes), "48 C7 C0 01");
    }

    #[test]
    fn test_hex_to_bytes() {
        let hex = "48 C7 C0 01";
        let bytes = hex_to_bytes(hex).unwrap();
        assert_eq!(bytes, vec![0x48, 0xC7, 0xC0, 0x01]);
    }

    #[test]
    fn test_inject_parameter() {
        let mut template = vec![0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];
        inject_parameter(&mut template, 2, 0x12345678, 4).unwrap();
        assert_eq!(template, vec![0x00, 0x00, 0x78, 0x56, 0x34, 0x12, 0x00]);
    }

    #[test]
    fn test_compile_template() {
        let node = compile_template(
            cpu::MATH_ADD64,
            1,
            "math_add64",
            &[],
        ).unwrap();
        
        assert_eq!(node.node_id, 1);
        assert_eq!(node.compiled_code, cpu::MATH_ADD64);
    }
}

