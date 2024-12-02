  .data
  .globl n
n:
  .zero 4

  .text
  .global bubblesort
bubblesort:
  addi  sp, sp, -192
  sw   a0 , 0(sp)
  sw   x0 , 4(sp)
  j while_entry0
while_entry0:
  lw   t0 , 4(sp)
  sw   t0 , 12(sp)
  la    t0, n
  lw    t0, 0(t0)
  lw   t0 , 0(t0)
  sw   t0 , 16(sp)
  lw   t0 , 16(sp)
  li    t1, 1
  sub   t0, t0, t1
  sw   t0 , 20(sp)
  lw   t0 , 12(sp)
  lw   t1 , 20(sp)
  slt   t0, t0, t1
  sw   t0 , 24(sp)
  lw   t0 , 24(sp)
  bnez t0, while_body1
  j while_end2
while_body1:
  sw   x0 , 8(sp)
  j while_entry3
while_entry3:
  lw   t0 , 8(sp)
  sw   t0 , 28(sp)
  la    t0, n
  lw    t0, 0(t0)
  lw   t0 , 0(t0)
  sw   t0 , 32(sp)
  lw   t0 , 4(sp)
  sw   t0 , 36(sp)
  lw   t0 , 32(sp)
  lw   t1 , 36(sp)
  sub   t0, t0, t1
  sw   t0 , 40(sp)
  lw   t0 , 40(sp)
  li    t1, 1
  sub   t0, t0, t1
  sw   t0 , 44(sp)
  lw   t0 , 28(sp)
  lw   t1 , 44(sp)
  slt   t0, t0, t1
  sw   t0 , 48(sp)
  lw   t0 , 48(sp)
  bnez t0, while_body4
  j while_end5
while_body4:
  lw   t0 , 0(sp)
  sw   t0 , 52(sp)
  lw   t0 , 8(sp)
  sw   t0 , 56(sp)
  lw   t0 , 52(sp)
  lw   t1 , 56(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 60(sp)
  lw   t0 , 60(sp)
  lw   t0 , 0(t0)
  sw   t0 , 64(sp)
  lw   t0 , 0(sp)
  sw   t0 , 68(sp)
  lw   t0 , 8(sp)
  sw   t0 , 72(sp)
  lw   t0 , 72(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 76(sp)
  lw   t0 , 68(sp)
  lw   t1 , 76(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 80(sp)
  lw   t0 , 80(sp)
  lw   t0 , 0(t0)
  sw   t0 , 84(sp)
  lw   t0 , 64(sp)
  lw   t1 , 84(sp)
  sgt   t0, t0, t1
  sw   t0 , 88(sp)
  lw   t0 , 88(sp)
  bnez t0, then6
  j else7
then6:
  lw   t0 , 0(sp)
  sw   t0 , 96(sp)
  lw   t0 , 8(sp)
  sw   t0 , 100(sp)
  lw   t0 , 100(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 104(sp)
  lw   t0 , 96(sp)
  lw   t1 , 104(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 108(sp)
  lw   t0 , 108(sp)
  lw   t0 , 0(t0)
  sw   t0 , 112(sp)
  lw   t0 , 112(sp)
  sw   t0 , 92(sp)
  lw   t0 , 0(sp)
  sw   t0 , 116(sp)
  lw   t0 , 8(sp)
  sw   t0 , 120(sp)
  lw   t0 , 120(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 124(sp)
  lw   t0 , 116(sp)
  lw   t1 , 124(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 128(sp)
  lw   t0 , 0(sp)
  sw   t0 , 132(sp)
  lw   t0 , 8(sp)
  sw   t0 , 136(sp)
  lw   t0 , 132(sp)
  lw   t1 , 136(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 140(sp)
  lw   t0 , 140(sp)
  lw   t0 , 0(t0)
  sw   t0 , 144(sp)
  lw   t0 , 144(sp)
  lw   t1 , 128(sp)
  sw   t0 , 0(t1)
  lw   t0 , 0(sp)
  sw   t0 , 148(sp)
  lw   t0 , 8(sp)
  sw   t0 , 152(sp)
  lw   t0 , 148(sp)
  lw   t1 , 152(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 156(sp)
  lw   t0 , 92(sp)
  sw   t0 , 160(sp)
  lw   t0 , 160(sp)
  lw   t1 , 156(sp)
  sw   t0 , 0(t1)
  j ifend8
else7:
  j ifend8
ifend8:
  lw   t0 , 8(sp)
  sw   t0 , 164(sp)
  lw   t0 , 164(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 168(sp)
  lw   t0 , 168(sp)
  sw   t0 , 8(sp)
  j while_entry3
while_end5:
  lw   t0 , 4(sp)
  sw   t0 , 172(sp)
  lw   t0 , 172(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 176(sp)
  lw   t0 , 176(sp)
  sw   t0 , 4(sp)
  j while_entry0
while_end2:
  li    a0, 0
  addi  sp, sp, 192
  ret

  .text
  .global main
main:
  addi  sp, sp, -144
  sw   ra , 140(sp)
  li    t0, 10
  la    t1, n
  sw    t0, 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 40(sp)
  li    t0, 4
  lw   t1 , 40(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 44(sp)
  li    t0, 3
  lw   t1 , 44(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 2
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 48(sp)
  li    t0, 9
  lw   t1 , 48(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 3
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 52(sp)
  li    t0, 2
  lw   t1 , 52(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 4
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 56(sp)
  lw   t1 , 56(sp)
  sw   x0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 5
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 60(sp)
  li    t0, 1
  lw   t1 , 60(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 6
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 64(sp)
  li    t0, 6
  lw   t1 , 64(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 7
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 68(sp)
  li    t0, 5
  lw   t1 , 68(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 8
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 72(sp)
  li    t0, 7
  lw   t1 , 72(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 9
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 76(sp)
  li    t0, 8
  lw   t1 , 76(sp)
  sw   t0 , 0(t1)
  li    t0,  0
  add  t0 , sp , t0
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 84(sp)
  lw   a0 , 84(sp)
  call  bubblesort
  sw   a0 , 88(sp)
  lw   t0 , 88(sp)
  sw   t0 , 80(sp)
  j while_entry9
while_entry9:
  lw   t0 , 80(sp)
  sw   t0 , 92(sp)
  la    t0, n
  lw    t0, 0(t0)
  lw   t0 , 0(t0)
  sw   t0 , 96(sp)
  lw   t0 , 92(sp)
  lw   t1 , 96(sp)
  slt   t0, t0, t1
  sw   t0 , 100(sp)
  lw   t0 , 100(sp)
  bnez t0, while_body10
  j while_end11
while_body10:
  lw   t0 , 80(sp)
  sw   t0 , 108(sp)
  li    t0,  0
  add  t0 , sp , t0
  lw   t1 , 108(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 112(sp)
  lw   t0 , 112(sp)
  lw   t0 , 0(t0)
  sw   t0 , 116(sp)
  lw   t0 , 116(sp)
  sw   t0 , 104(sp)
  lw   t0 , 104(sp)
  sw   t0 , 120(sp)
  lw   a0 , 120(sp)
  call  putint
  li    t0, 10
  sw   t0 , 104(sp)
  lw   t0 , 104(sp)
  sw   t0 , 124(sp)
  lw   a0 , 124(sp)
  call  putch
  lw   t0 , 80(sp)
  sw   t0 , 128(sp)
  lw   t0 , 128(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 132(sp)
  lw   t0 , 132(sp)
  sw   t0 , 80(sp)
  j while_entry9
while_end11:
  li    a0, 0
  lw   ra , 140(sp)
  addi  sp, sp, 144
  ret

