use core::arch::naked_asm;

const NUM_GP_REGS: usize = 15;
const SAVED_REGS_SIZE: usize = NUM_GP_REGS * 8;

#[repr(C)]
pub struct InterruptStackFrame {
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

macro_rules! exception_stub {
    ($name:ident, $handler:ident) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            naked_asm!(
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
                "iretq",
                offset = const SAVED_REGS_SIZE,
                handler = sym $handler,
            );
        }
    };
}

exception_stub!(divide_by_zero_stub, divide_by_zero_handler);

extern "C" fn divide_by_zero_handler(frame: &InterruptStackFrame) {
    panic!("Divide by zero at {:#x}", frame.rip);
}
