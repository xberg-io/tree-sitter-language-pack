const STACK_SIZE 4096

.section .text
.global _start

_start:
    mov rax, 60
    call clear
    jmp .done
.done:
    ret

clear:
    xor rdi, rdi
    ret
