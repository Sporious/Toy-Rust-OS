#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods, const_fn)]
#![no_builtins]
#![no_std]

//pub mod lang_items;
//use core::ptr::{read_volatile, write_volatile};

extern crate rlibc;
extern crate volatile;
mod common;
mod gpio;
mod prettyprinter;
mod stdio;
mod timer;
mod uart;
use core::fmt::Write;
use core::sync::atomic::AtomicBool;
use gpio::*;
use prettyprinter::*;
use stdio::stdout;
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
    let mut uart = Uart::new();

    /*
    loop {
        match stdout() {
            Ok(mut stdout) => match stdout.write_char('c') {
                Ok(_) => {}
                Err(_) => loop {
                    uart.write_byte(7 as u8);
                },
            },
            _ => loop {
                uart.write_byte(6 as u8);
            },
        }

        for i in match stdout() {
            Ok(mut stdout) => stdout,
            _ => loop {
                uart.write_byte(8 as u8);
            },
        }.into_iter()
        {
            uart.write_byte(*i);
        }
    }
    */
    let strs = [
        "Im back",
        "cus im back",
        "snap out of it now",
        "we're out of the loop",
        "its time to move on",
        "its all or nothing now",
    ];
    let mut idx = 0;
    let mut pin = Gpio::new(21).as_output();
    let mut other_pin = Gpio::new(20).as_output();
    let mut on = false;
    loop {
        uart.set_fg_colour(FG_RED);
        uart.set_bg_colour(BG_GREEN);
        if idx == strs.len() {
            idx = 0
        }

        uart.write_str(strs[idx]);
        idx += 1;

        uart.set_bg_colour(BG_CLEAR);
        uart.clr();

        if uart.has_byte() {
            let byte = uart.read_byte();

            if byte == 't' as u8 {
                if !on {
                    pin.set();
                    on = true;
                } else {
                    pin.clear();
                    on = false;
                }
            } else if byte == 'p' as u8 {
                let mut i = 0;
                'w: while i <= 60 {
                    if !on {
                        pin.set();
                        other_pin.clear();
                        on = true;
                    } else {
                        pin.clear();
                        other_pin.set();
                        on = false;
                    }
                    if uart.has_byte() {
                        if uart.read_byte() == 'b' as u8 {
                            break 'w;
                        }
                    }
                    spin_sleep_millis(1000);
                    i += 1;
                }
                on = false;
                pin.clear();
                other_pin.clear();
            }
        }
    }
}
