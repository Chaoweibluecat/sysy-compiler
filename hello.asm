.data
  .globl var
var:
  .zero 4
  .text
  .global main
main:
  addi  sp, sp, -16
  la    t1, var
  lw    t0, 0(t1)
  sw    t0, 0(sp)
  lw    t0, 0(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 4(sp)
  lw    a0, 4(sp)
  addi  sp, sp,  16
  ret

