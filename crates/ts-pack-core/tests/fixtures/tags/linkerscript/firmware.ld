ENTRY(_start)

MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 256K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
}

_stack_size = 0x400;

SECTIONS
{
  .text :
  {
    *(.text*)
    . = ALIGN(4);
    _etext = .;
    PROVIDE(_heap_start = _etext);
  } > FLASH

  .data :
  {
    *(.data*)
  } > RAM AT > FLASH
}
