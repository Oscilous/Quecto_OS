# Quecto_OS
Good ol' hobby operating system written in Rust. The initial implementation is for the x86-64 architecture(AMD64).

## How to build

### Dependencies

* Rust (nightly)
* Nasm
* Make
* QEMU (or other VM for running)
* grub-mkrescue (For generating an ISO)

### Build process

After installing the dependencies, you can run `make` to build the OS binary.
The OS contains the Multiboot header, so it can be booted using any Multiboot compatible bootloader.

If you want to fully use all OS features, you need the InitRD filesystem, for this you need an ISO with
GRUB included. Run `make iso` to build it.

2 files will be created in the `build/` directory of the project: 

* `quecto_kernel-x86_64.bin` - Bootable OS binary, that can be used by GRUB on your machine.
* `quecto_os-x86_64.iso` - OS image for booting as CDROM.

## Running

To run the created binary you can use any Multiboot compatible bootloader. QEMU can directly boot it, too.
The Makefile contains a `run` command that starts QEMU with the following arguments:

By .bin:
```bash
qemu-system-x86_64 -kernel quecto_kernel-x86_64.bin
```

or 
By .iso:
```bash
qemu-system-x86_64 -cdrom quecto_os-x86_64.iso
```

## Progress

* [x] Long mode
* [?] Framebuffer
* [] Serial output
* [] Interrupts
* [] Timer peripheral
* [] Memory management
* [] Filesystem
* [] Processes
* [] Syscalls/User space

## Organization

- Kernel code is placed in the `kernel` Rust crate, which contains the following:
  - `src/` - all the kernel code
  - `src/arch/` - platform specific internals (including target description, linker script and startup code)