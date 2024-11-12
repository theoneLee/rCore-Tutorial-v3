.PHONY: build run

build:
	cargo build --release

run: build
	qemu-system-riscv64 \
		-machine virt \
        -nographic \
        -bios ../../../bootloader/rustsbi-qemu.bin \
        -device loader,file=target/riscv64gc-unknown-none-elf/release/helloworld,addr=0x80200000
        