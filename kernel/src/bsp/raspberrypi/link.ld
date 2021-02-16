SECTIONS
{
    /* Initial PC - start here */
    . = 0x80000;

    .text :
    {
        *(.text._start) *(.text)
    }

    /DISCARD/ : { *(.comments) }
}