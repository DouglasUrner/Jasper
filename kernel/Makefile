# Default target
BSP ?= rpi3

ifeq ($(BSP), rpi3)
TARGET				= aarch64-unknown-none
VERSION				= debug
QEMU_CMD			= qemu-system-aarch64
QUEM_MACH			= raspi3
QEMU_FLAGS			= -d in_asm -display none
RUSTC_MISC_FLAGS	= -C target-cpu=cortex-a53
LINK_INS			= src/bsp/raspberrypi/link.ld
endif

KERNEL_PATH			= target/$(TARGET)/$(VERSION)/kernel

# Export, used by build.rs
export LINK_INS

RUSTFLAGS			= -C link-arg=-T$(LINK_INS) $(RUSTC_MISC_FLAGS)

FEATURES 			= bsp_$(BSP)
COMPILE_FLAGS		= --target=$(TARGET) --features $(FEATURES)
RUSTC_CMD			= cargo rustc $(COMPILE_FLAGS)

.PHONY: all kernel qemu

all: kernel

kernel:
	RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

qemu: kernel
	$(QEMU_CMD) -M $(QUEM_MACH) $(QEMU_FLAGS) -kernel $(KERNEL_PATH)