  .text
  .global main
main:
  addi  sp, sp, -48
  li    t0, 1
  li    t1, 0
  xor   t0, t0, t1
  snez  t0, t0
  sw    t0, 0(sp)
  lw    t0, 0(sp)
  bnez t0, then_block
  j else
then_block:
  li    t0, 1
  sw    t0, 4(sp)
  j ifend
else:
  li    t0, 2
  li    t1, 0
  xor   t0, t0, t1
  seqz  t0, t0
  sw    t0, 8(sp)
  lw    t0, 8(sp)
  bnez t0, then_block
  j else
  lw    t0, 36(sp)
  sw    t0, 4(sp)
  j ifend
ifend:
  lw    t0, 4(sp)
  sw    t0, 16(sp)
  lw    t0, 16(sp)
  sw    t0, 20(sp)
  lw    t0, 20(sp)
  sw    t0, 24(sp)
  lw    a0, 24(sp)
  addi  sp, sp,  48
  ret
then_block:
  sw    x0, 12(sp)
  j ifend
else:
  li    t0, 3
  li    t1, 0
  xor   t0, t0, t1
  snez  t0, t0
  sw    t0, 28(sp)
  lw    t0, 28(sp)
  sw    t0, 12(sp)
  j ifend
ifend:
  lw    t0, 12(sp)
  sw    t0, 32(sp)
  lw    t0, 32(sp)
  li    t1, 0
  xor   t0, t0, t1
  snez  t0, t0
  sw    t0, 36(sp)
