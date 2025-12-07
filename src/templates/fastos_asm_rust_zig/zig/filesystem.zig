// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Sistema de Archivos (Zig)
// Sistema de archivos básico implementado en Zig
// ═══════════════════════════════════════════════════════════════════════════════

const std = @import("std");

// ═══════════════════════════════════════════════════════════════════════════════
// ESTRUCTURA DE ARCHIVO BÁSICA
// ═══════════════════════════════════════════════════════════════════════════════
pub const File = struct {
    name: []const u8,
    data: []u8,
    size: usize,
    
    pub fn init(name: []const u8, data: []u8) File {
        return File{
            .name = name,
            .data = data,
            .size = data.len,
        };
    }
    
    pub fn read(self: *const File) []const u8 {
        return self.data;
    }
    
    pub fn write(self: *File, new_data: []const u8) void {
        if (new_data.len <= self.data.len) {
            std.mem.copy(u8, self.data, new_data);
            self.size = new_data.len;
        }
    }
};

// ═══════════════════════════════════════════════════════════════════════════════
// SISTEMA DE ARCHIVOS SIMPLE
// ═══════════════════════════════════════════════════════════════════════════════
pub const FileSystem = struct {
    files: []File,
    
    pub fn init(allocator: std.mem.Allocator) FileSystem {
        return FileSystem{
            .files = &[_]File{},
        };
    }
    
    pub fn create_file(self: *FileSystem, name: []const u8, data: []u8) !File {
        const file = File.init(name, data);
        // Aquí se agregaría a la lista de archivos
        return file;
    }
    
    pub fn find_file(self: *FileSystem, name: []const u8) ?*File {
        for (self.files) |*file| {
            if (std.mem.eql(u8, file.name, name)) {
                return file;
            }
        }
        return null;
    }
};

