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


#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {

    let mut stdin = stdin();
    stdin.push('a' as u8).unwrap();
    stdin.push('b' as u8).unwrap();
    stdin.push('c' as u8).unwrap();
    let mut uart = Uart::new();
    /*
    loop {
        
        if uart.has_byte() {
            let c = uart.read_byte();
            uart.write_byte(c);
        }
    }
    */

    loop {
        for c in &stdin {
            uart.write_byte(*c);
        }
        uart.write_byte('\r' as u8);
        uart.write_byte('\n' as u8);
    }
}
