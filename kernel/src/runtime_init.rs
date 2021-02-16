//! Rust runtime initialization

use crate::{bsp, memory};

//-----------------------------------------------------------------------------
// Public
//-----------------------------------------------------------------------------

// Clear bss, then jump to kernel initialization.
//
// Unsafe - must be called before starting additional cores.

#[no_mangle]
pub unsafe fn runtime_init() -> ! {
    zero_bss();

    crate::kernel_init()
}

//-----------------------------------------------------------------------------
// Private
//-----------------------------------------------------------------------------

// zero_bss() - zero out the BSS section
//
// * Must be called before kernel_init().

#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bsp::memory::bss_range_inclusive());
}
