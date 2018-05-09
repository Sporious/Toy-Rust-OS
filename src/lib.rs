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
    let mut pin = Gpio::new(16).as_output();
    let mut pin_status = false;
    loop {
        if uart.has_byte( ) {
            let byte = uart.read_byte();
            if byte >= 97 && byte<=122 {
                stdin.push(byte).unwrap();
            } 

            if byte == 't' as u8 {
                if pin_status {
                    pin.clear();
                    pin_status = false;
                    uart.write_str("It's on!\r\n").unwrap();
                }
                else {
                    pin.set();
                    pin_status = true;
                    uart.write_str("It's off!\r\n").unwrap();
                }
            }
        }

        
    }
}
