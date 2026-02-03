    .align 2
    .global _start

_stray:

_start:
	sub	sp, sp, #0
	mov	x0, #5
	mov	x1, #0
	mov	x2, #1
	mov	x3, #0
	cmp	x2, x3
	beq	LB2

LB0:
	mov	x2, #0
	mov	x16, #1
	mov	x0, x2
	svc	#0x80
	b	LB3

LB2:

LB3:
	mov	x2, #1
	mov	x16, #1
	mov	x0, x2
	svc	#0x80
