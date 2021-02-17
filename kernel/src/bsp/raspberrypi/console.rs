//! BSP Console (UART)

use crate::console;
use core::fmt;

//-----------------------------------------------------------------------------
// Public
//-----------------------------------------------------------------------------

pub fn console() -> impl console::interface::Write {
    QEMUOutput {}
}

//-----------------------------------------------------------------------------
// Private Definitions
//-----------------------------------------------------------------------------

// Magic, don't ask...
struct QEMUOutput;

//-----------------------------------------------------------------------------
// Private
//-----------------------------------------------------------------------------

/**
*** Implementing the trait (interface) 'core::fmt::Write' enables use of the
*** 'format_args!' macros, which in turn are used to implement the kernel
*** 'print!' and 'println' macros. Implementing 'write_str()' also gives us
*** 'write_fmt()'. Magic.
***
*** See ['src/print.rs'].
***
*** ['src/print.rs']: ../../print/index.html
**/

impl fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
        Ok(())
    }
}
