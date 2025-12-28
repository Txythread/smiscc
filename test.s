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
	mov	x3, 2
	mov	x5, x4
	sdiv	x5, x5, x3
	mov	x3, 1
	mov	x5, x4
	add	x5, x5, x3
	mov	x4, x5

	mov x0, #0
	mov x16, #1
	svc #0
