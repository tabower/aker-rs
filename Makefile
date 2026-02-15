ARCH      := riscv64
BUILD_MOD := debug

include mk/$(ARCH).mk

KERNEL    := aker-rs

# Paths
BUILD_DIR  := target/$(TARGET)/$(BUILD_MOD)
KERNEL_ELF := $(BUILD_DIR)/$(KERNEL)
KERNEL_BIN := $(BUILD_DIR)/$(KERNEL).bin
KERNEL_ASM := $(BUILD_DIR)/$(KERNEL).asm

# Toolchain
CARGO   := cargo
DTC     := dtc
OBJDUMP := rust-objdump
OBJCOPY := rust-objcopy
GDB     := RUST_GDB=gdb-multiarch rust-gdb

# Cargo flags
CARGO_FLAGS := --target $(TARGET)
ifeq ($(BUILD_MOD), release)
    CARGO_FLAGS += --release
endif

# QEMU flags
QEMU_OPTS = -machine $(QEMU_MACHINE) \
            -m $(QEMU_MEMS) \
            -smp $(QEMU_CPUS) \
            -kernel $(KERNEL_ELF) \
            -accel tcg,thread=multi \
            -nographic \
            -serial mon:stdio \
            $(QEMU_ARCH_OPTS)

# GDB
GDB_PORT   := 1234
QEMU_DEBUG := -gdb tcp::$(GDB_PORT) -S

# Expand arch-specific rules
$(eval $(ARCH_RULES))

# Targets
.PHONY: all build clean kernel asm bin run qemu qemu-gdb gdb $(ARCH_BUILD_TARGETS)

.DEFAULT_GOAL := build

all: build

build: kernel asm $(ARCH_BUILD_TARGETS)

kernel:
	@echo "   CARGO    $(KERNEL_ELF)"
	@$(CARGO) build $(CARGO_FLAGS)

asm: kernel
	@echo "   OBJDUMP  $(KERNEL_ASM)"
	@$(OBJDUMP) -D -j .text -S $(KERNEL_ELF) 2>/dev/null | rustfilt > $(KERNEL_ASM)

bin: kernel
	@echo "   OBJCOPY  $(KERNEL_BIN)"
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

run: qemu
qemu: build
	@echo "   QEMU     Running..."
	$(QEMU) $(QEMU_OPTS)

qemu-gdb: build
	@echo "   QEMU     Waiting for GDB on port $(GDB_PORT)..."
	$(QEMU) $(QEMU_OPTS) $(QEMU_DEBUG)

gdb:
	@if [ ! -f $(KERNEL_ELF) ]; then \
		echo "   [ERROR]  Kernel ELF not found!"; \
		exit 1; \
	fi
	@echo "   GDB      Connecting..."
	$(GDB) $(KERNEL_ELF) $(GDB_FLAGS)

clean:
	@echo "   CLEAN"
	@cargo clean