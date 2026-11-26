const STACK_SIZE 4096
const EXIT_CODE 0

.section .text
.global _start

_start:
    mov rax, 60
    call clear
    jmp .done
.done:
    ret

label reset
    xor rax, rax
    ret

clear:
    xor rdi, rdi
    ret
