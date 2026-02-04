    .align 2
    .global _start

_print:
        mov     x2, x1
        mov     x1, x0
        mov     x0, #1
        mov     x16, #4
        svc     #0
        ret

_stray:

_start:
	sub	sp, sp, #48
	mov	x0, #6
	mov	x1, x2
	cmp	x1, x0
	cset	x1, gt
	mov	x3, x1
	mov	x4, #0
	str	x0, [sp, #0]
	str	x1, [sp, #8]
	str	x2, [sp, #16]
	str	x3, [sp, #24]
	str	x4, [sp, #32]
	ldr	x0, [sp, #32]
	bl	_println
	ldr	x1, [sp, #16]
	mov	x0, x1
	ldr	x1, [sp, #0]
	cmp	x0, x1
	cset	x0, gt
	mov	x1, #0
	cmp	x0, x1
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
	add	sp, sp, #48
