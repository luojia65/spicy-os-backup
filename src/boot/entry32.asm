    .section .text.entry
    .globl _start
_start:
    csrw mie, 0
    csrw mip, 0
.option push
.option norelax
	la gp, __global_pointer$
.option pop
    la sp, bootstacktop
    call rust_main

    .section .bss.stack
    .align 12
    .global bootstack
bootstack:
    .space 4096 * 4
    .global bootstacktop
bootstacktop:
