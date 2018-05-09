#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods, const_fn)]
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
mod stdin;
use gpio::*;
use timer::spin_sleep_millis;
use uart::Uart;
use stdin::stdin;
use core::fmt::Write;


#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {

    let mut uart = Uart::new();
    let mut stdin = stdin().unwrap();
    loop {
        if uart.has_byte( ) {
            let byte = uart.read_byte();
            if byte >= 97 && byte<=122 {
                stdin.push(byte).unwrap();
            } 
        }

        if (&stdin).into_iter().count() >= 5 {
            uart.write_str("Hit 5, resetting \r\n");
            stdin.clear();
        }
    }
}
