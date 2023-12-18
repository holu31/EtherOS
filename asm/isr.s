.extern     isr_handler

.macro ISR_NOERR isr_num

.global	_isr\isr_num

_isr\isr_num:
	cli
	push $0	
	push $\isr_num
	jmp	isr_common_stub_noerr
.endm
	
.macro ISR_ERR isr_num

.global	_isr\isr_num
_isr\isr_num:
	cli
	push $\isr_num
	jmp	isr_common_stub_err
.endm

ISR_NOERR 0
ISR_NOERR 1
ISR_NOERR 2
ISR_NOERR 3
ISR_NOERR 4
ISR_NOERR 5
ISR_NOERR 6
ISR_NOERR 7
ISR_ERR   8
ISR_NOERR 9
ISR_ERR   10
ISR_ERR   11
ISR_ERR   12
ISR_ERR   13
ISR_ERR   14
ISR_NOERR 15
ISR_NOERR 16
ISR_NOERR 17
ISR_NOERR 18
ISR_NOERR 19
ISR_NOERR 20
ISR_NOERR 21
ISR_NOERR 22
ISR_NOERR 23
ISR_NOERR 24
ISR_NOERR 25
ISR_NOERR 26
ISR_NOERR 27
ISR_NOERR 28
ISR_NOERR 29
ISR_NOERR 30
ISR_NOERR 31
ISR_NOERR 128 # Для сисколлов

isr_common_stub_err:
    pusha
    push %ds
    mov $0x10, %ax
	mov %ax, %ds      
	call isr_handler
	pop %ds
    popa
    add $8,%esp
    iret
      
isr_common_stub_noerr:
    pusha
	push %ds
    mov $0x10, %ax
    mov %ax, %ds
    call isr_handler
	pop %ds
    popa
	add $8, %esp
    iret