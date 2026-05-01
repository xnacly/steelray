TARGET = aarch64-unknown-none

all:
	aarch64-linux-gnu-as -o start.o start.S
	rustc \
		--target $(TARGET) \
		-C panic=abort \
		-C overflow-checks=off \
		-C link-arg=start.o \
		-C linker=aarch64-linux-gnu-ld \
		-C link-arg=-Tlinker.ld \
		-o kernel.elf \
		src/main.rs

run: all
	qemu-system-aarch64 \
		-M virt \
		-cpu cortex-a57 \
		-nographic \
		-S -s \
		-kernel kernel.elf

clean:
	rm kernel.elf start.o
