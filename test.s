    .align 2
    .global _start

_stray:

_start:
	sub	sp, sp, #0
	mov	x0, #5
	mov	x1, #1
	mul	x0, x0, x1
	mov	x2, #6
	mov	x3, x0
	cmp	x3, x2
	cset	x3, gt
	mov	x4, x3
	mov	x3, x0
	cmp	x3, x2
	cset	x3, gt
	mov	x0, #0
	cmp	x3, x0
	beq	LB2

LB0:
	mov	x0, #0
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
	b	LB3

LB2:

LB3:
	mov	x0, #1
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
	add	sp, sp, #0
