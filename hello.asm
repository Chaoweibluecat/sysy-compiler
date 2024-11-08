.data
  .text
  .global main
main:
  addi  sp, sp, -32
  li   t0 , 0
  li    t1, 0
  lw   t2 , 8
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 4(sp)
  lw   t0 , 4(sp)
  addi t0, sp, t0
  li    t1, 0
  lw   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 8(sp)
  li    t0, 1
  lw    t1, 8(sp)
  sw    t0, 0(t1)
  li   t0 , 0
  li    t1, 0
  lw   t2 , 8
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 12(sp)
  lw   t0 , 12(sp)
  addi t0, sp, t0
  li    t1, 1
  lw   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 16(sp)
  li    t0, 2
  lw    t1, 16(sp)
  sw    t0, 0(t1)
  li   t0 , 0
  li    t1, 0
  lw   t2 , 8
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 20(sp)
  lw   t0 , 20(sp)
  addi t0, sp, t0
  li    t1, 1
  lw   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 24(sp)
  lw    t0, 24(sp)
  add   t0, t0, sp
  lw    t0, 0(t0)
  sw    t0, 28(sp)
  lw    a0, 28(sp)
  addi  sp, sp,  32
  ret

