  .text
  .global main
main:
  addi  sp, sp, -32
  sw    x0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 4(sp)
  lw    t0, 4(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 8(sp)
  lw    t0, 8(sp)
  bnez t0, while_body
  j while-end
while_body:
  lw    t0, 0(sp)
  sw    t0, 12(sp)
  lw    t0, 12(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 16(sp)
  lw    t0, 16(sp)
  sw    t0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 20(sp)
  lw    t0, 20(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 24(sp)
  lw    t0, 24(sp)
  bnez t0, while_body
  j while-end
while-end:
  lw    t0, 0(sp)
  sw    t0, 28(sp)
  lw    a0, 28(sp)
  addi  sp, sp,  32
  ret
