ENTRY(_start)
SECTIONS
{
    . = 0x80000;
    .text :
    {
        *(.text._start)
        *(.text)
    }
    . = ALIGN(4096);
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(4096);
    .data :
    {
        *(.data)
    }
    . = ALIGN(4096);
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    .ARM.exidx :
    {
        *(.ARM.exidx*)
    }
    . = ALIGN(4096); 
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;


    . = ALIGN(4096);
    _stack_start = .;
    . = . + 0x10000;
}
