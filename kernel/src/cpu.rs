//! Processor Specific Code

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/cpu.rs"]
mod arch_cpu;

mod boot;

//--------------------------------------------------------------------------------------------------
// Public Re-exports
//--------------------------------------------------------------------------------------------------
pub use arch_cpu::wait_forever;
