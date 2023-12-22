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

	_data_end = .;

    .rodata ALIGN (0x1000) :
    {
        *(.rodata*)
    }

	_rodata_end = .;

    .bss :
    {
        *(COMMON)
        *(.bss .bss.*)
    }

	/DISCARD/ : {
		*(.note.gnu.build-id)
        *(.comment)
		*(.eh_frame)
		*(.note .note.*)
	}

    _kernel_end = .;
}
