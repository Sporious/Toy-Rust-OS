#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]
//pub mod lang_items;
//use core::ptr::{read_volatile, write_volatile};

extern crate rlibc;
extern crate volatile;
mod common;
mod timer;
use common::{GPIO_CLR0, GPIO_FSEL1, GPIO_SET0};

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 600) {
        unsafe {
            asm!("nop" :::: "volatile");
        }
    }
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
#[no_mangle]
pub unsafe extern "C" fn kmain() {
    GPIO_FSEL1.write_volatile(0b001 << 18);

    loop {
        GPIO_SET0.write_volatile(0b1 << 16);
        spin_sleep_ms(1000);
        GPIO_CLR0.write_volatile(0b1 << 16);
        spin_sleep_ms(1000);
    }
}
