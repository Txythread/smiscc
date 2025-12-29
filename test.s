    .align 2
    .global _start

_start:
	mov	x0, 3
	mov	x1, 5
	mov	x0, x2
	mov	x0, x1
	ble	_print
	mov	x1, 0
	mov	x16, #1
	mov	x0, x1
	svc	#0x80
