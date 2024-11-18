.data
  .text
  .global f
f:
  addi  sp, sp, -16
  sw    a0, 0(sp)
  lw    t0, 0(sp)
  sw    t0, 4(sp)
  lw   t0 , 4(sp)
  add t0, sp, t0
  li    t1, 1
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

  .text
  .global main
main:
  addi  sp, sp, -32
    sw  ra, 28(sp)
  lw   t0 , 0(sp)
  add t0, sp, t0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 4(sp)
  li    t0, 1
  lw    t0, 4(sp)
  addi  t0, sp, t0
  sw    t0, 0(t0)
  lw   t0 , 0(sp)
  add t0, sp, t0
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 8(sp)
  li    t0, 2
  lw    t0, 8(sp)
  addi  t0, sp, t0
  sw    t0, 0(t0)
  lw   t0 , 0(sp)
  add t0, sp, t0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 12(sp)
  lw    a0, 12(sp)
  call  @f
  sw    a0, 16(sp)
  lw    a0, 16(sp)
    lw  ra, 28(sp)
  addi  sp, sp,  32
  ret

