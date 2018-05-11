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
use stdio::{stdin, Stdio};
use timer::spin_sleep_millis;
use uart::Uart;

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(args: core::fmt::Arguments, _: &(&'static str, u32)) -> ! {
    let mut uart = Uart::new();
    loop {
        uart.set_bg_colour(BG_RED);
        uart.set_fg_colour(FG_YELLOW);
        uart.write_fmt(args).unwrap();
        spin_sleep_millis(1000);
        uart.clr();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() 
{
    let mut stdin = stdin().unwrap();
    let mut uart = Uart::new();
    let mut led1 = Gpio::new(20).as_output();
    let mut led2 = Gpio::new(21).as_output();
    loop {
        if uart.has_byte() {
            let byte = uart.read_byte();
            stdin.push(byte).expect("Stdin full");
            uart.write_byte(byte);
            if !test_for_special_char(byte, &mut stdin, &mut uart) {
                evaluate_stdin_buffer(&mut stdin, &mut led1, &mut led2);
            }
        }
    }
}

fn test_for_special_char(byte: u8, stdin: &mut Stdio, uart: &mut Uart) -> bool {
    if byte == 3 {
        stdin.clear();
        uart.clr();
        true
    }
    else {
        false
    }

}

fn evaluate_stdin_buffer( stdin: &mut Stdio, led1: &mut Gpio<Output>, led2: &mut Gpio<Output>) {
    
    match stdin.as_str() {
        "led1 on" => led1.set(),
        "led1 off" => led1.clear(),
        "led2 on" => led1.set(),
        "led2 off" => led2.clear(),
        _ => {}
    }
}
