OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x10000;

SECTIONS
{
  . = BASE_ADDRESS;
  .text : { 
      *(.text.entry) 
      *(.text .text.*) 
  }

  . = ALIGN(4K);
  .rodata : { 
      *(.rodata .rodata.*)
      *(.srodata .srodata.*)
  }

  . = ALIGN(4K);
  .data : { *(.data .data.*) *(.sdata .sdata.*) }

  . = ALIGN(4K);
  sbss = .;
  .bss : { 
      *(.bss .bss.*)
      *(.sbss .sbss.*)
  }
  ebss = .;

  /DISCARD/ : {
      *(.eh_frame)
      *(.debug*)
  }
}