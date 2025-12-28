    .align 2
    .global _start

_start:
	mov	x0, 1
	mov	x1, 2
	mov	x2, 3
	mov	x3, x1
	mul	x3, x3, x2
	mov	x4, x0
	add	x4, x4, x3
	mov	x5, 2
	mov	x6, x4
	sdiv	x6, x6, x5
	mov	x5, 1
	mov	x6, x4
	add	x6, x6, x5
	mov	x4, x6
	mov	x5, 1
	mov	x6, x4
	sub	x6, x6, x5
	mov	x16, #1
	mov	x0, x6
	svc	#0x80
