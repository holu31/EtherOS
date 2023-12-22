ENTRY(_start)

SECTIONS
{
    . = 0x00200000;

    _kernel_start = .;

    .text ALIGN (0x1000) : 
    {
        *(.boot)
        *(.text*)
    }

	_text_end = .;
	
    .data ALIGN (0x1000) :
    {
        *(.data*)
    }

    .rodata ALIGN (0x1000) :
    {
        *(.rodata*)
    }


	_data_end = .;
	
    .bss :
    {
        *(COMMON)
        *(.bss)
    }

	/DISCARD/ : {
		*(.note.gnu.build-id)
        *(.comment)
		*(.eh_frame)
		*(.note .note.*)
	}

    _kernel_end = .;
}
