.data
  .text
  .global f1d
f1d:
  addi  sp, sp, -16
  sw    a0, 0(sp)
  addi  sp, sp,  16
  ret

  .text
  .global f2d
f2d:
  addi  sp, sp, -32
  sw    a0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 4(sp)
  lw   t0 , 4(sp)
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 8(sp)
  lw   t0 , 8(sp)
  li    t1, 2
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 12(sp)
  lw    t0, 12(sp)
  lw    t0, 0(t0)
  sw    t0, 16(sp)
  lw    a0, 16(sp)
  addi  sp, sp,  32
  ret
  addi  sp, sp,  32
  ret

  .text
  .global main
main:
  addi  sp, sp, -0
  li    a0, 33
  addi  sp, sp,  0
  ret

