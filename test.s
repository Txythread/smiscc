    .align 2
    .global _start

_print:
    mov x1, #0
    ret

_stray:

LB0:
	mov	x0, #0
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
