  .text
  .global init
init:
  addi  sp, sp, -128
  sw   a0 , 0(sp)
  sw   x0 , 4(sp)
  j while_entry0
while_entry0:
  lw   t0 , 4(sp)
  sw   t0 , 8(sp)
  lw   t0 , 8(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw   t0 , 12(sp)
  lw   t0 , 12(sp)
  bnez t0, while_body1
  j while_end2
while_body1:
  sw   x0 , 16(sp)
  j while_entry3
while_entry3:
  lw   t0 , 16(sp)
  sw   t0 , 20(sp)
  lw   t0 , 20(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw   t0 , 24(sp)
  lw   t0 , 24(sp)
  bnez t0, while_body4
  j while_end5
while_body4:
  sw   x0 , 28(sp)
  j while_entry6
while_entry6:
  lw   t0 , 28(sp)
  sw   t0 , 32(sp)
  lw   t0 , 32(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw   t0 , 36(sp)
  lw   t0 , 36(sp)
  bnez t0, while_body7
  j while_end8
while_body7:
  lw   t0 , 0(sp)
  sw   t0 , 40(sp)
  lw   t0 , 4(sp)
  sw   t0 , 44(sp)
  lw   t0 , 40(sp)
  lw   t1 , 44(sp)
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 48(sp)
  lw   t0 , 16(sp)
  sw   t0 , 52(sp)
  lw   t0 , 48(sp)
  lw   t1 , 52(sp)
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 56(sp)
  lw   t0 , 28(sp)
  sw   t0 , 60(sp)
  lw   t0 , 56(sp)
  lw   t1 , 60(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 64(sp)
  lw   t0 , 4(sp)
  sw   t0 , 68(sp)
  lw   t0 , 68(sp)
  li    t1, 100
  mul   t0, t0, t1
  sw   t0 , 72(sp)
  lw   t0 , 16(sp)
  sw   t0 , 76(sp)
  lw   t0 , 76(sp)
  li    t1, 10
  mul   t0, t0, t1
  sw   t0 , 80(sp)
  lw   t0 , 72(sp)
  lw   t1 , 80(sp)
  add   t0, t0, t1
  sw   t0 , 84(sp)
  lw   t0 , 28(sp)
  sw   t0 , 88(sp)
  lw   t0 , 84(sp)
  lw   t1 , 88(sp)
  add   t0, t0, t1
  sw   t0 , 92(sp)
  lw   t0 , 92(sp)
  lw   t1 , 64(sp)
  sw   t0 , 0(t1)
  lw   t0 , 28(sp)
  sw   t0 , 96(sp)
  lw   t0 , 96(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 100(sp)
  lw   t0 , 100(sp)
  sw   t0 , 28(sp)
  j while_entry6
while_end8:
  lw   t0 , 16(sp)
  sw   t0 , 104(sp)
  lw   t0 , 104(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 108(sp)
  lw   t0 , 108(sp)
  sw   t0 , 16(sp)
  j while_entry3
while_end5:
  lw   t0 , 4(sp)
  sw   t0 , 112(sp)
  lw   t0 , 112(sp)
  li    t1, 1
  add   t0, t0, t1
  sw   t0 , 116(sp)
  lw   t0 , 116(sp)
  sw   t0 , 4(sp)
  j while_entry0
while_end2:
  addi  sp, sp, 128
  ret

  .text
  .global f1
f1:
  addi  sp, sp, -208
  sw   a0 , 0(sp)
  sw   a1 , 4(sp)
  sw   a2 , 8(sp)
  sw   a3 , 12(sp)
  sw   a4 , 16(sp)
  sw   a5 , 20(sp)
  sw   a6 , 24(sp)
  sw   a7 , 28(sp)
  lw   t0 , 208(sp)
  sw   t0 , 32(sp)
  lw   t0 , 212(sp)
  sw   t0 , 36(sp)
  lw   t0 , 0(sp)
  sw   t0 , 40(sp)
  lw   t0 , 40(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 44(sp)
  lw   t0 , 44(sp)
  lw   t0 , 0(s0)
  sw   t0 , 48(sp)
  lw   t0 , 4(sp)
  sw   t0 , 52(sp)
  lw   t0 , 52(sp)
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 56(sp)
  lw   t0 , 56(sp)
  lw   t0 , 0(s0)
  sw   t0 , 60(sp)
  lw   t0 , 48(sp)
  lw   t1 , 60(sp)
  add   t0, t0, t1
  sw   t0 , 64(sp)
  lw   t0 , 8(sp)
  sw   t0 , 68(sp)
  lw   t0 , 68(sp)
  li    t1, 2
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 72(sp)
  lw   t0 , 72(sp)
  lw   t0 , 0(s0)
  sw   t0 , 76(sp)
  lw   t0 , 64(sp)
  lw   t1 , 76(sp)
  add   t0, t0, t1
  sw   t0 , 80(sp)
  lw   t0 , 12(sp)
  sw   t0 , 84(sp)
  lw   t0 , 84(sp)
  li    t1, 3
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 88(sp)
  lw   t0 , 88(sp)
  lw   t0 , 0(s0)
  sw   t0 , 92(sp)
  lw   t0 , 80(sp)
  lw   t1 , 92(sp)
  add   t0, t0, t1
  sw   t0 , 96(sp)
  lw   t0 , 16(sp)
  sw   t0 , 100(sp)
  lw   t0 , 100(sp)
  li    t1, 4
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 104(sp)
  lw   t0 , 104(sp)
  lw   t0 , 0(s0)
  sw   t0 , 108(sp)
  lw   t0 , 96(sp)
  lw   t1 , 108(sp)
  add   t0, t0, t1
  sw   t0 , 112(sp)
  lw   t0 , 20(sp)
  sw   t0 , 116(sp)
  lw   t0 , 116(sp)
  li    t1, 5
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 120(sp)
  lw   t0 , 120(sp)
  lw   t0 , 0(s0)
  sw   t0 , 124(sp)
  lw   t0 , 112(sp)
  lw   t1 , 124(sp)
  add   t0, t0, t1
  sw   t0 , 128(sp)
  lw   t0 , 24(sp)
  sw   t0 , 132(sp)
  lw   t0 , 132(sp)
  li    t1, 6
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 136(sp)
  lw   t0 , 136(sp)
  lw   t0 , 0(s0)
  sw   t0 , 140(sp)
  lw   t0 , 128(sp)
  lw   t1 , 140(sp)
  add   t0, t0, t1
  sw   t0 , 144(sp)
  lw   t0 , 28(sp)
  sw   t0 , 148(sp)
  lw   t0 , 148(sp)
  li    t1, 7
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 152(sp)
  lw   t0 , 152(sp)
  lw   t0 , 0(s0)
  sw   t0 , 156(sp)
  lw   t0 , 144(sp)
  lw   t1 , 156(sp)
  add   t0, t0, t1
  sw   t0 , 160(sp)
  lw   t0 , 32(sp)
  sw   t0 , 164(sp)
  lw   t0 , 164(sp)
  li    t1, 8
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 168(sp)
  lw   t0 , 168(sp)
  lw   t0 , 0(s0)
  sw   t0 , 172(sp)
  lw   t0 , 160(sp)
  lw   t1 , 172(sp)
  add   t0, t0, t1
  sw   t0 , 176(sp)
  lw   t0 , 36(sp)
  sw   t0 , 180(sp)
  lw   t0 , 180(sp)
  li    t1, 9
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 184(sp)
  lw   t0 , 184(sp)
  lw   t0 , 0(s0)
  sw   t0 , 188(sp)
  lw   t0 , 176(sp)
  lw   t1 , 188(sp)
  add   t0, t0, t1
  sw   t0 , 192(sp)
  lw   a0 , 192(sp)
  addi  sp, sp, 208
  ret

  .text
  .global f2
f2:
  addi  sp, sp, -208
  sw   a0 , 0(sp)
  sw   a1 , 4(sp)
  sw   a2 , 8(sp)
  sw   a3 , 12(sp)
  sw   a4 , 16(sp)
  sw   a5 , 20(sp)
  sw   a6 , 24(sp)
  sw   a7 , 28(sp)
  lw   t0 , 208(sp)
  sw   t0 , 32(sp)
  lw   t0 , 212(sp)
  sw   t0 , 36(sp)
  lw   t0 , 0(sp)
  sw   t0 , 40(sp)
  lw   t0 , 40(sp)
  li    t1, 0
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 44(sp)
  lw   t0 , 44(sp)
  li    t1, 9
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 48(sp)
  lw   t0 , 48(sp)
  lw   t0 , 0(s0)
  sw   t0 , 52(sp)
  lw   t0 , 4(sp)
  sw   t0 , 56(sp)
  lw   t0 , 56(sp)
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 60(sp)
  lw   t0 , 60(sp)
  lw   t0 , 0(s0)
  sw   t0 , 64(sp)
  lw   t0 , 52(sp)
  lw   t1 , 64(sp)
  add   t0, t0, t1
  sw   t0 , 68(sp)
  lw   t0 , 8(sp)
  sw   t0 , 72(sp)
  lw   t0 , 68(sp)
  lw   t1 , 72(sp)
  add   t0, t0, t1
  sw   t0 , 76(sp)
  lw   t0 , 12(sp)
  sw   t0 , 80(sp)
  lw   t0 , 80(sp)
  li    t1, 3
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 84(sp)
  lw   t0 , 84(sp)
  lw   t0 , 0(s0)
  sw   t0 , 88(sp)
  lw   t0 , 76(sp)
  lw   t1 , 88(sp)
  add   t0, t0, t1
  sw   t0 , 92(sp)
  lw   t0 , 16(sp)
  sw   t0 , 96(sp)
  lw   t0 , 96(sp)
  li    t1, 4
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 100(sp)
  lw   t0 , 100(sp)
  lw   t0 , 0(s0)
  sw   t0 , 104(sp)
  lw   t0 , 92(sp)
  lw   t1 , 104(sp)
  add   t0, t0, t1
  sw   t0 , 108(sp)
  lw   t0 , 20(sp)
  sw   t0 , 112(sp)
  lw   t0 , 112(sp)
  li    t1, 5
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 116(sp)
  lw   t0 , 116(sp)
  li    t1, 5
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 120(sp)
  lw   t0 , 120(sp)
  li    t1, 5
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 124(sp)
  lw   t0 , 124(sp)
  lw   t0 , 0(s0)
  sw   t0 , 128(sp)
  lw   t0 , 108(sp)
  lw   t1 , 128(sp)
  add   t0, t0, t1
  sw   t0 , 132(sp)
  lw   t0 , 24(sp)
  sw   t0 , 136(sp)
  lw   t0 , 136(sp)
  li    t1, 6
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 140(sp)
  lw   t0 , 140(sp)
  lw   t0 , 0(s0)
  sw   t0 , 144(sp)
  lw   t0 , 132(sp)
  lw   t1 , 144(sp)
  add   t0, t0, t1
  sw   t0 , 148(sp)
  lw   t0 , 28(sp)
  sw   t0 , 152(sp)
  lw   t0 , 152(sp)
  li    t1, 7
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 156(sp)
  lw   t0 , 156(sp)
  lw   t0 , 0(s0)
  sw   t0 , 160(sp)
  lw   t0 , 148(sp)
  lw   t1 , 160(sp)
  add   t0, t0, t1
  sw   t0 , 164(sp)
  lw   t0 , 32(sp)
  sw   t0 , 168(sp)
  lw   t0 , 164(sp)
  lw   t1 , 168(sp)
  add   t0, t0, t1
  sw   t0 , 172(sp)
  lw   t0 , 36(sp)
  sw   t0 , 176(sp)
  lw   t0 , 176(sp)
  li    t1, 9
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 180(sp)
  lw   t0 , 180(sp)
  li    t1, 8
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw   t0 , 184(sp)
  lw   t0 , 184(sp)
  lw   t0 , 0(s0)
  sw   t0 , 188(sp)
  lw   t0 , 172(sp)
  lw   t1 , 188(sp)
  add   t0, t0, t1
  sw   t0 , 192(sp)
  lw   a0 , 192(sp)
  addi  sp, sp, 208
  ret

  .text
  .global main
main:
  li   t3 , -4288
  add  sp, t3, sp
  li   t3 , 4284
  add   t3 , t3, sp
  sw   ra , 0(t3)
  li   t3 , 4008
  add   t3 , t3, sp
  sw   x0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 0
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4012
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   a0 , 4012
  add   a0 , sp, a0
  lw   a0 , 0(a0)
  call  init
  li   t0 , 4008
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li   t3 , 4016
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 0
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4020
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4020
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4024
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4024
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4028
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 1
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4032
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4032
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 1
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4036
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4036
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4040
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 2
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4044
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4044
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 2
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4048
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4048
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4052
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 3
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4056
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4056
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 3
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4060
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4060
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4064
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 4
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4068
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4068
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 4
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4072
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4072
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4076
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 5
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4080
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4080
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 5
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4084
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4084
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4088
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 6
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4092
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4092
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 6
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4096
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4096
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4100
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 7
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4104
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4104
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 7
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4108
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4108
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4112
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 8
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4116
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4116
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 8
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4120
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4120
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4124
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 9
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4128
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4128
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 9
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4132
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4132
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4136
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   a0 , 4028
  add   a0 , sp, a0
  lw   a0 , 0(a0)
  li   a1 , 4040
  add   a1 , sp, a1
  lw   a1 , 0(a1)
  li   a2 , 4052
  add   a2 , sp, a2
  lw   a2 , 0(a2)
  li   a3 , 4064
  add   a3 , sp, a3
  lw   a3 , 0(a3)
  li   a4 , 4076
  add   a4 , sp, a4
  lw   a4 , 0(a4)
  li   a5 , 4088
  add   a5 , sp, a5
  lw   a5 , 0(a5)
  li   a6 , 4100
  add   a6 , sp, a6
  lw   a6 , 0(a6)
  li   a7 , 4112
  add   a7 , sp, a7
  lw   a7 , 0(a7)
  li   t0 , 4124
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  sw   t0 , 0(sp)
  li   t0 , 4136
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  sw   t0 , 4(sp)
  call  f1
  li   t3 , 4140
  add   t3 , t3, sp
  sw   a0 , 0(t3)
  li   t0 , 4016
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li   t1 , 4140
  add   t1 , sp, t1
  lw   t1 , 0(t1)
  add   t0, t0, t1
  li   t3 , 4144
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4144
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li   t3 , 4008
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4008
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li   t3 , 4148
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 0
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4152
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4152
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4156
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 1
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4160
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4160
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 1
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4164
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4164
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4168
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 2
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4172
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4172
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 2
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4176
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4176
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 2
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4180
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4180
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  lw   t0 , 0(s0)
  li   t3 , 4184
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 3
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4188
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4188
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 3
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4192
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4192
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4196
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 4
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4200
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4200
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 4
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4204
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4204
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4208
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 0
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4212
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 6
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4216
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4216
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 6
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4220
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4220
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4224
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 7
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4228
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4228
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 7
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4232
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4232
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4236
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 8
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4240
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4240
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 8
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4244
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4244
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 8
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4248
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4248
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  lw   t0 , 0(s0)
  li   t3 , 4252
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li    t0,  8
  add  t0 , sp , t0
  li    t1, 9
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4256
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4256
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li    t1, 0
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  li   t3 , 4260
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   a0 , 4156
  add   a0 , sp, a0
  lw   a0 , 0(a0)
  li   a1 , 4168
  add   a1 , sp, a1
  lw   a1 , 0(a1)
  li   a2 , 4184
  add   a2 , sp, a2
  lw   a2 , 0(a2)
  li   a3 , 4196
  add   a3 , sp, a3
  lw   a3 , 0(a3)
  li   a4 , 4208
  add   a4 , sp, a4
  lw   a4 , 0(a4)
  li   a5 , 4212
  add   a5 , sp, a5
  lw   a5 , 0(a5)
  li   a6 , 4224
  add   a6 , sp, a6
  lw   a6 , 0(a6)
  li   a7 , 4236
  add   a7 , sp, a7
  lw   a7 , 0(a7)
  li   t0 , 4252
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  sw   t0 , 0(sp)
  li   t0 , 4260
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  sw   t0 , 4(sp)
  call  f2
  li   t3 , 4264
  add   t3 , t3, sp
  sw   a0 , 0(t3)
  li   t0 , 4148
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li   t1 , 4264
  add   t1 , sp, t1
  lw   t1 , 0(t1)
  add   t0, t0, t1
  li   t3 , 4268
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4268
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li   t3 , 4008
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   t0 , 4008
  add   t0 , sp, t0
  lw   t0 , 0(t0)
  li   t3 , 4272
  add   t3 , t3, sp
  sw   t0 , 0(t3)
  li   a0 , 4272
  add   a0 , sp, a0
  lw   a0 , 0(a0)
  call  putint
  li    a0, 10
  call  putch
  li    a0, 0
  li   ra , 4284
  add   ra , sp, ra
  lw   ra , 0(ra)
  li   t3 , 4288
  add  sp, t3, sp
  ret

