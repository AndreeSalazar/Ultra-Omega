// ═══════════════════════════════════════════════════════════════════════════════
// Zig: Variables y Tipos
// Tipos explícitos, inferencia opcional, sin NULL por defecto
// ═══════════════════════════════════════════════════════════════════════════════

const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    
    // Constantes (inmutables)
    const pi: f64 = 3.14159;
    const nombre = "Zig"; // Inferencia de tipo
    
    // Variables (mutables)
    var contador: i32 = 0;
    var temperatura: f32 = 25.5;
    
    // Tipos numéricos explícitos
    const entero: i32 = 42;
    const flotante: f64 = 3.14;
    const booleano: bool = true;
    
    // Arrays
    var numeros = [_]i32{ 1, 2, 3, 4, 5 };
    const texto = "Hola, Mundo!";
    
    // Punteros opcionales (nullables)
    var valor: ?i32 = null;
    valor = 100;
    
    // Result (error handling)
    const resultado: !i32 = 42; // Result<ErrorSet, i32>
    
    try stdout.print("Pi: {d}\n", .{pi});
    try stdout.print("Nombre: {s}\n", .{nombre});
    try stdout.print("Contador: {d}\n", .{contador});
    try stdout.print("Temperatura: {d}\n", .{temperatura});
    try stdout.print("Entero: {d}\n", .{entero});
    try stdout.print("Flotante: {d}\n", .{flotante});
    try stdout.print("Booleano: {}\n", .{booleano});
    try stdout.print("Array: {any}\n", .{numeros});
    try stdout.print("Texto: {s}\n", .{texto});
    
    if (valor) |v| {
        try stdout.print("Valor: {d}\n", .{v});
    } else {
        try stdout.print("Valor es null\n", .{});
    }
}

