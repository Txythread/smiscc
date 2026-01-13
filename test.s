    .align 2
    .global _start

_print:
    mov x1, #0
    ret

_stray:

LB0:
	sub	sp, sp, #0
	mov	x16, #1
	mov	x0, x0
	svc	#0x80
	add	sp, sp, #0

_start:
	sub	sp, sp, #16
	mov	x1, #0
	str	x0, [sp, #0]
	str	x1, [sp, #8]
	ldr	x0, [sp, #8]
	bl	LB0
	add	sp, sp, #16
