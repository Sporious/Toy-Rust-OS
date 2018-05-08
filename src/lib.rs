#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]
//pub mod lang_items;
//use core::ptr::{read_volatile, write_volatile};

extern crate rlibc;
extern crate volatile;
mod common;
mod gpio;
mod timer;
mod uart;
use gpio::*;
use timer::spin_sleep_millis;
use uart::Uart;

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
  
  /*
    let mut pin = Gpio::new(16).as_output();
    loop {
        spin_sleep_millis(1000);
        pin.set();
        spin_sleep_millis(1000);
        pin.clear();
    }
    */

    let mut uart = Uart::new()    ;
    loop {
        spin_sleep_millis(5000);
        uart.write_byte('a' as u8);
    }
}
