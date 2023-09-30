all: quecto_os.x86_64.elf

# Kernel build
quecto_os.x86_64.elf: src/**
	objcopy -O elf64-x86-64 -B i386 -I binary font.psf font.o
	cargo build -Z build-std=core,alloc --target ./triplets/quecto_os-x86.json
	cp ./target/quecto_os-x86/debug/quecto_os quecto_os.x86_64.elf
