ENTRY(_start)

SECTIONS
{
    . = 0x80000;

    .text :
    {
        KEEP(*(.text._start))
        *(.text*)
    }

    .rodata : ALIGN(4096) { *(.rodata*) }
    .data : ALIGN(4096) { *(.data*) }


    .core1_main 0x90000 : {
        __core1_main_start = .;
        KEEP(*(.core1_main))
    }

    .core2_main 0xA0000 : {
        __core2_main_start = .;
        KEEP(*(.core2_main))
    }

    .core3_main 0xB0000 : {
        __core3_main_start = .;
        KEEP(*(.core3_main))
    }
    .bss : ALIGN(4096)
    {
        __bss_start = .;
        *(.bss*)
        __bss_end = .;
    }

    . = ALIGN(4096);
    _stack_start_0 = .;
    . = . + 1024 * 1024;
    _stack_start_1 = .;
    . = . + 1024 * 1024;
    _stack_start_2 = .;
    . = . + 1024 * 1024;
    _stack_start_3 = .;
    . = . + 1024 * 1024;
}
