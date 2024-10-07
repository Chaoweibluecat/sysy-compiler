  .text
  .global main
main:
  li    t0, 1
  li    t1, 2
  slt   t2, t0, t1
  seqz  t2, t2
  li    t1, 1
  and   t3, t2, t1
  snez  t3, t3
  li    t0, 2
  li    t1, 1
  sgt   t4, t0, t1
  or    t5, t3, t4
  snez  t5, t5
  mv a0, t5
  ret
