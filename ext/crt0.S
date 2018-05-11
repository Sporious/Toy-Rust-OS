.section .traps, "ax"
.global __start
__start:
  b		__start_ram

.text
.type __start_ram, %function
__start_ram:
  mrs	x7, mpidr_el1
  and	x7, x7,
  cbz	x7, __start_master
  b		__hang

__start_master:
  # load stack pointer
  adrp	x2, __cpu0_stack_end
  add	x2, x2, #:lo12:__cpu0_stack_end
  mov	sp, x2

__clear_bss:
  ldr	w0, _bss_segment + 0
  ldr	w1, _bss_segment + 4
__clear:
  cbz	x1, __go_main
  str	xzr, [x0], #8
  sub	x1, x1, #1
  cbnz	x1, __clear
__go_main:
  # load main function
  bl	kmain
__hang:
  # wait forever in case of exit
  wfe
  b		__hang

# add section debug inf
.size	__start_ram, . - __start_ram

# uninitialised data
_bss_segment:
  .word	__bss_start
  .word	__bss_dwords
