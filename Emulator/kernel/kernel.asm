#include "art32.asm"
#include "ports.asm"
#include "env.asm"

#bankdef KernelRam {
    #bits 8
    #addr 0x1000_0000
    #size 0x0000_8000
    #outp 0 * 8
    #fill 0
}


INTERRUPT_STACK_BASE = 0x1000_8000
KERNEL_STACK_BASE = 0x1000_7F00


__reset:
    ; disable all interrupts
    out [zero, INT_MASK_ADDR], zero
    ; initialize interrupt stack
    ldi sp, INTERRUPT_STACK_BASE

    ; set default trap handlers
    ldi a2, __unimplemented
    out [zero, ILLEGAL_INSTRUCTION_SLOT_ADDR], a2
    out [zero, ACCESS_VIOLATION_SLOT_ADDR], a2
    out [zero, UNALIGNED_ACCESS_SLOT_ADDR], a2
    
    ; set default hardware interrupt handlers
    ldi a0, HARD_INT_TABLE_START
    ldi a1, HARD_INT_TABLE_END
    .set_hard_ints:
    out [a0, 0], a2
    addi a0, a0, 1
    cmp a0, a1
    br.lt .set_hard_ints

    ; set default software interrupt handlers
    ldi a0, SOFT_INT_TABLE_START
    ldi a1, SOFT_INT_TABLE_END
    .set_soft_ints:
    out [a0, 0], a2
    addi a0, a0, 1
    cmp a0, a1
    br.lt .set_soft_ints

    ; initialize kernel stack
    ldi a0, KERNEL_STACK_BASE
    out [zero, ALT_REGS_START + 2], a0
    ; set privilege level to `system`
    out [zero, PRIV_LEVEL_ADDR], zero
    ; set kernel entry point address
    ldi a0, __start
    out [zero, INT_RET_ADDR], a0
    ; jump into kernel
    sysret

__unimplemented:
    ; we shouldn't be here
    err
    ; infinite loop to halt execution
    .loop:
    jr .loop


MSG:
#d "Hello world!\0"


#align 16
__start:
    ; call main function
    jrl kernel_main
    ; we shouldn't be here
    jr __unimplemented


#align 16
kernel_main:
    ldi a0, MSG
    jrl serial_print

    ; infinite loop to halt execution
    .loop:
    jr .loop


; fn serial_print_char(c: u8{a0})
#align 16
serial_print_char:
    ; wait for room in the queue
    .wait:
    in a1, [zero, SERIAL_OUT_COUNT_ADDR]
    cmp a1, zero
    br.eq .wait

    out [zero, SERIAL_OUT_DATA_ADDR], a0
    ret


; fn serial_print(c: *const u8{a0})
#align 16
serial_print:
    ; save clobbered registers
    addi sp, sp, -8
    st.32 [sp, 0], ra
    st.32 [sp, 4], s0

    ; make room in a0 for passing chars to `serial_print_char`
    mov s0, a0

    .loop:
    ; load next char in the string
    ld.8u a0, [s0, 0]

    ; break the loop if we encounter a null terminator
    cmp a0, zero
    br.eq .break

    ; print char
    jrl serial_print_char

    ; increment pointer and continue the loop
    addi s0, s0, 1
    jr .loop
    .break:

    ; restore clobbered registers
    ld.32 ra, [sp, 0]
    ld.32 s0, [sp, 4]
    addi sp, sp, 8
    ret
