/// Documentation for the hardware for the timer can be found on page 172 of the Broadcom manual

use common::TIMER_BASE;
///Use the volatile wrappers ReadOnly and Volatile
///They're functionaly identical and really don't do much, they just prevent accidental
///non-volatile reads/writes
use volatile::{ReadOnly, Volatile};

///This structure uses [repr(C)].
///This means that the compiler is NOT free to reorder the fields of this structure for alignment
///reasons, its very important the layout is exactly as ive defined it.
///
///This is the Register table found on page 172 of the Broadcom manual, figure 12.1
#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    //System Timer Control/Status
    CS: Volatile<u32>, 
    
    //System Timer Counter Lower 32 bits
    CLO: ReadOnly<u32>, 
    
    // System Timer Counter Higher 32 bits
    CHI: ReadOnly<u32>,

    // 4x System Timer Compare (for timer interrupts)
    COMPARE: [Volatile<u32>; 4],
}

///My system timer structure which is just a wrapper for the static pointer the the registers,
///static means the structure is guarenteed to be alive for the whole duration of the program
///(memory doesnt just vanish)
pub struct SystemTimer {
    registers: &'static mut Registers,
}

impl SystemTimer {
    ///Constructor, builds this struct from casting the const address as a pointer then wrapping it
    pub fn new() -> SystemTimer {
        
        SystemTimer {
            //Casts the TIMER_BASE address defined in common.rs as a Registers struct
            //Dereferencing a raw pointer cannot be validated by the compiler so needs an unsafe block
            registers: unsafe { &mut *(TIMER_BASE as *mut Registers) },
        }
    }
    ///Reads elapsed time in microseconds
    pub fn read(&self) -> u64 {
        //Get the lower 32 bits from CLO
        let lower: u64 = self.registers.CLO.read() as u64;

        //Get the higher 32 bits from CHI
        let higher: u64 = self.registers.CHI.read() as u64;

        //Bitshift the higher bits to the left 32 and use binary OR to combine them
        (higher << 32) | lower
    }
}

/// Gets the current time in microseconds
pub fn current_time() -> u64 {
    SystemTimer::new().read()
}

/// Gets the current time in milliseconds
pub fn current_time_ms() -> u64 {
    current_time() / 1000
}

///Locks the thread for so many microseconds
pub fn spin_sleep_micros(micros: u64) {
    let start = current_time();
    while current_time() < start + micros {}
}

///Locks the thread for so many milliseconds
pub fn spin_sleep_millis(i_millis: u64) {
    spin_sleep_micros(i_millis * 1000)
}
