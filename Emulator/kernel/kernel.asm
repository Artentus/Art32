#include "art32.asm"

#bankdef KernelRam {
    #bits 8
    #addr 0x1000_0000
    #size 0x0000_8000
    #outp 0 * 8
    #fill 0
}

ldi a0, 0
loop:
addi a0, a0, 1
jr loop
