  .text
  .global main
main:
  addi  sp, sp, -144
  li    t0, 1
  sw    t0, 0(sp)
  li    t0, 2
  sw    t0, 4(sp)
  j while_entry0
while_entry0:
  lw    t0, 0(sp)
  sw    t0, 8(sp)
  lw    t0, 8(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 12(sp)
  lw    t0, 12(sp)
  bnez t0, while_body1
  j while_end2
while_body1:
  lw    t0, 0(sp)
  sw    t0, 16(sp)
  lw    t0, 16(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 20(sp)
  lw    t0, 20(sp)
  sw    t0, 0(sp)
  j while_entry3
while_end2:
  lw    t0, 0(sp)
  sw    t0, 24(sp)
  lw    t0, 4(sp)
  sw    t0, 28(sp)
  lw    t0, 24(sp)
  lw    t1, 28(sp)
  add   t0, t0, t1
  sw    t0, 32(sp)
  lw    a0, 32(sp)
  addi  sp, sp,  144
  ret
while_entry3:
  lw    t0, 0(sp)
  sw    t0, 36(sp)
  lw    t0, 36(sp)
  li    t1, 5
  slt   t0, t0, t1
  sw    t0, 40(sp)
  lw    t0, 40(sp)
  li    t1, 0
  xor   t0, t0, t1
  seqz  t0, t0
  sw    t0, 44(sp)
  lw    t0, 44(sp)
  bnez t0, then_block4
  j else5
  lw    t0, 4(sp)
  sw    t0, 52(sp)
  lw    t0, 52(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 56(sp)
  lw    t0, 56(sp)
  sw    t0, 4(sp)
  j while_entry3
  j while_entry6
then_block4:
  sw    x0, 48(sp)
  j ifend7
else5:
  lw    t0, 4(sp)
  sw    t0, 60(sp)
  lw    t0, 60(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 64(sp)
  lw    t0, 64(sp)
  li    t1, 0
  xor   t0, t0, t1
  snez  t0, t0
  sw    t0, 68(sp)
  lw    t0, 68(sp)
  sw    t0, 48(sp)
  j ifend7
ifend7:
  lw    t0, 48(sp)
  sw    t0, 72(sp)
  lw    t0, 72(sp)
  bnez t0, while_body8
  j while_end9
while_entry6:
  lw    t0, 4(sp)
  sw    t0, 76(sp)
  lw    t0, 76(sp)
  li    t1, 20
  slt   t0, t0, t1
  sw    t0, 80(sp)
  lw    t0, 80(sp)
  bnez t0, while_body10
  j while_end11
while_body10:
  j while_entry12
while_end11:
  j while_entry0
while_entry12:
  lw    t0, 4(sp)
  sw    t0, 84(sp)
  lw    t0, 84(sp)
  li    t1, 6
  slt   t0, t0, t1
  sw    t0, 88(sp)
  lw    t0, 88(sp)
  li    t1, 0
  xor   t0, t0, t1
  snez  t0, t0
  sw    t0, 92(sp)
  lw    t0, 92(sp)
  bnez t0, then_block13
  j else14
  lw    t0, 4(sp)
  sw    t0, 100(sp)
  lw    t0, 100(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 104(sp)
  lw    t0, 104(sp)
  sw    t0, 4(sp)
  j while_entry12
  lw    t0, 4(sp)
  sw    t0, 108(sp)
  lw    t0, 108(sp)
  li    t1, 2
  add   t0, t0, t1
  sw    t0, 112(sp)
  lw    t0, 112(sp)
  sw    t0, 4(sp)
  j while_entry6
then_block13:
  li    t0, 1
  sw    t0, 96(sp)
  j ifend15
else14:
  lw    t0, 4(sp)
  sw    t0, 116(sp)
  lw    t0, 116(sp)
  li    t1, 6
  xor   t0, t0, t1
  seqz  t0, t0
  sw    t0, 120(sp)
  lw    t0, 120(sp)
  li    t1, 0
  xor   t0, t0, t1
  snez  t0, t0
  sw    t0, 124(sp)
  lw    t0, 124(sp)
  sw    t0, 96(sp)
  j ifend15
ifend15:
  lw    t0, 96(sp)
  sw    t0, 128(sp)
  lw    t0, 128(sp)
  bnez t0, while_body16
  j while_end17
