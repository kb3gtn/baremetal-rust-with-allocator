/* Memory Layout Definition of Target */
MEMORY {
    ROM (rx)     : ORIGIN = 0x00000000, LENGTH = 16K     /* RTG4 UPROM is 16K in side */
    RAM (rwx)    : ORIGIN = 0x00008000, LENGTH = 16M-4K  /* SRAM is 16MB in size */
    STACK (rw)   : ORIGIN = 0x00FFF000, LENGTH = 4K      /* define stack area */ 
}

__stack_start = ORIGIN(RAM) + LENGTH(RAM);

REGION_ALIAS("REGION_TEXT", ROM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_STACK", STACK);
REGION_ALIAS("REGION_RODATA", ROM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_BSS", RAM);
