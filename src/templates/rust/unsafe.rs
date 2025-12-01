// ═══════════════════════════════════════
// Unsafe Rust
// Raw pointers, unsafe blocks, FFI
// ═══════════════════════════════════════

// ═══════════════════════════════════════
// RAW POINTERS
// ═══════════════════════════════════════
fn ejemplo_raw_pointers() {
    let mut x = 42;
    
    // Raw pointer inmutable
    let ptr: *const i32 = &x;
    
    // Raw pointer mutable
    let mut_ptr: *mut i32 = &mut x;
    
    unsafe {
        println!("Valor a través de *const: {}", *ptr);
        *mut_ptr = 100;
        println!("Valor modificado: {}", *mut_ptr);
    }
}

// ═══════════════════════════════════════
// UNSAFE FUNCTION
// ═══════════════════════════════════════
unsafe fn funcion_unsafe(ptr: *const i32) -> i32 {
    *ptr
}

// ═══════════════════════════════════════
// UNSAFE BLOCK
// ═══════════════════════════════════════
fn llamar_unsafe() {
    let valor = 42;
    let resultado = unsafe {
        funcion_unsafe(&valor)
    };
    println!("Resultado: {}", resultado);
}

// ═══════════════════════════════════════
// UNSAFE TRAIT
// ═══════════════════════════════════════
unsafe trait TraitUnsafe {
    fn metodo_unsafe(&self);
}

unsafe impl TraitUnsafe for i32 {
    fn metodo_unsafe(&self) {
        println!("Método unsafe para i32: {}", self);
    }
}

// ═══════════════════════════════════════
// MANIPULACIÓN DE MEMORIA
// ═══════════════════════════════════════
fn ejemplo_memoria() {
    let mut array = [1, 2, 3, 4, 5];
    
    unsafe {
        let ptr = array.as_mut_ptr();
        
        // Acceder a memoria directamente
        *ptr.add(0) = 10;
        *ptr.add(1) = 20;
        
        println!("Array modificado: {:?}", array);
    }
}

// ═══════════════════════════════════════
// UNSAFE CON STATIC MUTABLE
// ═══════════════════════════════════════
static mut CONTADOR: i32 = 0;

fn incrementar_contador() {
    unsafe {
        CONTADOR += 1;
    }
}

fn obtener_contador() -> i32 {
    unsafe {
        CONTADOR
    }
}

// ═══════════════════════════════════════
// UNSAFE CON UNION (FFI)
// ═══════════════════════════════════════
#[repr(C)]
union UnionEjemplo {
    entero: u32,
    flotante: f32,
}

fn ejemplo_union() {
    let mut union = UnionEjemplo { entero: 0x3F800000 };
    
    unsafe {
        println!("Como entero: 0x{:X}", union.entero);
        println!("Como flotante: {}", union.flotante);
    }
}

// ═══════════════════════════════════════
// UNSAFE CON EXTERNAL FUNCTIONS (FFI)
// ═══════════════════════════════════════
#[link(name = "c")]
extern "C" {
    fn abs(x: i32) -> i32;
}

fn llamar_funcion_c() {
    unsafe {
        let resultado = abs(-42);
        println!("abs(-42) = {}", resultado);
    }
}

// ═══════════════════════════════════════
// UNSAFE IMPL
// ═══════════════════════════════════════
struct Buffer {
    datos: Vec<u8>,
}

impl Buffer {
    fn new(tamaño: usize) -> Self {
        Buffer {
            datos: vec![0; tamaño],
        }
    }
    
    unsafe fn escribir_raw(&mut self, offset: usize, valor: u8) {
        if offset < self.datos.len() {
            *self.datos.as_mut_ptr().add(offset) = valor;
        }
    }
    
    unsafe fn leer_raw(&self, offset: usize) -> u8 {
        if offset < self.datos.len() {
            *self.datos.as_ptr().add(offset)
        } else {
            0
        }
    }
}

fn main() {
    println!("=== UNSAFE RUST ===\n");
    println!("⚠️  Unsafe permite bypass de las garantías de seguridad de Rust");
    println!("   Úsalo solo cuando sea absolutamente necesario\n");
    
    // ═══════════════════════════════════════
    // RAW POINTERS
    // ═══════════════════════════════════════
    println!("=== RAW POINTERS ===");
    ejemplo_raw_pointers();
    println!();
    
    // ═══════════════════════════════════════
    // UNSAFE FUNCTION
    // ═══════════════════════════════════════
    println!("=== UNSAFE FUNCTION ===");
    llamar_unsafe();
    println!();
    
    // ═══════════════════════════════════════
    // UNSAFE TRAIT
    // ═══════════════════════════════════════
    println!("=== UNSAFE TRAIT ===");
    let valor: i32 = 42;
    unsafe {
        valor.metodo_unsafe();
    }
    println!();
    
    // ═══════════════════════════════════════
    // MANIPULACIÓN DE MEMORIA
    // ═══════════════════════════════════════
    println!("=== MANIPULACIÓN DE MEMORIA ===");
    ejemplo_memoria();
    println!();
    
    // ═══════════════════════════════════════
    // STATIC MUTABLE
    // ═══════════════════════════════════════
    println!("=== STATIC MUTABLE ===");
    incrementar_contador();
    incrementar_contador();
    incrementar_contador();
    println!("Contador: {}", obtener_contador());
    println!();
    
    // ═══════════════════════════════════════
    // UNION
    // ═══════════════════════════════════════
    println!("=== UNION ===");
    ejemplo_union();
    println!();
    
    // ═══════════════════════════════════════
    // EXTERNAL FUNCTIONS
    // ═══════════════════════════════════════
    println!("=== EXTERNAL FUNCTIONS (FFI) ===");
    llamar_funcion_c();
    println!();
    
    // ═══════════════════════════════════════
    // BUFFER UNSAFE
    // ═══════════════════════════════════════
    println!("=== BUFFER UNSAFE ===");
    let mut buffer = Buffer::new(10);
    
    unsafe {
        buffer.escribir_raw(0, 100);
        buffer.escribir_raw(1, 200);
        buffer.escribir_raw(2, 300);
        
        println!("Leído [0]: {}", buffer.leer_raw(0));
        println!("Leído [1]: {}", buffer.leer_raw(1));
        println!("Leído [2]: {}", buffer.leer_raw(2));
    }
    
    println!("\n⚠️  RECUERDA: Unsafe no significa 'no verificado'");
    println!("   Significa que TÚ eres responsable de la seguridad");
}

