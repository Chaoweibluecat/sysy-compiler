  .text
  .global main
main:
  addi  sp, sp, -48
  li    t0, 1
  sw    t0, 0(sp)
  j while-entry
while-entry:
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
  slt   t0, t0, t1
  sw    t0, 16(sp)
  lw    t0, 16(sp)
  bnez t0, then
  j else
while-end:
  li    a0, 0
  addi  sp, sp,  48
  ret
then:
  lw    t0, 0(sp)
  sw    t0, 20(sp)
  lw    t0, 20(sp)
  li    t1, 2
  add   t0, t0, t1
  sw    t0, 24(sp)
  lw    t0, 24(sp)
  sw    t0, 0(sp)
  j if-end
else:
  lw    t0, 0(sp)
  sw    t0, 28(sp)
  lw    t0, 28(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 32(sp)
  lw    t0, 32(sp)
  sw    t0, 0(sp)
  j while-entry
  j if-end
if-end:
  j while-entry
