use core::ops::Drop;
use core::iter::{IntoIterator, Iterator};
use core::slice::Iter;

pub static mut STDINBACK: StdinBack = StdinBack {
    backing: [0;1000],
    guard_out: false
};

pub struct StdinBack {
    backing: [u8;1000],
    guard_out: bool
}


pub struct Stdin<'a> {
    stdinback: &'a mut StdinBack,
    position: usize,
}
impl<'a> Drop for Stdin<'a> {
    fn drop(&mut self) {
        self.stdinback.backing = [0;1000];
        self.stdinback.guard_out = false;
    }
}

pub fn stdin<'a>() -> Result<Stdin<'a>, ()> {
    unsafe {
    if STDINBACK.guard_out {
        Err(())
    }
    else {
        Ok(Stdin::new( &mut STDINBACK))
    }
    }
}

impl<'a> Stdin<'a> {
    fn new(stdinback: &mut StdinBack) -> Stdin {
        stdinback.guard_out = true;
        Stdin {
            stdinback,
            position: 0,
        }
    }
    pub fn clear(&mut self) {
        self.stdinback.backing = [0;1000];
        self.position = 0;
    }
   pub  fn push<T: Into<u8>>(&mut self, c: T) -> Result< (), () > {
        if self.position >= (self.stdinback.backing.len() - 1)
        {
            Err(())
        }
        else {
            self.stdinback.backing[self.position] = c.into();
            self.position += 1;
            Ok(())
        }
    }
}

impl <'a> IntoIterator for &'a Stdin<'a> {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        if self.position == 0 {
            [].into_iter()
        }
        else {
            self.stdinback.backing[0..self.position].into_iter()
        }
    }
}