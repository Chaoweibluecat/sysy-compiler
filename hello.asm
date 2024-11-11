.data
  .text
  .global main
main:
  addi  sp, sp, -16
  lw   t0 , 0(sp)
  add t0, sp, t0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 4(sp)
  li    t0, 1
  lw    t0, 4(sp)
  addi    t0, sp, t0
  sw    t0, 0(t0)
  lw   t0 , 0(sp)
  add t0, sp, t0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 8(sp)
  lw    t0, 8(sp)
  add   t0, t0, sp
  lw    t0, 0(t0)
  sw    t0, 12(sp)
  lw    a0, 12(sp)
  addi  sp, sp,  16
  ret

