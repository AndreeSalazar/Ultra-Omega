// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Scheduler de Procesos (Zig)
// Scheduler simple para gestión de procesos
// ═══════════════════════════════════════════════════════════════════════════════

const std = @import("std");

// ═══════════════════════════════════════════════════════════════════════════════
// PROCESO BÁSICO
// ═══════════════════════════════════════════════════════════════════════════════
pub const Process = struct {
    pid: u32,
    name: []const u8,
    state: ProcessState,
    
    pub const ProcessState = enum {
        Ready,
        Running,
        Blocked,
        Terminated,
    };
    
    pub fn init(pid: u32, name: []const u8) Process {
        return Process{
            .pid = pid,
            .name = name,
            .state = .Ready,
        };
    }
};

// ═══════════════════════════════════════════════════════════════════════════════
// SCHEDULER SIMPLE (Round Robin)
// ═══════════════════════════════════════════════════════════════════════════════
pub const Scheduler = struct {
    processes: []Process,
    current: usize,
    
    pub fn init(allocator: std.mem.Allocator) Scheduler {
        return Scheduler{
            .processes = &[_]Process{},
            .current = 0,
        };
    }
    
    pub fn add_process(self: *Scheduler, process: Process) void {
        _ = self;
        _ = process;
        // Aquí se agregaría el proceso a la lista
    }
    
    pub fn schedule(self: *Scheduler) ?*Process {
        if (self.processes.len == 0) {
            return null;
        }
        
        const proc = &self.processes[self.current];
        self.current = (self.current + 1) % self.processes.len;
        return proc;
    }
};

