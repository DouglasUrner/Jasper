//! Architecture Specific Boot Code

// Start in assembly
global_asm!(include_str!("boot.S"));
