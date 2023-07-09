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
    ; print `OKAY` to screen
    mov rax, 0x2f6c2f6c2f652f48
    mov qword [0xb8000], rax
    mov rax, 0x2f202f6f2f722f66
    mov qword [0xb8010], rax
    mov rax, 0x2f6d2f6f2f512f20
    mov qword [0xb8020], rax
    mov rax, 0x2f652f752f742f63
    mov qword [0xb8030], rax
    mov rax, 0x2f4f2f6f2f212f53
    mov qword [0xb8040], rax
    hlt