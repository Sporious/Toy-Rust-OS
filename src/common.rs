pub const GPIO_BASE: usize = 0x3F000000 + 0x200000;

pub const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
pub const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
pub const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;
pub const IO_BASE: usize = 0x3F000000;
pub const TIMER_BASE: usize = IO_BASE + 0x3000;
