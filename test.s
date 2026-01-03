    .align 2
    .global _start

_print:
    mov x1, #0
    ret

_start:
	mov	x0, #1
	mov	x19, x0
	mov	x0, x19
	bl	_print
	mov	x0, x1
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
