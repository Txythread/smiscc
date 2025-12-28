.align 2
.global _start

_start:
	mov	x0, 1
	mov	x1, 1
	mov	x2, x0
	add	x2, x2, x1
	mov	x3, 1
	mov	x4, x2
	add	x4, x4, x3
	mov	x5, 1
	mov	x6, x4
	add	x6, x6, x5
	mov	x7, 11
	mov	x8, x6
	add	x8, x8, x7
	mov	x9, 1
	mov	x10, x8
	add	x10, x10, x9
	mov	x11, 1
	mov	x12, x10
	add	x12, x12, x11
	mov	x13, 1
	mov	x14, x12
	add	x14, x14, x13
	mov	x15, 1
	mov	x19, x14
	add	x19, x19, x15
	mov	x20, 1
	mov	x21, x19
	add	x21, x21, x20
	mov	x22, 1
	mov	x23, x21
	add	x23, x23, x22
	mov	x24, 1
	mov	x25, x23
	add	x25, x25, x24
	mov	x26, 1
	mov	x27, x25
	add	x27, x27, x26
	mov	x28, 1
	str	x0, [sp, #0]
	mov	x0, x27
	add	x0, x0, x28
	str	x0, [sp, #8]
	mov	x0, 1
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	ldr	x1, [sp, #8]
	mov	x0, x1
	ldr	x1, [sp, #16]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #32]
	mov	x1, x0
	str	x0, [sp, #40]
	ldr	x0, [sp, #32]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #48]
	mov	x0, x1
	str	x1, [sp, #56]
	ldr	x1, [sp, #48]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #64]
	mov	x1, x0
	str	x0, [sp, #72]
	ldr	x0, [sp, #64]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #80]
	mov	x0, x1
	str	x1, [sp, #88]
	ldr	x1, [sp, #80]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #96]
	mov	x1, x0
	str	x0, [sp, #104]
	ldr	x0, [sp, #96]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #112]
	mov	x0, x1
	ldr	x1, [sp, #112]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #120]
	mov	x1, x0
	ldr	x0, [sp, #120]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #128]
	mov	x0, x1
	ldr	x1, [sp, #128]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #136]
	mov	x1, x0
	ldr	x0, [sp, #136]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #144]
	mov	x0, x1
	ldr	x1, [sp, #144]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #152]
	mov	x1, x0
	ldr	x0, [sp, #152]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #160]
	mov	x0, x1
	ldr	x1, [sp, #160]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #168]
	mov	x1, x0
	ldr	x0, [sp, #168]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #176]
	mov	x0, x1
	ldr	x1, [sp, #176]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #184]
	mov	x1, x0
	ldr	x0, [sp, #184]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #192]
	mov	x0, x1
	ldr	x1, [sp, #192]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #200]
	mov	x1, x0
	ldr	x0, [sp, #200]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #208]
	mov	x0, x1
	ldr	x1, [sp, #208]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #216]
	mov	x1, x0
	ldr	x0, [sp, #216]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #224]
	mov	x0, x1
	ldr	x1, [sp, #224]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #232]
	mov	x1, x0
	ldr	x0, [sp, #232]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #240]
	mov	x0, x1
	ldr	x1, [sp, #240]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #248]
	mov	x1, x0
	ldr	x0, [sp, #248]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #256]
	mov	x0, x1
	ldr	x1, [sp, #256]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #264]
	mov	x1, x0
	ldr	x0, [sp, #264]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #272]
	mov	x0, x1
	ldr	x1, [sp, #272]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #280]
	mov	x1, x0
	ldr	x0, [sp, #280]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #288]
	mov	x0, x1
	ldr	x1, [sp, #288]
	add	x0, x0, x1
	mov	x1, 1
	str	x1, [sp, #296]
	mov	x1, x0
	ldr	x0, [sp, #296]
	add	x1, x1, x0
	mov	x0, 1
	str	x0, [sp, #304]
	mov	x0, x1
	ldr	x1, [sp, #304]
	add	x0, x0, x1
	mov	x0, 1
	mov	x1, 2
	str	x0, [sp, #312]
	mov	x0, 3
	str	x0, [sp, #320]
	mov	x0, x1
	ldr	x1, [sp, #320]
	mul	x0, x0, x1
	str	x0, [sp, #328]
	ldr	x0, [sp, #312]
	mov	x1, x0
	ldr	x0, [sp, #328]
	add	x1, x1, x0
	mov	x0, 2
	str	x0, [sp, #336]
	mov	x0, x1
	ldr	x1, [sp, #336]
	sdiv	x0, x0, x1

	mov x0, #0
	mov x16, #1
	svc #0
