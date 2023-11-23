#include "art32.asm"
#include "ports.asm"

#bankdef KernelRam {
    #bits 8
    #addr 0x1000_0000
    #size 0x0000_8000
    #outp 0 * 8
    #fill 0
}

jr _start

MSG:
#d "Hello world!\0"

#align 16
_start:
    ; initialize stack
    ldi sp, 0x1000_8000

    ldi a0, MSG
    jrl serial_print

    ; infinite loop to halt execution
    hlt:
    jr hlt


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
