    .align 2
    .global _start

_stray:

_start:
	sub	sp, sp, #16
	mov	x0, #0
	str	x0, [sp, #0]
	ldr	x0, [sp, #0]
	bl	LB0
	add	sp, sp, #16

LB0:
	sub	sp, sp, #0
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
	add	sp, sp, #0
