MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 8K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 4K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
