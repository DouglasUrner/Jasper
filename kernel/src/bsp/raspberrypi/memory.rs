//! BSP Memory Management

use core::{cell::UnsafeCell, ops::RangeInclusive};

//-----------------------------------------------------------------------------
// Public
//-----------------------------------------------------------------------------

// bss_range_inclusive - return a range spanning the .bss section.
//
// * The values provided come from the linker script, and must be truste as-is.
// * The linker script is responsible for ensuring that they are u64 aligned.

pub fn bss_range_inclusive() -> RangeInclusive<*mut u64> {
    let range;
    unsafe {
        range = RangeInclusive::new(__bss_start.get(), __bss_end_inclusive.get());
    }
    assert!(!range.is_empty());

    range
}

//-----------------------------------------------------------------------------
// Private
//-----------------------------------------------------------------------------

// Symbols from the linker script:
extern "Rust" {
    static __bss_start: UnsafeCell<u64>;
    static __bss_end_inclusive: UnsafeCell<u64>;
}
