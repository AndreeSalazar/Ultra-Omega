// ═══════════════════════════════════════════════════════════════════════════════
// Zig: Funciones
// Sintaxis clara, tipos explícitos, error handling integrado
// ═══════════════════════════════════════════════════════════════════════════════

const std = @import("std");

// Función simple
fn suma(a: i32, b: i32) i32 {
    return a + b;
}

// Función con error handling
fn dividir(a: f64, b: f64) !f64 {
    if (b == 0) {
        return error.DivisionPorCero;
    }
    return a / b;
}

// Función con múltiples valores de retorno
fn dividirConResiduo(a: i32, b: i32) !struct { cociente: i32, residuo: i32 } {
    if (b == 0) {
        return error.DivisionPorCero;
    }
    return .{
        .cociente = @divTrunc(a, b),
        .residuo = @mod(a, b),
    };
}

// Función genérica
fn maximo(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

// Función que acepta función como parámetro
fn aplicar(fn_ptr: *const fn (i32) i32, valor: i32) i32 {
    return fn_ptr(valor);
}

fn cuadrado(x: i32) i32 {
    return x * x;
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    
    // Llamar función simple
    const resultado = suma(10, 20);
    try stdout.print("10 + 20 = {d}\n", .{resultado});
    
    // Llamar función con error handling
    const division = dividir(15.0, 3.0) catch |err| {
        try stdout.print("Error: {}\n", .{err});
        return;
    };
    try stdout.print("15 / 3 = {d}\n", .{division});
    
    // Múltiples valores de retorno
    const resultado_div = dividirConResiduo(17, 5) catch |err| {
        try stdout.print("Error: {}\n", .{err});
        return;
    };
    try stdout.print("17 / 5 = {} residuo {}\n", .{ resultado_div.cociente, resultado_div.residuo });
    
    // Función genérica
    const max_int = maximo(i32, 10, 20);
    const max_float = maximo(f64, 3.14, 2.71);
    try stdout.print("Max(10, 20) = {d}\n", .{max_int});
    try stdout.print("Max(3.14, 2.71) = {d}\n", .{max_float});
    
    // Función como parámetro
    const resultado_cuadrado = aplicar(cuadrado, 5);
    try stdout.print("cuadrado(5) = {d}\n", .{resultado_cuadrado});
}

