  .text
  .global main
main:
  addi  sp, -16
  li    t0, 10
  sw    t0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 4(sp)
  lw    t0, 4(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 8(sp)
  lw    t0, 8(sp)
  sw    t0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 12(sp)
  lw    a0, 12(sp)
  addi  sp, 16
  ret
