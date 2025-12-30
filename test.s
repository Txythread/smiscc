    .align 2
    .global _start

_start:
	mov	x0, #5
	b	_print
	mov	x0, #1
	mov	x16, #1
	svc	#0x80
