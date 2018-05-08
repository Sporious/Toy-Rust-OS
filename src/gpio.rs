use common::GPIO_BASE;
use core::marker::PhantomData;
use volatile::{ReadOnly, ReadWrite, WriteOnly};

enum FunctionSelectMask {
    Input = 0b000,
    Output = 0b001,
    AF0 = 0b100,
    AF1 = 0b101,
    AF2 = 0b110,
    AF3 = 0b111,
    AF4 = 0b011,
    AF5 = 0b010,
}
pub struct AltFunction(pub u8);

impl Into<FunctionSelectMask> for AltFunction {
    fn into(self) -> FunctionSelectMask {
        match self.0 {
            0 => FunctionSelectMask::AF0,
            1 => FunctionSelectMask::AF1,
            2 => FunctionSelectMask::AF2,
            3 => FunctionSelectMask::AF3,
            4 => FunctionSelectMask::AF4,
            5 => FunctionSelectMask::AF5,
            _ => panic!(),
        }
    }
}

/*
    Layout for this struct can be found on page 90
*/
#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    FSEL: [ReadWrite<u32>; 6], // 0000 -> 0018; 6 Function select 32 bits (R/W)
    _a: u32,                   //0018 -> 001C reserved
    SET: [WriteOnly<u32>; 2],  //001C -> 0024; 2 function set 32 bits(W)
    _b: u32,
    CLR: [WriteOnly<u32>; 2], //Clear
    _c: u32,
    LEV: [ReadOnly<u32>; 2], //Level
    _d: u32,
    EDS: [ReadWrite<u32>; 2], //Event detect status
    _e: u32,
    REN: [ReadWrite<u32>; 2], //Rising edge detect status
    _f: u32,
    FEN: [ReadWrite<u32>; 2], //Falling edge detect status
    _g: u32,
    HEN: [ReadWrite<u32>; 2], //High detect enable
    _h: u32,
    LEN: [ReadWrite<u32>; 2], //Low detect enable
    _i: u32,
    AREN: [ReadWrite<u32>; 2], //Async rising edge detect
    _j: u32,
    AFEN: [ReadWrite<u32>; 2], //Async falling edge detect
    _k: u32,
    PUD: ReadWrite<u32>,         //pull-up/down enable
    PUDCLK: [ReadWrite<u32>; 2], //pull-up/down enable clock
}

pub enum Uninitialised {}
pub enum Input {}
pub enum Output {}
pub enum Alt {}

pub struct Gpio<State> {
    pin: u8,
    registers: &'static mut Registers,
    _phantom: PhantomData<State>,
}
impl<T> Gpio<T> {
    fn transition<S>(self) -> Gpio<S> {
        Gpio {
            pin: self.pin,
            registers: self.registers,
            _phantom: PhantomData,
        }
    }
}

impl Gpio<Uninitialised> {
    pub fn new(pin: u8) -> Gpio<Uninitialised> {
        if pin > 53 {
            panic!()
        }

        Gpio {
            registers: unsafe { &mut *(GPIO_BASE as *mut Registers) },
            pin: pin,
            _phantom: PhantomData,
        }
    }

    pub fn as_output(mut self) -> Gpio<Output> {
        self.update_pin_fsel(FunctionSelectMask::Output);
        self.transition()
    }
    pub fn as_input(mut self) -> Gpio<Input> {
        self.update_pin_fsel(FunctionSelectMask::Input);
        self.transition()
    }
    pub fn as_alt<T: Into<FunctionSelectMask>>(mut self, f: T) -> Gpio<Alt> {
        self.update_pin_fsel(f.into());
        self.transition()
    }

    fn update_pin_fsel(&mut self, f: FunctionSelectMask) {
        let fsel_mumber = self.pin / 10;
        let pin_offset = (self.pin % 10) * 3;
        {
            let fsel = &mut self.registers.FSEL[fsel_mumber as usize];
            let old = fsel.read();
            fsel.write(((f as u32) << (pin_offset)) | old);
        }
    }
}
impl Gpio<Output> {
    pub fn set(&mut self) {
        let set_number = self.pin / 32;
        let pin_offset = self.pin % 32;
        self.registers.SET[set_number as usize].write(0b1 << pin_offset);
    }
    pub fn clear(&mut self) {
        let set_number = self.pin / 32;
        let pin_offset = self.pin % 32;
        self.registers.CLR[set_number as usize].write(0b1 << pin_offset);
    }
}

impl Gpio<Input> {
    pub fn read_level(&mut self) -> bool {
        unimplemented!()
    }
}

/*



*/
