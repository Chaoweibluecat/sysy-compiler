.data
  .globl arr1
arr1:
  .word 1

  .text
  .global main
main:
  addi  sp, sp, -16
  la    t0, arr1
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 0(sp)
  lw    t0, 0(sp)
  addi  t0, sp, t0
  sw    x0, 0(t0)
  li    a0, 1
  addi  sp, sp,  16
  ret

