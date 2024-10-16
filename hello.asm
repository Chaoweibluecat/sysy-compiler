  .text
  .global main
main:
  addi  sp, sp, -16
  sw    x0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 4(sp)
  lw    t0, 4(sp)
  bnez t0, then0
  j else1
then0:
  li    a0, 1
  addi  sp, sp,  16
  ret
else1:
  li    a0, 2
  addi  sp, sp,  16
  ret
  li    a0, 3
  addi  sp, sp,  16
  ret
  j ifend2
  j ifend2
