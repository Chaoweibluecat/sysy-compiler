  .text
  .global main
main:
  addi  sp, sp, -32
  sw    x0, 0(sp)
  li    t0, 1
  sw    t0, 4(sp)
  lw    t0, 0(sp)
  sw    t0, 8(sp)
  lw    t0, 8(sp)
  li    t1, 0
  xor   t0, t0, t1
  seqz  t0, t0
  sw    t0, 12(sp)
  lw    t0, 12(sp)
  bnez t0, then_block
  j else
then_block:
  sw    x0, 16(sp)
  j if-end
else:
  lw    t0, 4(sp)
  sw    t0, 20(sp)
  lw    t0, 20(sp)
  li    t1, 0
  xor   t0, t0, t1
  snez  t0, t0
  sw    t0, 24(sp)
  lw    t0, 24(sp)
  sw    t0, 16(sp)
  j if-end
if-end:
  lw    t0, 16(sp)
  sw    t0, 28(sp)
  lw    a0, 28(sp)
  addi  sp, sp,  32
  ret
