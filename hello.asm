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
  addi  sp, sp, -16
    sw  ra, 12(sp)
  sw    a0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 4(sp)
  lw   t0 , 4(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 8(sp)
  lw    a0, 8(sp)
  call  f1d
    lw  ra, 12(sp)
  addi  sp, sp,  16
  ret

  .text
  .global f3d
f3d:
  addi  sp, sp, -16
  sw    a0, 0(sp)
  addi  sp, sp,  16
  ret

  .text
  .global main
main:
  addi  sp, sp, -32
    sw  ra, 28(sp)
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
  lw   t0 , 8(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 12(sp)
  li    t0, 1
  lw    t1, 12(sp)
  sw    t0, 0(t1)
