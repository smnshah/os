#[macro_export]
macro_rules! print {
    ($fmt:expr $(, $arg:tt)*) => {{
        use ::core::fmt::Write;
        let mut serial = $crate::arch::x86_64::serial::Serial;
        let _ = ::core::write!(serial, $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    ($fmt:expr $(, $arg:tt)*) => {{
        $crate::print!(concat!($fmt, "\n") $(, $arg)*)
    }};

    () => {{
        $crate::print!("\n")
    }};
}
