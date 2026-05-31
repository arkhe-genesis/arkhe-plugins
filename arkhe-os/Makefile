.PHONY: all bootloader kernel servers tools iso clean

all: bootloader kernel servers tools iso

bootloader:
	@echo "Building bootloader..."
	nasm -f elf64 boot/bootloader.asm -o boot/bootloader.o
	ld -T boot/bootloader.ld -o boot/bootloader.bin boot/bootloader.o

kernel:
	@echo "Building kernel..."
	cd kernel && cargo build --release

servers:
	@echo "Building servers..."
	cd servers/vfs && cargo build --release
	cd servers/net && cargo build --release
	cd servers/passport && cargo build --release
	cd servers/orchestrator && cargo build --release
	cd servers/bindu && cargo build --release

tools:
	@echo "Building tools..."
	cd tools/arkhe-sh && cargo build --release
	cd tools/pkg && cargo build --release
	cd tools/checkpoint && cargo build --release

iso:
	@echo "Building ISO..."
	# Placeholder for creating a bootable ISO

clean:
	rm -f boot/*.o boot/*.bin
	cargo clean
