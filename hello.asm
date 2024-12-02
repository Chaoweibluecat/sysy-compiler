  .data
  .globl arr
arr:
  .zero 28

  .text
  .global test
test:
  addi  sp, sp, -16
  sw   a0 , 0(sp)
  li    a0, 1
  addi  sp, sp, 16
  ret

  .text
  .global main
main:
  addi  sp, sp, -16
  sw   ra , 12(sp)
  la    t0, arr
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 0(sp)
  lw   a0 , 0(sp)
  call  test
  sw   a0 , 4(sp)
  lw   a0 , 4(sp)
  lw   ra , 12(sp)
  addi  sp, sp, 16
  ret

