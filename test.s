    .align 2
    .global _start

_stray:

_start:
	sub	sp, sp, #32
	mov	x0, #5
	mov	x1, #6
	str	x0, [sp, #0]
	str	x1, [sp, #8]
	ldr	x0, [sp, #0]
	ldr	x1, [sp, #8]
	bl	LB0
	mov	x0, #0
	str	x0, [sp, #16]
	ldr	x0, [sp, #16]
	bl	LB1
	add	sp, sp, #32

LB0:
	sub	sp, sp, #0
	mov	x2, x0
	mov	x3, x1
	mov	x4, x0
	add	x4, x4, x1
	mov	x0, x2
	sub	x0, x0, x3
	add	sp, sp, #0

LB1:
	sub	sp, sp, #0
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
	add	sp, sp, #0
