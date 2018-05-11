//Crate attributes for features disabled by default
#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods, const_fn)]

//Disables certain optimisations that can normally occur, but cant in this low level context
#![no_builtins]

//Disables the standard library. Links against libcore instead which is far more limited.
#![no_std]

//pub mod lang_items;
//use core::ptr::{read_volatile, write_volatile};

//External libraries
/// Tiny libc implementation. Provides memset and memcpy, used for clearing the IO buffer.
extern crate rlibc; 

/// This is a very small library that provides the volatile wrapper for structures
/// when the compiler sees a write or a read that has no effect to the rest of the program, it may optimise that read or write away.
/// in C you can mark things with volatile to ensure this doesnt happen, for example
/// volatile int a = 22;
/// 
/// In Rust however, the core::ptr::read_volatile and write_volatile methods are implemented for pointers (*mut T, *const T)
/// however its easy to forget to use them by accident and hard to find them if they're missing so using the volatile wrappers for the types ala C can save a headache
extern crate volatile; 

/// My modules
mod common;
mod gpio;
mod prettyprinter;
mod stdio;
mod timer;
mod uart;

///Imports
use core::fmt::Write;
use core::sync::atomic::AtomicBool;
use gpio::*;
use prettyprinter::*;
use stdio::{stdin, Stdio};
use timer::spin_sleep_millis;
use uart::Uart;

///Error handling personality, the behaviour of theerror handling, which is Abort for this, do no stack unwind.
/// more info <https://doc.rust-lang.org/1.4.0/book/no-stdlib.html>
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

///Code executed on panics
/// more info <https://doc.rust-lang.org/1.4.0/book/no-stdlib.html>
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

///Main function for the kernel
#[no_mangle]
pub unsafe extern "C" fn kmain() {
    
    let mut stdin = stdin().unwrap(); //Get stdin handle
    let mut uart = Uart::new().with_auto_flow_control(); //Use builder pattern to create Uart device
    let mut led1 = Gpio::new(20).as_output(); //Create GPIO device on GPIO20 and set it as an output
    let mut led2 = Gpio::new(21).as_output();
    led1.set(); //Turn on the LED
    led2.set();
    stdin.clear(); //zero out the stdin buffer and reset its cursor
    loop { //Infinite loop
        if uart.has_byte() { // If the Uart device has received a transmission
            let byte = uart.read_byte(); // Read the data it was sent
            stdin
                .push(byte)
                .expect("Error pushing to stdin, probably full"); // Add this input from the user to the stdin buffer
            uart.write_byte(byte); // Write the users input back to them (otherwise they cant see their own keypresses)
            if !test_for_special_char(byte, &mut stdin, &mut uart) { //Check for ctrl-c basically
                evaluate_stdin_buffer(&mut stdin, &mut led1, &mut led2, &mut uart); //Compare the text in stdin to the preset commands
            }
        }
    }
}
///This function checks for the ctrl-c escape key, which has an ASCII value of 3.Alt
/// If it finds it, it clears the screen and clears stdin
fn test_for_special_char(byte: u8, stdin: &mut Stdio, uart: &mut Uart) -> bool {
    match byte {
        //Escape key
        3 => {
            stdin.clear();
            uart.clr();
            true
        }
        _ => false,
    }
}
///This function compares the contents of stdin with preset text commands and executes them if it finds them
fn evaluate_stdin_buffer(
    stdin: &mut Stdio,
    led1: &mut Gpio<Output>,
    led2: &mut Gpio<Output>,
    uart: &mut Uart,
) {
    match stdin.as_str().expect("Error on line 81") {
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

///This function turns on LED's in a predetermined pattern
fn prog_1(stdin: &mut Stdio, uart: &mut Uart, led1: &mut Gpio<Output>, led2: &mut Gpio<Output>) {
    uart.clr();
    stdin.clear();
    uart.set_fg_colour(FG_GREEN); //Sets foreground text to green
    uart.write_str("Press k to end the program").unwrap();
    let mut i = 60;
    while i > 0 {
        if uart.has_byte() {
            if uart.read_byte() == 'k' as u8 {
                break;
            }
        }
        spin_sleep_millis(1000); //Use a spinlock style sleep for 1 second, this is where interrupts would have helped
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

///Display some help text
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
