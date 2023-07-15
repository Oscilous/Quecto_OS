global long_mode_start

section .text
bits 64
long_mode_start:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; print `long mode entered = OK'
    mov rax, 0x2f672f6E2f6F2f6C
    mov qword [0xb8000], rax
    mov rax, 0x2f642f6F2f6D2f5f
    mov qword [0xb8008], rax
    mov rax, 0x2f742f732f5F2f65
    mov qword [0xb8010], rax
    mov rax, 0x2f202f742f722f61
    mov qword [0xb8018], rax
    mov rax, 0x2f4B2f4F2f202f3D
    mov qword [0xb8020], rax

    mov edi, eax
    mov esi, ebx

    ; call the main kernel function in src/lib.rs
    extern _start
    call _start

    hlt
    