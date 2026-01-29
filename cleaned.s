    .align 2
    .global _start

_start:
	mov	x0, #5
	bl	LB1

LB0:
	sub	x0, x0, x1
	ret

LB1:
	cmp	x0, #0
	bne	LB4
	mov	x16, #1
	svc	#0x80

LB4:
	mov	x0, #1
	mov	x16, #1
	svc	#0x80
