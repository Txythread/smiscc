	.global _start
	.align 2

_start:
	mov     x0, 1
        mov     x1, 2
        mov     x2, 3
        mul     x1, x1, x2
        add     x0, x0, x1
        mov     x3, 2
        udiv    x0, x0, x3

	mov x16, 1
	svc 0x80
