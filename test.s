    .align 2
    .global _start

_print:
    ret

_start:
	mov	x0, #6
	mov	x1, #7
	add	x1, x1, x0
