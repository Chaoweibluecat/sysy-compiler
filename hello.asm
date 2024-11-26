.data
  .globl garr
garr:
  .word 6
  .word 7
  .word 8
  .word 9
  .word 10
  .word 11
  .word 12
  .word 13
  .word 14
  .word 15

  .text
  .global main
main:
  addi  sp, sp, -112
  addi t0, sp, 0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 4(sp)
  li    t0, 1
  lw    t1, 4(sp)
  sw    t0, 0(t1)
  addi t0, sp, 0
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 8(sp)
  li    t0, 2
  lw    t1, 8(sp)
  sw    t0, 0(t1)
  addi t0, sp, 0
  li    t1, 2
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 12(sp)
  li    t0, 3
  lw    t1, 12(sp)
  sw    t0, 0(t1)
  addi t0, sp, 0
  li    t1, 3
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 16(sp)
  li    t0, 4
  lw    t1, 16(sp)
  sw    t0, 0(t1)
  addi t0, sp, 0
  li    t1, 4
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 20(sp)
  li    t0, 5
  lw    t1, 20(sp)
  sw    t0, 0(t1)
  addi t0, sp, 0
  li    t1, 5
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 24(sp)
  lw    t1, 24(sp)
  sw    x0, 0(t1)
  addi t0, sp, 0
  li    t1, 6
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 28(sp)
  lw    t1, 28(sp)
  sw    x0, 0(t1)
  addi t0, sp, 0
  li    t1, 7
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 32(sp)
  lw    t1, 32(sp)
  sw    x0, 0(t1)
  addi t0, sp, 0
  li    t1, 8
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 36(sp)
  lw    t1, 36(sp)
  sw    x0, 0(t1)
  addi t0, sp, 0
  li    t1, 9
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 40(sp)
  lw    t1, 40(sp)
  sw    x0, 0(t1)
  sw    x0, 44(sp)
  sw    x0, 48(sp)
  j while_entry0
while_entry0:
  lw    t0, 44(sp)
  sw    t0, 52(sp)
  lw    t0, 52(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 56(sp)
  lw    t0, 56(sp)
  bnez t0, while_body1
  j while_end2
while_body1:
  lw    t0, 48(sp)
  sw    t0, 60(sp)
  lw    t0, 44(sp)
  sw    t0, 64(sp)
  addi t0, sp, 0
  lw ,  t1 , 64(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 68(sp)
  lw    t0, 68(sp)
  lw    t0, 0(t0)
  sw    t0, 72(sp)
  lw    t0, 60(sp)
  lw    t1, 72(sp)
  add   t0, t0, t1
  sw    t0, 76(sp)
  lw    t0, 44(sp)
  sw    t0, 80(sp)
  la    t0, garr
  lw ,  t1 , 80(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 84(sp)
  lw    t0, 84(sp)
  lw    t0, 0(t0)
  sw    t0, 88(sp)
  lw    t0, 76(sp)
  lw    t1, 88(sp)
  add   t0, t0, t1
  sw    t0, 92(sp)
  lw    t0, 92(sp)
  sw    t0, 48(sp)
  lw    t0, 44(sp)
  sw    t0, 96(sp)
  lw    t0, 96(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 100(sp)
  lw    t0, 100(sp)
  sw    t0, 44(sp)
  j while_entry0
while_end2:
  lw    t0, 48(sp)
  sw    t0, 104(sp)
  lw    a0, 104(sp)
  addi  sp, sp,  112
  ret

