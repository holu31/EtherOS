ENTRY(_start)

SECTIONS
{
    . = 0x00200000;

    _kernel_start = .;
    
    .text ALIGN (0x1000) :
    {
        *(.boot)
        *(.text)
    }

    .data ALIGN (0x1000) :
    {
        *(.data)
    }

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
