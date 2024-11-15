.data
  .globl arr1
arr1:
  .word 1

  .text
  .global get_first
get_first:
  addi  sp, sp, -32
  sw    a0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 4(sp)
  lw   t0 , 4(sp)
  add t0, sp, t0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 8(sp)
