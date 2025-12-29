    .align 2
    .global _start

_start:
	mov	x0, #5
	b	_print
	mov	x1, #0
	mov	x16, #1
	mov	x0, x1
	svc	#0x80
