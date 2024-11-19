.data
  .text
  .global main
main:
  addi  sp, sp, -32
  addi t0, sp, 0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 4(sp)
  lw   t0 , 4(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 8(sp)
  li    t0, 1
  lw    t1, 8(sp)
  sw    t0, 0(t1)
  addi t0, sp, 0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 12(sp)
  lw   t0 , 12(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 16(sp)
  lw    t0, 16(sp)
  lw    t0, 0(t0)
  sw    t0, 20(sp)
  lw    a0, 20(sp)
  addi  sp, sp,  32
  ret

