  .text
  .global main
main:
  addi  sp, sp, -16
  sw    x0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 4(sp)
  lw    t0, 4(sp)
  bnez t0, then
  j else
then:
  li    a0, 1
  addi  sp, sp,  16
  ret
  j if-end
else:
  j if-end
if-end:
  li    a0, 2
  addi  sp, sp,  16
  ret
