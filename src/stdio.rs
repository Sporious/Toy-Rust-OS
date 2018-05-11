use core::fmt;
use core::iter::{IntoIterator, Iterator};
use core::ops::Drop;
use core::slice::Iter;

pub static mut STDINBACK: StdioBack = StdioBack {
    backing: [0; 1000],
    cursor: 0,
    guard_out: false,
};

pub static mut STDOUTBACK: StdioBack = StdioBack {
    backing: [0; 1000],
    cursor: 0,
    guard_out: false,
};

pub struct StdioBack {
    backing: [u8; 1000],
    cursor: usize,
    guard_out: bool,
}

pub struct Stdio<'a> {
    stdioback: &'a mut StdioBack,
}
impl<'a> Drop for Stdio<'a> {
    fn drop(&mut self) {
        self.stdioback.guard_out = false;
    }
}

pub fn stdin<'a>() -> Result<Stdio<'a>, ()> {
    unsafe {
        match STDINBACK.guard_out {
            true => Err(()),
            false => Ok(Stdio::new(&mut STDINBACK)),
        }
    }
}
pub fn stdout<'a>() -> Result<Stdio<'a>, ()> {
    unsafe {
        match STDOUTBACK.guard_out {
            true => Err(()),
            false => Ok(Stdio::new(&mut STDOUTBACK)),
        }
    }
}

impl<'a> Stdio<'a> {
    fn new(stdioback: &mut StdioBack) -> Stdio {
        stdioback.guard_out = true;
        Stdio { stdioback }
    }
    pub fn clear(&mut self) {
        self.stdioback.backing = [0; 1000];
        self.stdioback.cursor = 0;
    }
    pub fn push<T: Into<u8>>(&mut self, c: T) -> Result<(), ()> {
        match self.stdioback.cursor + 1 >= (self.stdioback.backing.len()) {
            true => Err(()),
            false => {
                self.stdioback.backing[self.stdioback.cursor] = c.into();
                self.stdioback.cursor += 1;
                Ok(())
            }
        }
    }
    pub fn pop(&mut self) -> Option<u8> {
        match self.len() {
            0 => None,
            n @ _ => {
                let out = Some(self.stdioback.backing[n]);
                self.stdioback.cursor -= 1;
                out
            }
        }
    }
    pub fn len(&self) -> usize {
        self.stdioback.cursor
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.stdioback.backing[..self.len()]
    }
    pub fn as_str(&self) -> Option<&str> {
        use core::str::from_utf8;
        if self.len() < 1 {
            return None;
        }
        Some(from_utf8(self.as_slice()).expect("Failed to parse as slice"))
    }
}

impl<'a> IntoIterator for &'a Stdio<'a> {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.stdioback.backing[..self.stdioback.cursor].into_iter()
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
