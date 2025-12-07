// ═══════════════════════════════════════════════════════════════════════════════
// Zig: Hello World
// Lenguaje de sistemas moderno y seguro
// ═══════════════════════════════════════════════════════════════════════════════

const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Hola desde Zig en Ultra Omega!\n", .{});
}

