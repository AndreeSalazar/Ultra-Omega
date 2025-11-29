// ═══════════════════════════════════════════════════════════════════════════════
// FastOS 64-bit: Library Root
// Módulo principal que exporta todos los componentes
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]
#![feature(abi_x86_interrupt)]

// ═══════════════════════════════════════════════════════════════════════════════
// MÓDULOS
// ═══════════════════════════════════════════════════════════════════════════════
pub mod kernel_main;
pub mod ports;
pub mod interrupts;
pub mod memory;
pub mod drivers;
pub mod ffi;

// ═══════════════════════════════════════════════════════════════════════════════
// RE-EXPORT
// ═══════════════════════════════════════════════════════════════════════════════
pub use kernel_main::kernel_main_rust;
pub use ports::{Port, ports};
pub use interrupts::{InterruptFrame, interrupt_handler_rust, irq_handler_rust};
pub use memory::{BumpAllocator, MemoryMap};
pub use drivers::{VgaDriver, KeyboardDriver, TimerDriver, SerialDriver};
pub use ffi::*;

