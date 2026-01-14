    .align 2
    .global _start

_stray:

LB0:
	sub	sp, sp, #16
	mov	x1, #0
	str	x0, [sp, #0]
	str	x1, [sp, #8]
	ldr	x0, [sp, #8]
	bl	LB1
	add	sp, sp, #16

LB1:
	sub	sp, sp, #0
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
	add	sp, sp, #0
