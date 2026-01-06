    .align 2
    .global _start

_start:
	mov	x0, #0
	mov	x16, #1
	svc	#0x80
