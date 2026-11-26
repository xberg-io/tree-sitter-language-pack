BUFSIZE	equ	256

	rsreset
pos_x	rs.w	1
pos_y	rs.w	1

PUSHALL	macro
	movem.l	d0-d7/a0-a6,-(sp)
	endm

	section	code,code
start:
	PUSHALL
	lea	buffer,a0
	move.w	#BUFSIZE-1,d0
	bsr	clear
.wait:
	tst.w	d0
	bne.s	.wait
	rts

clear:
	clr.b	(a0)+
	dbf	d0,clear
	rts

	section	bss,bss
buffer:	ds.b	BUFSIZE
