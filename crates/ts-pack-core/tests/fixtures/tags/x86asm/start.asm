section .text
global _start

_start:
    mov rax, 1
    call print
    jmp .done
.done:
    ret

print:
    mov rdi, 1
    syscall
    ret
