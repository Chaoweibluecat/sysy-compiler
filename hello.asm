  .text
  .global main
main:
  addi  sp, sp, -160
    sw  ra, 156(sp)
  addi t0, sp, 0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 40(sp)
  lw    a0, 40(sp)
  call  getarray
  sw    a0, 44(sp)
  lw    t0, 44(sp)
  sw    t0, 48(sp)
  sw    x0, 52(sp)
  j while_entry0
while_entry0:
  lw    t0, 52(sp)
  sw    t0, 56(sp)
  lw    t0, 56(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 60(sp)
  lw    t0, 60(sp)
  bnez t0, while_body1
  j while_end2
while_body1:
  lw    t0, 52(sp)
  sw    t0, 64(sp)
  lw    t0, 48(sp)
  sw    t0, 68(sp)
  lw    t0, 64(sp)
  lw    t1, 68(sp)
  slt   t0, t0, t1
  sw    t0, 72(sp)
  lw    t0, 72(sp)
  bnez t0, then3
  j else4
then3:
  lw    t0, 52(sp)
  sw    t0, 76(sp)
  addi t0, sp, 0
  lw   t1 , 76(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 80(sp)
  lw    t0, 52(sp)
  sw    t0, 84(sp)
  addi t0, sp, 0
  lw   t1 , 84(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 88(sp)
  lw    t0, 88(sp)
  lw    t0, 0(t0)
  sw    t0, 92(sp)
  lw    t0, 52(sp)
  sw    t0, 96(sp)
  lw    t0, 92(sp)
  lw    t1, 96(sp)
  add   t0, t0, t1
  sw    t0, 100(sp)
  lw    t0, 100(sp)
  lw    t1, 80(sp)
  sw    t0, 0(t1)
  j ifend5
else4:
  lw    t0, 52(sp)
  sw    t0, 104(sp)
  addi t0, sp, 0
  lw   t1 , 104(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 108(sp)
  lw    t0, 52(sp)
  sw    t0, 112(sp)
  lw    t0, 112(sp)
  li    t1, 1
  sub   t0, t0, t1
  sw    t0, 116(sp)
  addi t0, sp, 0
  lw   t1 , 116(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 120(sp)
  lw    t0, 120(sp)
  lw    t0, 0(t0)
  sw    t0, 124(sp)
  lw    t0, 52(sp)
  sw    t0, 128(sp)
  lw    t0, 124(sp)
  lw    t1, 128(sp)
  add   t0, t0, t1
  sw    t0, 132(sp)
  lw    t0, 132(sp)
  lw    t1, 108(sp)
  sw    t0, 0(t1)
  j ifend5
ifend5:
  lw    t0, 52(sp)
  sw    t0, 136(sp)
  lw    t0, 136(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 140(sp)
  lw    t0, 140(sp)
  sw    t0, 52(sp)
  j while_entry0
while_end2:
  lw    t0, 48(sp)
  sw    t0, 144(sp)
  addi t0, sp, 0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 148(sp)
  lw    a0, 144(sp)
  lw    a1, 148(sp)
  call  putarray
  li    a0, 0
    lw  ra, 156(sp)
  addi  sp, sp,  160
  ret

