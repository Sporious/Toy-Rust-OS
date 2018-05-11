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
pub unsafe extern "C" fn kmain() {
    let mut stdin = stdin().unwrap();
    let mut uart = Uart::new().with_auto_flow_control();
    let mut led1 = Gpio::new(20).as_output();
    let mut led2 = Gpio::new(21).as_output();
    led1.set();
    led2.set();
    stdin.clear();
    loop {
        if uart.has_byte() {
            let byte = uart.read_byte();
            stdin
                .push(byte)
                .expect("Error pushing to stdin, probably full");
            uart.write_byte(byte);
            if !test_for_special_char(byte, &mut stdin, &mut uart) {
                evaluate_stdin_buffer(&mut stdin, &mut led1, &mut led2, &mut uart);
            }
        }
    }
}

fn test_for_special_char(byte: u8, stdin: &mut Stdio, uart: &mut Uart) -> bool {
    match byte {
        //Escape
        3 => {
            stdin.clear();
            uart.clr();
            true
        }
        _ => false,
    }
}

fn evaluate_stdin_buffer(
    stdin: &mut Stdio,
    led1: &mut Gpio<Output>,
    led2: &mut Gpio<Output>,
    uart: &mut Uart,
) {
    match stdin.as_str().expect("Error on like 81") {
        "led1 on" => {
            led1.set();
            stdin.clear();
            uart.clr();
        }
        "led1 off" => {
            led1.clear();
            stdin.clear();
            uart.clr();
        }
        "led2 on" => {
            led2.set();
            stdin.clear();
            uart.clr();
        }
        "led2 off" => {
            led2.clear();
            stdin.clear();
            uart.clr();
        }
        "prog1 on" => {
            stdin.clear();
            prog_1(stdin, uart, led1, led2);
        }
        "help" => {
            stdin.clear();
            help(uart)
        }
        _ => {}
    }
}

fn prog_1(stdin: &mut Stdio, uart: &mut Uart, led1: &mut Gpio<Output>, led2: &mut Gpio<Output>) {
    uart.clr();
    stdin.clear();
    uart.set_fg_colour(FG_GREEN);
    uart.write_str("Press k to end the program").unwrap();
    let mut i = 60;
    while i > 0 {
        if uart.has_byte() {
            if uart.read_byte() == 'k' as u8 {
                break;
            }
        }
        spin_sleep_millis(1000);
        led1.set();
        spin_sleep_millis(1000);
        led2.set();
        spin_sleep_millis(1000);
        led1.clear();
        spin_sleep_millis(1000);
        led2.clear();
        i -= 4;
    }
    led1.clear();
    led2.clear();
    uart.clr();
    stdin.clear();
    uart.set_fg_colour(FG_CLEAR);
}

fn help(uart: &mut Uart) {
    uart.clr();
    uart.set_bg_colour(BG_BLUE);
    uart.set_fg_colour(FG_WHITE);
    uart.write_str(
        "\r
\r\n
Help\r\n
led1 on : turns on LED1\r
led2 off : turns off LED2\r
prog1 on : turns on program 1\r
help: shows this message\r
pressing ctrl+c will clear the input buffer \r
    ",
    ).expect("error printing string");
    uart.set_bg_colour(BG_CLEAR);
    uart.set_fg_colour(FG_CLEAR);
}
