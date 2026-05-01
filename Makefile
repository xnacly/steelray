TARGET = aarch64-unknown-none

all:
	rustc \
		--target $(TARGET) \
		-C panic=abort \
		-C overflow-checks=off \
	    -C link-arg=-Tlinkmyballs.ld \
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
	rm kernel.elf
