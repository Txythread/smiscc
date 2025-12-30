    .align 2
    .global _start

_start:
	mov x0, #1
	mov x19, #5
#	b	_print
	mov	x16, #1
	mov	x0, x19
	svc	#0x80
