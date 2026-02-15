# --- RISC-V 64-bit Architecture Configuration ---

TARGET       := riscv64gc-unknown-none-elf
QEMU         := qemu-system-riscv64

# QEMU machine
QEMU_MACHINE := virt
QEMU_CPUS    := 3
QEMU_MEMS    := 3G

# NUMA topology
QEMU_NUMA_OPTS := \
    -object memory-backend-ram,id=mem0,size=1792M \
    -object memory-backend-ram,id=mem1,size=256M \
    -object memory-backend-ram,id=mem2,size=1024M \
    -numa node,nodeid=0,memdev=mem0,cpus=0 \
    -numa node,nodeid=1,memdev=mem1,cpus=1 \
    -numa node,nodeid=2,memdev=mem2,cpus=2

# BIOS
# https://github.com/riscv-software-src/opensbi/releases
QEMU_BIOS := firmware/opensbi/fw_jump.bin

# Device tree
QEMU_DTS_SOURCE := firmware/dts/riscv64/qemu-virt.dts
DTB_OUTPUT       = $(BUILD_DIR)/qemu-virt.dtb

# Arch-specific build targets
ARCH_BUILD_TARGETS := dtb

# Arch-specific QEMU options
QEMU_ARCH_OPTS = -bios $(QEMU_BIOS) \
                 -dtb $(DTB_OUTPUT) \
                 $(QEMU_NUMA_OPTS)

# Arch-specific build rules
define ARCH_RULES

dtb:
	@mkdir -p $$(BUILD_DIR)
	@echo "   DTC      $$(DTB_OUTPUT)"
	@$$(DTC) -I dts -O dtb -o $$(DTB_OUTPUT) $$(QEMU_DTS_SOURCE)

endef