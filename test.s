	.global _start
	.align 2

_start:
	mov     x0, 1
        mov     x1, 2
        mov     x2, 3
        mov     x3, x1
        mul     x3, x3, x2
        mov     x4, x0
        add     x4, x4, x3
        mov     x0, 2
        mov     x3, x4
        sdiv    x3, x3, x0

	mov x16, 1
	mov x0, x3
	svc 0x80
