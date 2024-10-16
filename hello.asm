  .text
  .global main
main:
  addi  sp, sp, -0
  lw    t0, 0
  bnez t0, then0
  j else1
then0:
  li    a0, 1
  addi  sp, sp,  0
  ret
  j ifend2
else1:
  li    a0, 2
  addi  sp, sp,  0
  ret
  j ifend2
ifend2:
  li    a0, 3
  addi  sp, sp,  0
  ret
