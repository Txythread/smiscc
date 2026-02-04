    .align 2
    .global _start

_stray:

_start:
	sub	sp, sp, #0
	mov	x0, #6
	mov	x1, #0
	mov	x2, x3
	cmp	x2, x0
	cset	x2, gt
	mov	x0, #0
	cmp	x2, x0
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
