.data
  .globl x
x:
  .word 10
  .word 20

  .text
  .global main
main:
  addi  sp, sp, -0
  li    a0, 1
  addi  sp, sp,  0
  ret

