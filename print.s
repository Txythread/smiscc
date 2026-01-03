	.global _start
	.align 2
	.text

_start:
	mov	x0, #1
	adrp	x1, msg@PAGE
	add 	x1, x1, msg@PAGEOFF
	mov 	x2, #14
	mov 	x16, #4
	svc	#0x80

	mov 	x0, x1
	mov 	x16, #1
	svc 	#0x80




	.data
msg:
	.asciz "Hello, world!\n"
