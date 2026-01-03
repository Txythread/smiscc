    .align 2
    .global _start

_print:
    ret

_start:
	mov	x0, #5
	mov	x19, x0
	mov	x0, x19
	bl	_print
	mov	x0, #0
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
