//! Memory Management

use core::ops::RangeInclusive;

//-----------------------------------------------------------------------------
// Public
//-----------------------------------------------------------------------------

// zero_volatile - zero out an inclusive range of memory.
//
// Unsafe:
// * range.start and range.end must be valid - XXX what does this mean?
// * range.start must be <= range.end
// * range.start and range.end must be 'T' aligned - XXX who checks?

pub unsafe fn zero_volatile<T>(range: RangeInclusive<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = *range.start();
    let end = *range.end();

    while ptr <= end {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}
