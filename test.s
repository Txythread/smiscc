    .align 2
    .global _start

_stray:

_start:
	sub	sp, sp, #0
	add	sp, sp, #0

LB0:
	sub	sp, sp, #0
	mov	x2, x0
	mov	x3, x1
	mov	x4, x0
	add	x4, x4, x1
	mov	x1, x2
	sub	x1, x1, x3
	add	sp, sp, #0

LB1:
	mov	x1, #0
	cmp	x0, x1
	bne	LB4

LB2:
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
	b	LB5

LB4:

LB5:
	sub	sp, sp, #0
	mov	x0, #1
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
	add	sp, sp, #0
