TARGET = aarch64-unknown-none

KERNEL = kernel.elf
CARGO_KERNEL = target/$(TARGET)/debug/steelray

QEMU = qemu-system-aarch64
QEMU_RAM = 128M

.PHONY: all run debug clean

# Build the kernel through Cargo. Target and linker flags live in
# `.cargo/config.toml`.
all:
	cargo build
	cp $(CARGO_KERNEL) $(KERNEL)

run: all
	$(QEMU) \
	  -M virt \
	  -cpu cortex-a57 \
	  -m $(QEMU_RAM) \
	  -nographic \
	  -kernel $(KERNEL)

debug: all
	$(QEMU) \
	  -M virt \
	  -cpu cortex-a57 \
	  -m $(QEMU_RAM) \
	  -nographic \
	  -S -s \
	  -kernel $(KERNEL)

clean:
	rm -f $(KERNEL)
