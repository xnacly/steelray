# Rust target triple for bare-metal 64-bit ARM.
TARGET = aarch64-unknown-none

# Output ELF loaded by QEMU.
KERNEL = kernel.elf

QEMU = qemu-system-aarch64
QEMU_RAM = 128M

RUSTFLAGS = \
	-C panic=abort \
	-C opt-level=s \
	-C debuginfo=0 \
	-C debug-assertions=off \
	-C overflow-checks=off \
	-C relocation-model=static \
	-C link-arg=-Tlinkmyballs.ld \
	-C link-arg=--gc-sections

# These names are commands, not files that make should look for.
.PHONY: all run debug clean

# Build the kernel directly with rustc. Cargo is not needed yet because this
# kernel has no dependencies or build script.
all:
	rustc \
		--target $(TARGET) \
		$(RUSTFLAGS) \
		-o $(KERNEL) \
		src/main.rs

# Boot the kernel in QEMU. `-nographic` connects the emulated serial port to
# this terminal, which is why UART output appears in `make run`.
run: all
	$(QEMU) \
	  -M virt \
	  -cpu cortex-a57 \
	  -m $(QEMU_RAM) \
	  -nographic \
	  -kernel $(KERNEL)

# Start QEMU paused and expose a GDB server on localhost:1234. This lets a
# debugger attach before the first instruction executes.
debug: all
	$(QEMU) \
	  -M virt \
	  -cpu cortex-a57 \
	  -m $(QEMU_RAM) \
	  -nographic \
	  -S -s \
	  -kernel $(KERNEL)

# Remove the generated kernel image.
clean:
	rm -f $(KERNEL)
