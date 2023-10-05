arch ?= x86_64
kernel := target/quecto_kernel-$(arch).elf
iso := build/quecto_os-$(arch).iso
grub_cfg := src/arch/$(arch)/grub.cfg

.PHONY: run iso build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

build: quecto_os.x86_64.elf

# Kernel build
quecto_os.x86_64.elf: src/**
	objcopy -O elf64-x86-64 -B i386 -I binary font.psf font.o
	cargo build -Z build-std=core,alloc --target ./triplets/quecto_os-x86.json
	cp ./target/quecto_os-x86/debug/quecto_os quecto_os.x86_64.elf