use core::fmt;
use core::iter::{IntoIterator, Iterator};
use core::ops::Drop;
use core::slice::Iter;
use core::sync::atomic::{AtomicBool, Ordering};

pub static mut STDINBACK: StdioBack = StdioBack {
    backing: [0; 1000],
    cursor: 0,
    guard_out: AtomicBool::new(false),
};

pub static mut STDOUTBACK: StdioBack = StdioBack {
    backing: [0; 1000],
    cursor: 0,
    guard_out: AtomicBool::new(false),
};

pub struct StdioBack {
    backing: [u8; 1000],
    cursor: usize,
    guard_out: AtomicBool,
}

pub struct Stdio<'a> {
    stdioback: &'a mut StdioBack,
}
impl<'a> Drop for Stdio<'a> {
    fn drop(&mut self) {
        self.stdioback.backing = [0; 1000];
        self.stdioback.guard_out.store(false, Ordering::Relaxed);
    }
}

pub fn stdin<'a>() -> Result<Stdio<'a>, ()> {
    unsafe {
        match STDINBACK.guard_out.load(Ordering::Relaxed) {
            true => Err(()),
            false => Ok(Stdio::new(&mut STDINBACK)),
        }
    }
}
pub fn stdout<'a>() -> Result<Stdio<'a>, ()> {
    unsafe {
        match STDOUTBACK.guard_out.load(Ordering::Relaxed) {
            true => Err(()),
            false => Ok(Stdio::new(&mut STDOUTBACK)),
        }
    }
}

impl<'a> Stdio<'a> {
    fn new(stdioback: &mut StdioBack) -> Stdio {
        stdioback.guard_out.store(true, Ordering::Relaxed);
        Stdio { stdioback }
    }
    pub fn clear(&mut self) {
        //self.stdioback.backing = [0; 1000];
        self.stdioback.cursor = 0;
    }
    pub fn push<T: Into<u8>>(&mut self, c: T) -> Result<(), ()> {
        match self.stdioback.cursor >= (self.stdioback.backing.len() - 1) {
            true => Err(()),
            false => {
                self.stdioback.backing[self.stdioback.cursor] = c.into();
                self.stdioback.cursor += 1;
                Ok(())
            }
        }
    }
}

impl<'a> IntoIterator for &'a Stdio<'a> {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        match self.stdioback.cursor {
            0 => [].into_iter(),
            _ => self.stdioback.backing[0..self.stdioback.cursor].into_iter(),
        }
    }
}

impl<'a> fmt::Write for Stdio<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if self.push(c as u8).is_err() {
                return Err(fmt::Error);
            }
        }
        Ok(())
    }
}
