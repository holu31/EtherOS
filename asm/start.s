.code32

.set ALIGN,						1<<0
.set MEMINFO,					1<<1
.set VBE_MODE,   				1<<2
.set FLAGS,						ALIGN | MEMINFO
.set MAGIC,						0x1BADB002
.set CHECKSUM,					-(MAGIC + FLAGS)

.extern kernel

.section .boot

.int MAGIC
.int FLAGS
.int CHECKSUM

.section .bss
.align 16
stack_bottom:
	.skip 1024 * 32  # 32 KB
stack_top:

.section	.text
.global		_start

_start:
	cli 

	# init FPU
	fninit
	fldcw (__fpu_control_word)

	mov $stack_top, %esp

	push %eax

	mov %cr0, %eax
	and $~0x04, %al
	or $0x22, %al
	mov %eax, %cr0
		
	mov %cr4, %eax
	or $0x600, %ax
	mov %eax, %cr4

	pop %eax

	push	%esp
	push	%ebx

	xor %ebp, %ebp

	call	kernel_main

	hlt
	jmp 1
1:
	jmp	1

__fpu_control_word:
		.word 0x37f
