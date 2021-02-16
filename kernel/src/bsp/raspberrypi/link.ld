SECTIONS
{
    /*
    ** Initial PC - start here
    */
    . = 0x80000;

    .text :
    {
        *(.text._start) *(.text)
    }

    .rodata :
    {
        *(.rodata*)
    }

    .data :
    {
        *(.data*)
    }

    /*
    ** BSS is zero'd in u64 hunks,
    ** start & end must be aligned on 8 byte boundaries.
    */
    .bss ALIGN(8) :
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(8);

        /*
        ** Handle the case where bss == 0, ensuring that
        ** __bss_start <= __bss_end_inclusive remains true.
        */
        . += 8;
        __bss_end_inclusive = . - 8;
    }

    /DISCARD/ : { *(.comments) }
}