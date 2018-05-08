use volatile::{Volatile, ReadWrite, ReadOnly,WriteOnly};
use gpio::{AltFunction, Gpio};
//use common::{IO_BASE, GPIO_BASE};
use common::{MU_REG_BASE, AUX_ENABLES};

#[repr(u8)]
enum LsrStatus {
    DataReady = 1,
    TxAvailable = 1 << 5,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    MU_IO_REG: ReadWrite<u8>,
    _a: [u8;3],
    MU_IER_REG: ReadWrite<u8>,
    _b: [u8;3],
    MU_IIR_REG: ReadWrite<u8>,
    _c: [u8;3],
    MU_LCR_REG: ReadWrite<u8>,
    _d: [u8;3],
    MU_MCR_REG: ReadWrite<u8>,
    _e: [u8;3],
    MU_LSR_REG: ReadWrite<u8>,
    _f: [u8;3],
    MU_MSR_REG: ReadWrite<u8>,
    _g: [u8;3],
    MU_SCRATCH: ReadWrite<u8>,
    _h: [u8;3],
    MU_CNTL_REG : ReadWrite<u8>,
    _i: [u8;3],
    MU_STAT_REG : ReadWrite<u32>,
    MU_BAUD_REG : ReadWrite<u16>,
    _j: [u8;2],

}



pub struct Uart {
    registers: &'static mut Registers,
    timeout: Option<u32>,
}

impl Uart {
    pub fn new() -> Uart {
        let registers = unsafe  {
                let mut aux = AUX_ENABLES as *mut u8 as *mut Volatile<u8> ;
                (&mut * (aux) ).write( (&*(aux)).read() | 0b1);
            &mut *(MU_REG_BASE as *mut Registers)
        };


        
        registers.MU_LCR_REG.write ( 0b11 ); //set datasize to 8 bit
        registers.MU_BAUD_REG.write( 270 );
        Gpio::new(14).as_alt(AltFunction(5));
        Gpio::new(15).as_alt(AltFunction(5));
        registers.MU_CNTL_REG.write( 0b11 ); //Enable rx/tv

        Uart {
        registers, timeout: None
        }

    }
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = Some(timeout);
    }

    pub fn write_byte( &mut self, b: u8) {
        let lsr_tx_idle = self.registers.MU_LSR_REG.read() & (1 << 5);
        while lsr_tx_idle == 0 {

        }
        self.registers.MU_IO_REG.write(b);

        //unimplemented!()
    }
    pub fn has_byte(&self) -> bool {
        unimplemented!()
    }

    pub fn wait_for_byte(&self) -> Result < (), () >
    {

        unimplemented!()
    }

    pub fn read_byte(&mut self) -> u8 {

        unimplemented!()
    }
}






