
use core::fmt;
///The IntoIterator and Iterator traits allow for data to be treated as an Iterator, therefore used in for example for loops
use core::iter::{IntoIterator, Iterator};
///The drop trait allows custom destructor code for when things go out of scope
use core::ops::Drop;
use core::slice::Iter;

///Statically allocated storage for the Stdin buffer
pub static mut STDINBACK: StdioBack = StdioBack {
    backing: [0; 1000],
    cursor: 0,
    guard_out: false,
};

///Statically allocated storage for the Stdin buffer, not used in current program, the Uart device is directly forwarded everything
/// However in the future pushing onto this buffer and then having the Uart device read from it and clear it would make code more flexible
pub static mut STDOUTBACK: StdioBack = StdioBack {
    backing: [0; 1000],
    
    cursor: 0, 
    guard_out: false, 
};

/// This structure is a generalisation for the backing storage of both Stdin and Stdout.
/// it acts as a single threaded mutex. It leverages RAII principles to avoid double lock or forgetting to lock problems.
/// 
/// The struct issues a guard, when it has issued a guard it's guard_out is set to true and it is "locked". New guards cannot be aquired in this time
/// when the guard is destroyed, the mutex unlocks and can issue a new guard. It is the responsibility of the guards cleanup code to unlock the mutex.
pub struct StdioBack {

    ///1000 8 bit slots,
    backing: [u8; 1000],
    ///Current head of the stack
    cursor: usize, 
///If we have already given out a guard to this struct
    guard_out: bool,
}

/// This is the guard for StdioBack
/// All functionality to access the StdioBack buffers goes through this
pub struct Stdio<'a> {

    ///Mutable reference to out backing store (Rust enforces at compile time that this is exclusive)
    /// the 'a is a lifetime marker, that indicates that the lifetime of the Stdio structure is equal to the valid lifetime of the pointer to the  backingstore
    /// therefore the compiler will never tolerate a situation where they are not destroyed simultaneously
    stdioback: &'a mut StdioBack, 
}

///This is the implementation of the Drop trait for stdio struct
/// The drop trait allows overriding the default destructor like ~Stdio() would be used in a C++ class
impl<'a> Drop for Stdio<'a> {
    fn drop(&mut self) {
        // Unlock the StdioBack to allow new guards to be created, then drop 
        self.stdioback.guard_out = false; 
    }
}

/// This function creates a new Stdio handle pointing to the STDIN backing store
/// This function can fail, if there is already a guard out, therefore the result type is used and the result of this call
/// must be processed at the call site
pub fn stdin<'a>() -> Result<Stdio<'a>, ()> {
    unsafe {
        match STDINBACK.guard_out {
            true => Err(()),
            false => Ok(Stdio::new(&mut STDINBACK)),
        }
    }
}

///Same as above but for STDOUT
pub fn stdout<'a>() -> Result<Stdio<'a>, ()> {
    unsafe {
        match STDOUTBACK.guard_out {
            true => Err(()),
            false => Ok(Stdio::new(&mut STDOUTBACK)),
        }
    }
}
///Methods for this structure
impl<'a> Stdio<'a> {
    ///Constructor
    fn new(stdioback: &mut StdioBack) -> Stdio {
        stdioback.guard_out = true; ///Lock the mutex 
        Stdio { stdioback }
    }

    ///Reset the backing store
    pub fn clear(&mut self) {
        self.stdioback.backing = [0; 1000];
        self.stdioback.cursor = 0;
    }
    ///Push something onto the store.
    /// This is polymorphic for Into<T>
    /// Into is a trait for converting between types easily. Anything that can be trivially converted to a u8 is valid here
    /// This method of polymorphism uses static dispatch, every possible variant of this call is monomorphised into a distinct function.
    /// This is faster and safer at runtime than reflection techiques but makes the binary bigger and takes more compile time.
    /// 
    /// This function returns a Result because it can fail, the stack may be full and not able to take more data
    pub fn push<T: Into<u8>>(&mut self, c: T) -> Result<(), ()> {
        //Check we're not going to overflow
        match self.stdioback.cursor + 1 >= (self.stdioback.backing.len()) {
            true => Err(()),
            false => {
                //Convert this input to a u8 (again static dispatch makes this cheap) and assign
                //it to the head of this stack style storage
                self.stdioback.backing[self.stdioback.cursor] = c.into();
                //Advance the cursor
                self.stdioback.cursor += 1; 
                //Our push was successful, return Ok
                Ok(())
            }
        }
    }
    ///Pop something off the stack.
    ///Like push this must return a monadic type, as there may not be any data to retrieve
    pub fn pop(&mut self) -> Option<u8> {
        match self.len() {
            //If stack is empty there's nothing to return
            0 => None,
            n @ _ => {
                //Get the data and back off the cursor
                let out = Some(self.stdioback.backing[n]);
                self.stdioback.cursor -= 1;
                out
            }
        }
    }
    pub fn len(&self) -> usize {
        self.stdioback.cursor
    }
    ///Retrieves a reference to a number of consecutive elements of the same type (basically a
    ///start and stop pointer). Rust calls this a slice. This set of data covers all the data up to our cursor
    pub fn as_slice(&self) -> &[u8] {
        &self.stdioback.backing[..self.len()]
    }
    ///This checks the u8 contains valid UTF-8 characters and returns if they do
    pub fn as_str(&self) -> Option<&str> {
        use core::str::from_utf8;
        if self.len() < 1 {
            return None;
        }
        Some(from_utf8(self.as_slice()).expect("Failed to parse as slice"))
    }
}
///The iterator trait is used in the language for elements that yield series of values, any
///iterator is valid to be used in say a for loop
///
///The iterator trait is already implemented for slice::Iter<T> in the core library, the
///IntoIterator trait defines the way to construct an Iter<T> from this struct
///
///Again the lifetime 'a binds the lifetime of the generated Iterator to the data it is iterating
///over, the data is not copied here only referenced
impl<'a> IntoIterator for &'a Stdio<'a> {
    //The type of thing this iterator will yield will be u8 references
    type Item = &'a u8;
    //The specific type of iterator we want
    type IntoIter = Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        //The slice primitive already includes an into_iter implementation in the core library so
        //we just make a slice of our data and call it
        self.stdioback.backing[..self.stdioback.cursor].into_iter()
    }
}

/// The write trait provides some methods for writing to output
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
