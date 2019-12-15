OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

PHDRS {
  text   PT_LOAD FLAGS(5);
  rodata PT_LOAD FLAGS(4);
  data   PT_LOAD FLAGS(6);
}

SECTIONS {
  /* The address where the payload should be loaded to. */
  PROVIDE(__payload_start__     = 0x40010000);
  /* The start of the execution stack. */
  PROVIDE(__stack_top__         = 0x40010000);
  /* Some address in upper IRAM where the relocator can be safely placed. */
  PROVIDE(__relocator_address__ = 0x4003F000);

  . = __payload_start__;

  /* App code. */
  .text : {
    . = ALIGN(16);
    KEEP(*(.text.crt0))
    *(.text .text.*)
    . = ALIGN(8);
  } :text

  /* App rodata. */
  .rodata : {
    *(.roda)
    *(.rodata .rodata.*)
    SORT(CONSTRUCTORS)
    . = ALIGN(8);
  } :rodata

  /* App data. */
  .data : {
    *(.data .data.*)
    CONSTRUCTORS
    . = ALIGN(16);
  } :data

  /* Payload ends here as the BSS shouldn't be copied to memory. */
  . = ALIGN(16);
  PROVIDE(__payload_end__ = ABSOLUTE(.));

  /* App BSS. */
  .bss (NOLOAD) : {
    . = ALIGN(16);
    PROVIDE(__bss_start__ = ABSOLUTE(.));
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(16);
    PROVIDE(__bss_end__ = ABSOLUTE(.));
  } :NONE

  /DISCARD/ : {
    *(.interp)
  }
}
