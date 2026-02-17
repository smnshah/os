use core::arch::asm;
use core::arch::x86_64::__cpuid;
use core::ptr::addr_of;

use super::gdt::{Gdt, TSS_SELECTOR};
use super::tss::Tss;

const APIC_ID_SHIFT: u32 = 24;
const APIC_ID_MASK: u32 = 0xFF;

static mut CPU: Cpu = Cpu::new();

pub struct Cpu {
    gdt: Gdt,
    tss: Tss,
    df_stack: [u8; 4096],
}

impl Cpu {
    const fn new() -> Self {
        Self {
            gdt: Gdt::new(),
            tss: Tss::new(),
            df_stack: [0; 4096],
        }
    }
}

pub fn init() {
    unsafe {
        let cpu = &mut *(&raw mut CPU);
        let df_stack_top = (addr_of!(cpu.df_stack)) as u64 + (cpu.df_stack.len()) as u64;

        cpu.tss.init(df_stack_top);

        let tss_addr = addr_of!(cpu.tss) as u64;
        cpu.gdt.init_tss(tss_addr);

        cpu.gdt.load();
        Tss::load(TSS_SELECTOR);
    }
}

pub fn switch_stack(stack_top: u64, target: extern "C" fn() -> !) -> ! {
    unsafe {
        asm!(
            "mov rsp, {}",
            "call {}",
            "ud2",
            in(reg) stack_top,
            in(reg) target,
            options(noreturn),
        );
    }
}

/// Uses CPUID leaf 1 initial APIC ID (bootstrap strategy for now, will adjust to more
/// sophisticated method as needed).
pub fn current_core_id() -> u16 {
    let cpu_id = unsafe { __cpuid(1) };
    ((cpu_id.ebx >> APIC_ID_SHIFT) & APIC_ID_MASK) as u16
}
