use core::arch::naked_asm;
use super::idt::Idt;
use super::mmu::read_cr2;

const DIVIDE_BY_ZERO_VEC: usize = 0;
const DOUBLE_FAULT_VEC: usize = 8;
const GENERAL_PROTECTION_FAULT_VEC: usize = 13;
const PAGE_FAULT_VEC: usize = 14;

const NUM_GP_REGS: usize = 15;
const SAVED_REGS_SIZE: usize = NUM_GP_REGS * 8;

const PF_PRESENT: u64 = 1 << 0;
const PF_WRITE: u64 = 1 << 1;
const PF_USER: u64 = 1 << 2;

const GPF_EXTERNAL: u64 = 1 << 0;
const GPF_TABLE_MASK: u64 = 0b11 << 1;
const GPF_SELECTOR_SHIFT: usize = 3;

#[repr(C)]
pub struct InterruptStackFrame {
    err_code: u64,
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

macro_rules! exception_stub {
    ($name:ident, $handler:ident, no_error_code) => {
        exception_stub!(@impl $name, $handler, "push 0",);
    };


    ($name:ident, $handler:ident, has_error_code) => {
        exception_stub!(@impl $name, $handler,);
    };

    (@impl $name:ident, $handler:ident, $($preamble:tt)*) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            naked_asm!(
                $($preamble)*
                "push rax",
                "push rbx",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",
                "push rbp",
                "push r8",
                "push r9",
                "push r10",
                "push r11",
                "push r12",
                "push r13",
                "push r14",
                "push r15",
                "lea rdi, [rsp + {offset}]",
                "call {handler}",
                "pop r15",
                "pop r14",
                "pop r13",
                "pop r12",
                "pop r11",
                "pop r10",
                "pop r9",
                "pop r8",
                "pop rbp",
                "pop rdi",
                "pop rsi",
                "pop rdx",
                "pop rcx",
                "pop rbx",
                "pop rax",
                "add rsp, 8",
                "iretq",
                offset = const SAVED_REGS_SIZE,
                handler = sym $handler,
            );
        }
    };
}

exception_stub!(divide_by_zero_stub, divide_by_zero_handler, no_error_code);
exception_stub!(double_fault_stub, double_fault_handler, has_error_code);
exception_stub!(general_protection_fault_stub, general_protection_fault_handler, has_error_code);
exception_stub!(page_fault_stub, page_fault_handler, has_error_code);

pub fn register_handlers(idt: &mut Idt) {
    idt.set_handler(DIVIDE_BY_ZERO_VEC, divide_by_zero_stub);
    idt.set_handler(DOUBLE_FAULT_VEC, double_fault_stub);
    idt.set_handler(GENERAL_PROTECTION_FAULT_VEC, general_protection_fault_stub);
    idt.set_handler(PAGE_FAULT_VEC, page_fault_stub);

    idt.set_ist(DOUBLE_FAULT_VEC, 1);
}

extern "C" fn divide_by_zero_handler(frame: &InterruptStackFrame) {
    panic!("Divide by zero at {:#x}", frame.rip);
}

extern "C" fn double_fault_handler(frame: &InterruptStackFrame) {
    panic!("Double fault at {:#x}
        RSP: {:#x}"
        , frame.rip, frame.rsp
    );
}

extern "C" fn page_fault_handler(frame: &InterruptStackFrame) {
    let fault_addr = read_cr2();
    let rip = frame.rip;
    let err_code = frame.err_code;
    let rsp = frame.rsp;
    let present = (err_code & PF_PRESENT) != 0;
    let write = (err_code & PF_WRITE) != 0;
    let user = (err_code & PF_USER) != 0;
    
    panic!("Page fault at {rip:#x}
        Address: {fault_addr:#x}
        Error: {err_code:#b} (present={present}, write={write}, user={user}) 
        RSP: {rsp:#x}"
    );
}

extern "C" fn general_protection_fault_handler(frame: &InterruptStackFrame) {
    let rip = frame.rip;
    let err_code = frame.err_code;
    let rsp = frame.rsp;
    let external = (err_code & GPF_EXTERNAL) != 0;
    let table = (err_code & GPF_TABLE_MASK) >> 1;
    let selector = err_code >> GPF_SELECTOR_SHIFT;
    panic!("General protection fault at {rip:#x}
        Error: {err_code:#b} (external={external}, table={table}, selector={selector})
        RSP: {rsp:#x}"
    );
}
