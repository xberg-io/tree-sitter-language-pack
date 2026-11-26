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

; The grammar also parses `objdump -d -M intel` listings, such as this one of a linked helper
0000000000401000 <helper>:
    mov rax, 1
