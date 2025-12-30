    .align 2
    .global _start

_start:
	mov	x0, #1
	mov	x1, #2		; x1 = 2
	add	x1, x1, x0	; x1 = 3
	mov	x2, #2
	sdiv	x2, x2, x1
	mov	x3, #1
	mov	x3, #1 		; x3 = 1
	add	x3, x3, x1	; x3 = 4
	mov	x1, x3		; x1 = x3 = 4
	mov	x3, x1		; x3 = x1 = 4
	mov	x3, #1		; x3 = 1
	sub	x3, x3, x1	; x3 = 1 - 4 = -3 (unsigned u8: 252)
	mov	x16, #1
	mov	x0, x3		; exit 252
	svc	#0x80
