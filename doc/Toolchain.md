# Rust (OS Development) Toolchain

## Editor

* [VS Code](https://code.visualstudio.com)
  * [Rust Analyzer](https://rust-analyzer.github.io)

## Compiler

Standard install of [Rust](https://rust-lang.org) using `rustup`. Use the nightly build by placing a file named `rust-toolchain` in the root of the project containing the line "nightly."

Add the ARM target triple `rustup target add aarch64-unknown-none`.

## Emulator

* [QEMU](https://qemu.org)
  * `brew install qemu`
    * Run as `qemu-system-aarch64 -M raspi3 -serial stdio -kernel <path-to-kernel>`

## Debugger

## Target Boards
