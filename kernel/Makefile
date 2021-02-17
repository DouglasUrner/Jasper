# Default target
BSP ?= rpi3

ifeq ($BSP), rpi0)
else ifeq ($(BSP), rpi3)
TARGET				= aarch64-unknown-none
VERSION				= debug
QEMU_CMD			= qemu-system-aarch64
QUEM_MACH			= raspi3
QEMU_FLAGS			= -d in_asm -display none
RUSTC_MISC_FLAGS	= -C target-cpu=cortex-a53
LINK_SCRIPT			= src/bsp/raspberrypi/link.ld
else ifeq ($(BSP), rpi4)
endif

KERNEL_PATH			= target/$(TARGET)/$(VERSION)/kernel

# Export, used by build.rs
export LINK_SCRIPT

RUSTFLAGS			= -C link-arg=-T$(LINK_SCRIPT) $(RUSTC_MISC_FLAGS)

FEATURES 			= bsp_$(BSP)
COMPILE_FLAGS		= --target=$(TARGET) --features $(FEATURES)
RUSTC_CMD			= cargo rustc $(COMPILE_FLAGS)
CHECK_CMD			= cargo check $(COMPILE_FLAGS)

.PHONY: all kernel qemu check

all: kernel

kernel:
	RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

qemu: kernel
	$(QEMU_CMD) -M $(QUEM_MACH) $(QEMU_FLAGS) -kernel $(KERNEL_PATH)

check:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json

clean:
	rm -rf target $(KERNEL_BIN)
