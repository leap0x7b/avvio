ENTRY(_start);

MEMORY {
    ram (wxa!ri) : ORIGIN = 0x80000000, LENGTH = 128M
}
 
PHDRS {
    text PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}
 
SECTIONS {
	/* Include entry point at start of binary */
	.text : {
	    PROVIDE(_text_start = .);
		*(.text.init) *(.text .text.*)
		PROVIDE(_text_end = .);
	} >ram AT>ram :text

	PROVIDE(_global_pointer = .);

    .rodata : {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    } >ram AT>ram :text

    .data : ALIGN(0x1000) {
        PROVIDE(_data_start = .);
        *(.sdata .sdata.*) *(.data .data.*)
        PROVIDE(_data_end = .);
    } >ram AT>ram :data

    .bss : {
        PROVIDE(_bss_start = .);
        *(.sbss .sbss.*) *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    } >ram AT>ram :bss

    PROVIDE(_memory_start = ORIGIN(ram));
    PROVIDE(_stack = _bss_end + 0x80000);
    PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));

    PROVIDE(_heap_start = _stack);
    PROVIDE(_heap_size = _memory_end - _stack);
}
