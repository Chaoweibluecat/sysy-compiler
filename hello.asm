.data
  .text
  .global init
init:
  addi  sp, sp, -128
  sw    a0, 0(sp)
  sw    x0, 4(sp)
  j while_entry0
while_entry0:
  lw    t0, 4(sp)
  sw    t0, 8(sp)
  lw    t0, 8(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 12(sp)
  lw    t0, 12(sp)
  bnez t0, while_body1
  j while_end2
while_body1:
  sw    x0, 16(sp)
  j while_entry3
while_entry3:
  lw    t0, 16(sp)
  sw    t0, 20(sp)
  lw    t0, 20(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 24(sp)
  lw    t0, 24(sp)
  bnez t0, while_body4
  j while_end5
while_body4:
  sw    x0, 28(sp)
  j while_entry6
while_entry6:
  lw    t0, 28(sp)
  sw    t0, 32(sp)
  lw    t0, 32(sp)
  li    t1, 10
  slt   t0, t0, t1
  sw    t0, 36(sp)
  lw    t0, 36(sp)
  bnez t0, while_body7
  j while_end8
while_body7:
  lw    t0, 0(sp)
  sw    t0, 40(sp)
  lw    t0, 4(sp)
  sw    t0, 44(sp)
  lw   t0 , 40(sp)
  lw ,  t1 , 44(sp)
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 48(sp)
  lw    t0, 16(sp)
  sw    t0, 52(sp)
  lw   t0 , 48(sp)
  lw ,  t1 , 52(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 56(sp)
  lw    t0, 28(sp)
  sw    t0, 60(sp)
  lw   t0 , 56(sp)
  lw ,  t1 , 60(sp)
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 64(sp)
  lw    t0, 4(sp)
  sw    t0, 68(sp)
  lw    t0, 68(sp)
  li    t1, 100
  mul   t0, t0, t1
  sw    t0, 72(sp)
  lw    t0, 16(sp)
  sw    t0, 76(sp)
  lw    t0, 76(sp)
  li    t1, 10
  mul   t0, t0, t1
  sw    t0, 80(sp)
  lw    t0, 72(sp)
  lw    t1, 80(sp)
  add   t0, t0, t1
  sw    t0, 84(sp)
  lw    t0, 28(sp)
  sw    t0, 88(sp)
  lw    t0, 84(sp)
  lw    t1, 88(sp)
  add   t0, t0, t1
  sw    t0, 92(sp)
  lw    t0, 92(sp)
  lw    t1, 64(sp)
  sw    t0, 0(t1)
  lw    t0, 28(sp)
  sw    t0, 96(sp)
  lw    t0, 96(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 100(sp)
  lw    t0, 100(sp)
  sw    t0, 28(sp)
  j while_entry6
while_end8:
  lw    t0, 16(sp)
  sw    t0, 104(sp)
  lw    t0, 104(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 108(sp)
  lw    t0, 108(sp)
  sw    t0, 16(sp)
  j while_entry3
while_end5:
  lw    t0, 4(sp)
  sw    t0, 112(sp)
  lw    t0, 112(sp)
  li    t1, 1
  add   t0, t0, t1
  sw    t0, 116(sp)
  lw    t0, 116(sp)
  sw    t0, 4(sp)
  j while_entry0
while_end2:
  addi  sp, sp,  128
  ret

  .text
  .global f1
f1:
  addi  sp, sp, -208
  sw    a0, 0(sp)
  sw    a1, 4(sp)
  sw    a2, 8(sp)
  sw    a3, 12(sp)
  sw    a4, 16(sp)
  sw    a5, 20(sp)
  sw    a6, 24(sp)
  sw    a7, 28(sp)
  lw    t0, 208(sp)
  sw    t0, 32(sp)
  lw    t0, 212(sp)
  sw    t0, 36(sp)
  lw    t0, 0(sp)
  sw    t0, 40(sp)
  lw   t0 , 40(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 44(sp)
  lw    t0, 44(sp)
  lw    t0, 0(t0)
  sw    t0, 48(sp)
  lw    t0, 4(sp)
  sw    t0, 52(sp)
  lw   t0 , 52(sp)
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 56(sp)
  lw    t0, 56(sp)
  lw    t0, 0(t0)
  sw    t0, 60(sp)
  lw    t0, 48(sp)
  lw    t1, 60(sp)
  add   t0, t0, t1
  sw    t0, 64(sp)
  lw    t0, 8(sp)
  sw    t0, 68(sp)
  lw   t0 , 68(sp)
  li    t1, 2
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 72(sp)
  lw    t0, 72(sp)
  lw    t0, 0(t0)
  sw    t0, 76(sp)
  lw    t0, 64(sp)
  lw    t1, 76(sp)
  add   t0, t0, t1
  sw    t0, 80(sp)
  lw    t0, 12(sp)
  sw    t0, 84(sp)
  lw   t0 , 84(sp)
  li    t1, 3
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 88(sp)
  lw    t0, 88(sp)
  lw    t0, 0(t0)
  sw    t0, 92(sp)
  lw    t0, 80(sp)
  lw    t1, 92(sp)
  add   t0, t0, t1
  sw    t0, 96(sp)
  lw    t0, 16(sp)
  sw    t0, 100(sp)
  lw   t0 , 100(sp)
  li    t1, 4
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 104(sp)
  lw    t0, 104(sp)
  lw    t0, 0(t0)
  sw    t0, 108(sp)
  lw    t0, 96(sp)
  lw    t1, 108(sp)
  add   t0, t0, t1
  sw    t0, 112(sp)
  lw    t0, 20(sp)
  sw    t0, 116(sp)
  lw   t0 , 116(sp)
  li    t1, 5
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 120(sp)
  lw    t0, 120(sp)
  lw    t0, 0(t0)
  sw    t0, 124(sp)
  lw    t0, 112(sp)
  lw    t1, 124(sp)
  add   t0, t0, t1
  sw    t0, 128(sp)
  lw    t0, 24(sp)
  sw    t0, 132(sp)
  lw   t0 , 132(sp)
  li    t1, 6
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 136(sp)
  lw    t0, 136(sp)
  lw    t0, 0(t0)
  sw    t0, 140(sp)
  lw    t0, 128(sp)
  lw    t1, 140(sp)
  add   t0, t0, t1
  sw    t0, 144(sp)
  lw    t0, 28(sp)
  sw    t0, 148(sp)
  lw   t0 , 148(sp)
  li    t1, 7
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 152(sp)
  lw    t0, 152(sp)
  lw    t0, 0(t0)
  sw    t0, 156(sp)
  lw    t0, 144(sp)
  lw    t1, 156(sp)
  add   t0, t0, t1
  sw    t0, 160(sp)
  lw    t0, 32(sp)
  sw    t0, 164(sp)
  lw   t0 , 164(sp)
  li    t1, 8
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 168(sp)
  lw    t0, 168(sp)
  lw    t0, 0(t0)
  sw    t0, 172(sp)
  lw    t0, 160(sp)
  lw    t1, 172(sp)
  add   t0, t0, t1
  sw    t0, 176(sp)
  lw    t0, 36(sp)
  sw    t0, 180(sp)
  lw   t0 , 180(sp)
  li    t1, 9
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 184(sp)
  lw    t0, 184(sp)
  lw    t0, 0(t0)
  sw    t0, 188(sp)
  lw    t0, 176(sp)
  lw    t1, 188(sp)
  add   t0, t0, t1
  sw    t0, 192(sp)
  lw    a0, 192(sp)
  addi  sp, sp,  208
  ret

  .text
  .global f2
f2:
  addi  sp, sp, -208
  sw    a0, 0(sp)
  sw    a1, 4(sp)
  sw    a2, 8(sp)
  sw    a3, 12(sp)
  sw    a4, 16(sp)
  sw    a5, 20(sp)
  sw    a6, 24(sp)
  sw    a7, 28(sp)
  lw    t0, 208(sp)
  sw    t0, 32(sp)
  lw    t0, 212(sp)
  sw    t0, 36(sp)
  lw    t0, 0(sp)
  sw    t0, 40(sp)
  lw   t0 , 40(sp)
  li    t1, 0
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 44(sp)
  lw   t0 , 44(sp)
  li    t1, 9
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 48(sp)
  lw    t0, 48(sp)
  lw    t0, 0(t0)
  sw    t0, 52(sp)
  lw    t0, 4(sp)
  sw    t0, 56(sp)
  lw   t0 , 56(sp)
  li    t1, 1
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 60(sp)
  lw    t0, 60(sp)
  lw    t0, 0(t0)
  sw    t0, 64(sp)
  lw    t0, 52(sp)
  lw    t1, 64(sp)
  add   t0, t0, t1
  sw    t0, 68(sp)
  lw    t0, 8(sp)
  sw    t0, 72(sp)
  lw    t0, 68(sp)
  lw    t1, 72(sp)
  add   t0, t0, t1
  sw    t0, 76(sp)
  lw    t0, 12(sp)
  sw    t0, 80(sp)
  lw   t0 , 80(sp)
  li    t1, 3
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 84(sp)
  lw    t0, 84(sp)
  lw    t0, 0(t0)
  sw    t0, 88(sp)
  lw    t0, 76(sp)
  lw    t1, 88(sp)
  add   t0, t0, t1
  sw    t0, 92(sp)
  lw    t0, 16(sp)
  sw    t0, 96(sp)
  lw   t0 , 96(sp)
  li    t1, 4
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 100(sp)
  lw    t0, 100(sp)
  lw    t0, 0(t0)
  sw    t0, 104(sp)
  lw    t0, 92(sp)
  lw    t1, 104(sp)
  add   t0, t0, t1
  sw    t0, 108(sp)
  lw    t0, 20(sp)
  sw    t0, 112(sp)
  lw   t0 , 112(sp)
  li    t1, 5
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 116(sp)
  lw   t0 , 116(sp)
  li    t1, 5
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 120(sp)
  lw   t0 , 120(sp)
  li    t1, 5
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 124(sp)
  lw    t0, 124(sp)
  lw    t0, 0(t0)
  sw    t0, 128(sp)
  lw    t0, 108(sp)
  lw    t1, 128(sp)
  add   t0, t0, t1
  sw    t0, 132(sp)
  lw    t0, 24(sp)
  sw    t0, 136(sp)
  lw   t0 , 136(sp)
  li    t1, 6
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 140(sp)
  lw    t0, 140(sp)
  lw    t0, 0(t0)
  sw    t0, 144(sp)
  lw    t0, 132(sp)
  lw    t1, 144(sp)
  add   t0, t0, t1
  sw    t0, 148(sp)
  lw    t0, 28(sp)
  sw    t0, 152(sp)
  lw   t0 , 152(sp)
  li    t1, 7
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 156(sp)
  lw    t0, 156(sp)
  lw    t0, 0(t0)
  sw    t0, 160(sp)
  lw    t0, 148(sp)
  lw    t1, 160(sp)
  add   t0, t0, t1
  sw    t0, 164(sp)
  lw    t0, 32(sp)
  sw    t0, 168(sp)
  lw    t0, 164(sp)
  lw    t1, 168(sp)
  add   t0, t0, t1
  sw    t0, 172(sp)
  lw    t0, 36(sp)
  sw    t0, 176(sp)
  lw   t0 , 176(sp)
  li    t1, 9
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 180(sp)
  lw   t0 , 180(sp)
  li    t1, 8
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 184(sp)
  lw    t0, 184(sp)
  lw    t0, 0(t0)
  sw    t0, 188(sp)
  lw    t0, 172(sp)
  lw    t1, 188(sp)
  add   t0, t0, t1
  sw    t0, 192(sp)
  lw    a0, 192(sp)
  addi  sp, sp,  208
  ret

  .text
  .global main
main:
  addi  sp, sp, -288
    sw  ra, 284(sp)
  sw    x0, 12(sp)
  addi t0, sp, 8
  li    t1, 0
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 16(sp)
  lw    a0, 16(sp)
  call  init
  lw    t0, 12(sp)
  sw    t0, 20(sp)
  addi t0, sp, 8
  li    t1, 0
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 24(sp)
  lw   t0 , 24(sp)
  li    t1, 0
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 28(sp)
  lw   t0 , 28(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 32(sp)
  addi t0, sp, 8
  li    t1, 1
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 36(sp)
  lw   t0 , 36(sp)
  li    t1, 1
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 40(sp)
  lw   t0 , 40(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 44(sp)
  addi t0, sp, 8
  li    t1, 2
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 48(sp)
  lw   t0 , 48(sp)
  li    t1, 2
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 52(sp)
  lw   t0 , 52(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 56(sp)
  addi t0, sp, 8
  li    t1, 3
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 60(sp)
  lw   t0 , 60(sp)
  li    t1, 3
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 64(sp)
  lw   t0 , 64(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 68(sp)
  addi t0, sp, 8
  li    t1, 4
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 72(sp)
  lw   t0 , 72(sp)
  li    t1, 4
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 76(sp)
  lw   t0 , 76(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 80(sp)
  addi t0, sp, 8
  li    t1, 5
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 84(sp)
  lw   t0 , 84(sp)
  li    t1, 5
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 88(sp)
  lw   t0 , 88(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 92(sp)
  addi t0, sp, 8
  li    t1, 6
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 96(sp)
  lw   t0 , 96(sp)
  li    t1, 6
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 100(sp)
  lw   t0 , 100(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 104(sp)
  addi t0, sp, 8
  li    t1, 7
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 108(sp)
  lw   t0 , 108(sp)
  li    t1, 7
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 112(sp)
  lw   t0 , 112(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 116(sp)
  addi t0, sp, 8
  li    t1, 8
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 120(sp)
  lw   t0 , 120(sp)
  li    t1, 8
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 124(sp)
  lw   t0 , 124(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 128(sp)
  addi t0, sp, 8
  li    t1, 9
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 132(sp)
  lw   t0 , 132(sp)
  li    t1, 9
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 136(sp)
  lw   t0 , 136(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 140(sp)
  lw    a0, 32(sp)
  lw    a1, 44(sp)
  lw    a2, 56(sp)
  lw    a3, 68(sp)
  lw    a4, 80(sp)
  lw    a5, 92(sp)
  lw    a6, 104(sp)
  lw    a7, 116(sp)
  lw    t0, 128(sp)
  sw    t0, 0(sp)
  lw    t0, 140(sp)
  sw    t0, 4(sp)
  call  f1
  sw    a0, 144(sp)
  lw    t0, 20(sp)
  lw    t1, 144(sp)
  add   t0, t0, t1
  sw    t0, 148(sp)
  lw    t0, 148(sp)
  sw    t0, 12(sp)
  lw    t0, 12(sp)
  sw    t0, 152(sp)
  addi t0, sp, 8
  li    t1, 0
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 156(sp)
  lw   t0 , 156(sp)
  li    t1, 0
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 160(sp)
  addi t0, sp, 8
  li    t1, 1
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 164(sp)
  lw   t0 , 164(sp)
  li    t1, 1
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 168(sp)
  lw   t0 , 168(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 172(sp)
  addi t0, sp, 8
  li    t1, 2
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 176(sp)
  lw   t0 , 176(sp)
  li    t1, 2
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 180(sp)
  lw   t0 , 180(sp)
  li    t1, 2
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 184(sp)
  lw    t0, 184(sp)
  lw    t0, 0(t0)
  sw    t0, 188(sp)
  addi t0, sp, 8
  li    t1, 3
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 192(sp)
  lw   t0 , 192(sp)
  li    t1, 3
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 196(sp)
  lw   t0 , 196(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 200(sp)
  addi t0, sp, 8
  li    t1, 4
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 204(sp)
  lw   t0 , 204(sp)
  li    t1, 4
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 208(sp)
  lw   t0 , 208(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 212(sp)
  addi t0, sp, 8
  li    t1, 0
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 216(sp)
  addi t0, sp, 8
  li    t1, 6
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 220(sp)
  lw   t0 , 220(sp)
  li    t1, 6
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 224(sp)
  lw   t0 , 224(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 228(sp)
  addi t0, sp, 8
  li    t1, 7
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 232(sp)
  lw   t0 , 232(sp)
  li    t1, 7
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 236(sp)
  lw   t0 , 236(sp)
  li    t1, 0
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 240(sp)
  addi t0, sp, 8
  li    t1, 8
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 244(sp)
  lw   t0 , 244(sp)
  li    t1, 8
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 248(sp)
  lw   t0 , 248(sp)
  li    t1, 8
  li   t2 , 4
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 252(sp)
  lw    t0, 252(sp)
  lw    t0, 0(t0)
  sw    t0, 256(sp)
  addi t0, sp, 8
  li    t1, 9
  li   t2 , 400
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 260(sp)
  lw   t0 , 260(sp)
  li    t1, 0
  li   t2 , 40
  mul t1, t1, t2
  add t0, t0, t1
  sw  t0, 264(sp)
  lw    a0, 160(sp)
  lw    a1, 172(sp)
  lw    a2, 188(sp)
  lw    a3, 200(sp)
  lw    a4, 212(sp)
  lw    a5, 216(sp)
  lw    a6, 228(sp)
  lw    a7, 240(sp)
  lw    t0, 256(sp)
  sw    t0, 0(sp)
  lw    t0, 264(sp)
  sw    t0, 4(sp)
  call  f2
  sw    a0, 268(sp)
  lw    t0, 152(sp)
  lw    t1, 268(sp)
  add   t0, t0, t1
  sw    t0, 272(sp)
  lw    t0, 272(sp)
  sw    t0, 12(sp)
  lw    t0, 12(sp)
  sw    t0, 276(sp)
  lw    a0, 276(sp)
  call  putint
  li    a0,  10
  call  putch
  li    a0, 0
    lw  ra, 284(sp)
  addi  sp, sp,  288
  ret

