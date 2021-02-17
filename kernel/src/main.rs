 //! # Boot Sequence
 //!
 //! 1. Kernel entry point is the function [`cpu::boot::arch_boot::_start()`].
 //!     - It is implemented in 'src/_arch/__arch_name__/cpu/boot.rs' and
 //!     'src/_arch/__arch_name__/cpu/boot.rs'.
 //! 2. Once the architectural specific setup completes, the arch code calls
 //!    [`runtime_init::runtime_init()'].
 //!
 //! [`cpu::boot::arch_boot::_start()`]: ../src/kernel/cpu/up/_arch/aarch64/cpu/boot.rs.html
 //! [`cpu::boot::arch_boot::_start()`]: cpu/boot/arch_boot/fn._start.html
 //! [`runtime_init::runtime_init()`]: runtime_init/fn.runtime_init.html

#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod memory;
mod panic_wait;
mod print;
mod runtime_init;

//-----------------------------------------------------------------------------
// Early initialization
//
// Unsafe - must be called before starting additional cores.
//-----------------------------------------------------------------------------

unsafe fn kernel_init() -> ! {
    println!("[0] I'm Rusting...");

    panic!("Stopping here.")
}
