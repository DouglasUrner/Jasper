//! Archicecture specific code
//!
//! crate::cpu::arch_cpu - XXX ???

//-----------------------------------------------------------------------------
// Public
//-----------------------------------------------------------------------------

// wait_forever - halt the calling core
#[inline(always)]
pub fn wait_forever() -> ! {
    unsafe {
        loop {
            #[rustfmt::skip]
            asm!(
                "wfe",
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}
