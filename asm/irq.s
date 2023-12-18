.extern     irq_handler

.macro IRQ irq_num, isr_num

.global _irq\irq_num

_irq\irq_num:
	cli
	push $0
	push $\isr_num
	jmp	irq_common_stub
.endm

IRQ 0, 32
IRQ 1, 33
IRQ 2, 34
IRQ 3, 35
IRQ 4, 36
IRQ 5, 37
IRQ 6, 38
IRQ 7, 39
IRQ 8, 40
IRQ 9, 41
IRQ 10, 42
IRQ 11, 43
IRQ 12, 44
IRQ 13, 45
IRQ 14, 46
IRQ 15, 47

irq_common_stub:
    pusha 
    push %ds
    mov $0x10, %ax
    mov %ax, %ds
    call irq_handler
    pop %ds      
    popa
    add $8, %esp    
    iret      