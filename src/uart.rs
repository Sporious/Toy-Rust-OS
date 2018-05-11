use common::{AUX_ENABLES, MU_REG_BASE};
use core::fmt;
use gpio::{AltFunction, Gpio};
use stdio::{stdin, stdout};
use volatile::{ReadOnly, ReadWrite, Volatile, WriteOnly};

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    MU_IO_REG: ReadWrite<u8>,
    _a: [u8; 3],
    MU_IER_REG: ReadWrite<u8>,
    _b: [u8; 3],
    MU_IIR_REG: ReadWrite<u8>,
    _c: [u8; 3],
    MU_LCR_REG: ReadWrite<u8>,
    _d: [u8; 3],
    MU_MCR_REG: ReadWrite<u8>,
    _e: [u8; 3],
    MU_LSR_REG: ReadWrite<u8>,
    _f: [u8; 3],
    MU_MSR_REG: ReadWrite<u8>,
    _g: [u8; 3],
    MU_SCRATCH: ReadWrite<u8>,
    _h: [u8; 3],
    MU_CNTL_REG: ReadWrite<u8>,
    _i: [u8; 3],
    MU_STAT_REG: ReadWrite<u32>,
    MU_BAUD_REG: ReadWrite<u16>,
    _j: [u8; 2],
}

pub struct Uart {
    registers: &'static mut Registers,
    timeout: Option<u32>,
}

impl Uart {
    pub fn new() -> Uart {
        let registers = unsafe {
            let mut aux = AUX_ENABLES as *mut u8 as *mut Volatile<u8>;
            (&mut *(aux)).write((&*(aux)).read() | 0b1);
            &mut *(MU_REG_BASE as *mut Registers)
        };

        registers.MU_LCR_REG.write(0b11); //set datasize to 8 bit
        registers.MU_BAUD_REG.write(135);
        Gpio::new(14).as_alt(AltFunction(5));
        Gpio::new(15).as_alt(AltFunction(5));
        registers.MU_CNTL_REG.write(0b11); //Enable rx/tx

        Uart {
            registers,
            timeout: None,
        }
    }
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = Some(timeout);
    }

    pub fn write_byte(&mut self, b: u8) {
        while self.registers.MU_LSR_REG.read() & (1 << 5) == 0 {}
        self.registers.MU_IO_REG.write(b);
    }
    pub fn has_byte(&self) -> bool {
        self.registers.MU_LSR_REG.read() & (1 << 0) == 1
    }

    pub fn read_byte(&self) -> u8 {
        self.registers.MU_IO_REG.read()
    }

    pub fn read_to_stdin(&self) {
        let mut stdin = stdin().unwrap();
        while self.has_byte() {
            stdin.push(self.read_byte()).unwrap();
        }
    }

    pub fn flush_stdout(&mut self) {
        let mut stdout = stdout().unwrap();
        stdout.into_iter().for_each(|&x| self.write_byte(x));
        stdout.clear();
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for i in s.chars() {
            self.write_byte(i as u8)
        }
        Ok(())
    }
}
