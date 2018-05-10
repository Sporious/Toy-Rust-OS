/*
    Documentation for the hardware for the timer can be found on page 172 of the Broadcom
 */

use common::TIMER_BASE;
use volatile::{ReadOnly, Volatile};

#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    CS: Volatile<u32>,
    CLO: ReadOnly<u32>,
    CHI: ReadOnly<u32>,
    COMPARE: [Volatile<u32>; 4],
}

pub struct SystemTimer {
    registers: &'static mut Registers,
}

impl SystemTimer {
    pub fn new() -> SystemTimer {
        SystemTimer {
            registers: unsafe { &mut *(TIMER_BASE as *mut Registers) },
        }
    }
    pub fn read(&self) -> u64 {
        let lower: u64 = self.registers.CLO.read() as u64;
        let higher: u64 = self.registers.CHI.read() as u64;
        (higher << 32) | lower
    }
}
pub fn current_time() -> u64 {
    SystemTimer::new().read()
}

pub fn current_time_ms() -> u64 {
    current_time()
}

pub fn spin_sleep_micros(micros: u64) {
    let start = current_time();
    while current_time() < start + micros {}
}

pub fn spin_sleep_millis(i_millis: u64) {
    spin_sleep_micros(i_millis * 1000)
}
