//! Minimal 16550 UART driver for early kernel serial output (COM1).
//!
//! Provides polled serial I/O for debugging before higher-level subsystems
//! (framebuffer, logging) are initialized. Assumes a 16550-compatible UART
//! at the standard COM1 base address (0x3F8).
//!
//! This driver uses x86_64 port I/O instructions.

use core::arch::asm;

const COM1_BASE: u16 = 0x3f8;

// UART register offsets from base
const DATA: u16 = 0; // Receive/transmit buffer or DLL
const IER: u16 = 1; // Interrupt Enable or DLM
const FCR: u16 = 2; // FIFO control
const LCR: u16 = 3; // Line control
const MCR: u16 = 4; // Modem control
const LSR: u16 = 5; // Line status

/// Initialize COM1 for 115200 baud, 8N1 (8 data bits, no parity, 1 stop bit).
///
/// See: <https://wiki.osdev.org/Serial_Ports>
pub fn init() {
    unsafe { 
        outb(COM1_BASE + IER, 0x00);    // Disable interrupts
        outb(COM1_BASE + LCR, 0x80);    // Enable DLAB
        outb(COM1_BASE + DATA, 0x01);   // Divisor low byte (115200 baud)
        outb(COM1_BASE + IER, 0x00);    // Divisor high byte
        outb(COM1_BASE + LCR, 0x03);    // 8N1, disable DLAB
        outb(COM1_BASE + FCR, 0xc7);    // Enable + clear FIFOs
        outb(COM1_BASE + MCR, 0x0b);    // Enable DTR, RTS, OUT2
    }
}

pub fn write_byte(value: u8) {
    // Wait for transmit buffer empty (bit 5 of LSR)
    loop {
        let in_byte = unsafe { inb(COM1_BASE + LSR) };
        if in_byte & 0x20 != 0 { break; }
    }
    unsafe { outb(COM1_BASE, value); }
}

pub fn write_str(string: &[u8]) {
    for &byte in string {
        write_byte(byte);
    }
}

#[inline]
unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    
    value
}