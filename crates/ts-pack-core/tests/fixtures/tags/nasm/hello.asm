%define SYS_EXIT 60

%macro exit 1
    mov rax, SYS_EXIT
    mov rdi, %1
    syscall
%endmacro

LEN equ 13

struc point
    .x: resd 1
    .y: resd 1
endstruc

section .data
msg: db "Hello, world", 10

section .text
global _start

_start:
    call print
    exit 0

print:
    mov rsi, msg
.loop:
    dec rdx
    jnz .loop
    ret
