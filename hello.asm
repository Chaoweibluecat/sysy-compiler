  .data
  .globl a
a:
  .zero 1600

  .text
  .global main
main:
  addi  sp, sp, 32
  la    t0, a
  li    t1, 1
  li   t2 , 800
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 0(sp)
  lw   t0 , 0(sp)
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 4(sp)
  la    t0, a
  li    t1, 0
  li   t2 , 800
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 8(sp)
  lw   t0 , 8(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 12(sp)
  lw    t0, 12(sp)
  lw    t0, 0(t0)
  sw   t0 , 16(sp)
  lw   t0 , 16(sp)
  lw    t1, 4(sp)
  sw    t0, 0(t1)
  li    a0, 1
  addi  sp, sp, 32
  ret

