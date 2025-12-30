    .align 2
    .global _start

_start:
	mov	x0, #1
	mov	x1, #2
	mov	x2, x0
	add	x2, x2, x1
	mov	x3, #2
	mov	x4, x2
	sdiv	x4, x4, x3
	mov	x5, #1
	mov	x5, #1
	mov	x6, x2
	add	x6, x6, x5
	mov	x2, x6
	mov	x5, #1
	mov	x6, x2
	sub	x6, x6, x5
	mov	x16, #1
	mov	x0, x6
	svc	#0x80
