.data
  .text
  .global main
main:
  addi  sp, sp, -16
li ,  t0 , 0
  li    t1, 0
sw ,  t2 , 4
mul t1, t1, t2
add t0, t0, t1
sw , t0, 4
  li    t0, 2
  lw    t1, 4(sp)
  sw    t0, 0(t1)
li ,  t0 , 0
  li    t1, 0
sw ,  t2 , 4
mul t1, t1, t2
add t0, t0, t1
sw , t0, 8
  lw    t0, 8(sp)
  add   t0, t0, sp
  lw    t0, 0(t0)
  sw    t0, 12(sp)
  lw    a0, 12(sp)
  addi  sp, sp,  16
  ret

