package := spicy-os
arch := riscv32
triple := riscv32imac-unknown-none-elf
mode := debug
addr := 0x80400000

kernel := target/$(triple)/$(mode)/$(package)
bin := target/$(triple)/$(mode)/kernel.bin
# objdump := rust-objdump --arch-name=$(arch)
objdump := riscv32-unknown-elf-objdump
objcopy := rust-objcopy --binary-architecture=$(arch)
qemu := qemu-system-$(arch)

.PHONY: kernel build qemu run clean asm readelf

kernel: 
	cargo build

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

build: $(bin)

qemu: build
	$(qemu) \
        -machine virt \
        -nographic \
        -bios default \
        -device loader,file=$(bin),addr=$(addr)

run: build qemu

asm:
	$(objdump) -d $(kernel) | less

readelf:
	$(readelf) -S $(kernel) | less

clean:
	cargo clean
