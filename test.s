    .align 2
    .global _start

_start:
	mov	x0, #1
	mov	x1, #2
	add	x1, x1, x0
	mov	x2, #2
	mov	x3, x1
	sdiv	x3, x3, x2
	mov	x4, #1
	mov	x4, #1
	add	x4, x4, x1
	mov	x1, x4
	mov	x4, x1
	mov	x4, #1
	mov	x5, x1
	sub	x5, x5, x4
	mov	x16, #1
	mov	x0, x5
	svc	#0x80
