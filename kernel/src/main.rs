#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod memory;
mod panic_wait;
mod runtime_init;

//-----------------------------------------------------------------------------
// Early initialization
//
// Unsafe - must be called before starting additional cores.
//-----------------------------------------------------------------------------

unsafe fn kernel_init() -> ! {
    panic!()
}
