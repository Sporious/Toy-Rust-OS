use common::{AUX_ENABLES, MU_REG_BASE};
use core::fmt;
use gpio::{AltFunction, Gpio};
use stdio::{stdin, stdout};
use volatile::{ReadOnly, ReadWrite, Volatile, WriteOnly};

///Auxiliary peripherals Register Map as defined on page 205 figure 2.1 of the Broadcom manual
///
///This structure uses [repr(C)].
///This means that the compiler is NOT free to reorder the fields of this structure for alignment
///reasons, its very important the layout is exactly as ive defined it.

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    ///Mini Uart I/O Data
    MU_IO_REG: ReadWrite<u8>,
    ///3 byte padding
    _a: [u8; 3],

    ///Mini Uart Interrupt Enable
    MU_IER_REG: ReadWrite<u8>,
    _b: [u8; 3],

    ///Mini Uart Interrupt Identify
    MU_IIR_REG: ReadWrite<u8>,
    _c: [u8; 3],

    ///Mini Uart Line Control
    MU_LCR_REG: ReadWrite<u8>,
    _d: [u8; 3],

    ///Mini Uart Modem Control
    MU_MCR_REG: ReadWrite<u8>,
    _e: [u8; 3],

    ///Mini Uart Line Status
    MU_LSR_REG: ReadWrite<u8>,
    _f: [u8; 3],

    ///Mini Uart Modem Status
    MU_MSR_REG: ReadWrite<u8>,
    _g: [u8; 3],

    ///Mini Uart Scratch
    MU_SCRATCH: ReadWrite<u8>,
    _h: [u8; 3],

    ///Mini Uart Extra Control
    MU_CNTL_REG: ReadWrite<u8>,
    _i: [u8; 3],

    ///Mini Uart Extra Status
    MU_STAT_REG: ReadWrite<u32>,

    ///Mini Uart Baudrate
    MU_BAUD_REG: ReadWrite<u16>,
    _j: [u8; 2],
}


///Wrapper for the Uart registers
pub struct Uart {
    registers: &'static mut Registers,
    timeout: Option<u32>,
}

impl Uart {
    ///Constructor
    pub fn new() -> Uart {
        let registers = unsafe {
            
            let mut aux = AUX_ENABLES as *mut u8 as *mut Volatile<u8>;

            //Enable Mini-Uart by setting bit 0 on AUX_ENABLES
            (&mut *(aux)).write((&*(aux)).read() | 0b1);
            &mut *(MU_REG_BASE as *mut Registers)
        };
        
        //set datasize to 8 bit
        registers.MU_LCR_REG.write(0b11);
        registers.MU_BAUD_REG.write(270);

        //Create new Gpio pins and set them to use AltFunction 5, see page 102 of Broadcom manual
        Gpio::new(14).as_alt(AltFunction(5));
        Gpio::new(15).as_alt(AltFunction(5));

        //Enable TX|RX pins
        registers.MU_CNTL_REG.write(0b11); 

        Uart {
            registers,
            timeout: None,
        }
    }
    ///This function enables the CTS and RTS pins and sets their FSEL state via the documentation
    ///on pages 16, 90 and 102 of the Broadcom manual
    ///
    ///Uses builder pattern to make a nice to use constructor, feeding in one into the other like
    ///x = a().b().c();
    pub fn with_auto_flow_control(self) -> Self {
        //Create new Gpio pins and set them to use AltFunction 5, see page 102 of Broadcom manual
        Gpio::new(16).as_alt(AltFunction(5));
        Gpio::new(17).as_alt(AltFunction(5));
        //Set CTS assert polarity, RTS assert polarity, flow level, enable CTS and RTS and enable
        //TX and RX (probs already on from new())
        self.registers.MU_CNTL_REG.write(0b11111111); 
        //return modified structure
        self
    }
    ///Set a timeout for this Uart device via builder pattern
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }
    ///Set a timeout for this Uart device
    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = Some(timeout);
    }

    ///Writes a u8 byte to the device, blocking until written successfully
    pub fn write_byte(&mut self, b: u8) {

        //While bit 5 of MU_LSR_REG is low the transmitter is not available, so spinlock the thread
        //See page 15 of Broadcom manual
        while self.registers.MU_LSR_REG.read() & (1 << 5) == 0 {}
        //When bit 5 is live, we can write to the data fields on MU_IO_REG
        //See page page 11 of Broadcom manual
        self.registers.MU_IO_REG.write(b);
    }

    ///If bit 0 of the MU_LSR_REG is high there is data waiting to be read out
    pub fn has_byte(&self) -> bool {
        //Use a binary AND operation to read only 1 bit
        self.registers.MU_LSR_REG.read() & (1 << 0) == 1
    }

    ///Read the incoming data from the device
    pub fn read_byte(&self) -> u8 {

        self.registers.MU_IO_REG.read()
    }

    ///Pushes the data onto stdin directly
    pub fn read_to_stdin(&self) {
        //Get new stdin handle
        let mut stdin = stdin().unwrap();
        while self.has_byte() {
            stdin.push(self.read_byte()).unwrap();
        }
    }
    
    ///Write all data in stdout to the device then clear it
    pub fn flush_stdout(&mut self) {
        let mut stdout = stdout().unwrap();
        stdout.into_iter().for_each(|&x| self.write_byte(x));
        stdout.clear();
    }
}
///Implement write for Uart, allowing access to many methods for writing different types of output
///through the device
impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for i in s.chars() {
            self.write_byte(i as u8)
        }
        Ok(())
    }
}
