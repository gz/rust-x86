/*PHDRS
{
    test PT_LOAD FLAGS(0x7); /* rwx 0b111 *//*
}*/

SECTIONS
{
  .kvmtest :
  {
    kvmtest_begin_loc = .;
    *(.kvmtest*)
    kvmtest_end_loc = .;
  } /* :test */
}

INSERT AFTER .rodata; /* do not override default script */
