//! Common kernel debug print support

extern "C" {
    pub fn kprintf(fmt: *const u8) -> i32;
    // pub fn printk(fmt: *const u8) -> i32;
}

// Common printing macro
macro_rules! print {
    ($e:expr) => (
        unsafe { ::print::kprintf(concat!($e, "\0").as_ptr()); }
        // unsafe { ::print::printk(concat!($e, "\0").as_ptr()); }
    )
}

// Common printing macro including newline
macro_rules! println {
    ($e:expr) => (print!(concat!($e, "\n")))
}
