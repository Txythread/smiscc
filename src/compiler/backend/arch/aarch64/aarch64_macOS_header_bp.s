    .align 2
    .global _start

_print:
        mov     x2, x1
        mov     x1, x0
        mov     x0, #1
        mov     x16, #4
        svc     #0
        ret
