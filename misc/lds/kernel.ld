OUTPUT_ARCH( "riscv" )
ENTRY( _entry )

SECTIONS
{
  . = 0x80000000;


  /* 
   * Dedicated section for initialization code 
   * This was created as there was a bug
   * where when `panic_handler` actually used
   * `panic_info` (it's argument) the function body
   * would get inserted to the top of .text
   *
   * */
  .text.init : {
    *(.text.init .text.init.*)
    . = ALIGN(0x1000); 
  }

  .text : {
    PROVIDE(stext = .);
    *(.text .text.*)
    . = ALIGN(0x1000);
    PROVIDE(etext = .);
  }

  .rodata : {
    . = ALIGN(16);
    *(.srodata .srodata.*) /* do not need to distinguish this from .rodata */
    . = ALIGN(16);
    *(.rodata .rodata.*)
  }

  .data : {
    . = ALIGN(16);
    *(.sdata .sdata.*) /* do not need to distinguish this from .data */
    . = ALIGN(16);
    *(.data .data.*)
  }

  .bss : {
    . = ALIGN(16);
    *(.sbss .sbss.*) /* do not need to distinguish this from .bss */
    . = ALIGN(16);
    *(.bss .bss.*)
  }
  PROVIDE(__global_pointer = .);
  PROVIDE(__heap_start = .);
  . = 0x88000000;
  PROVIDE(__kernel_stack_start = . - 0x80000);
  PROVIDE(__kernel_stack_end = .);
  PROVIDE(__kernel_stack_size = __kernel_stack_end - __kernel_stack_start);
  PROVIDE(__heap_size = __kernel_stack_start - __heap_start);
  PROVIDE(end = .);
}
