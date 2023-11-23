#once

#subruledef Reg {
    zero =>  0`5
    ra   =>  1`5
    sp   =>  2`5
    fp   =>  3`5
    gp   =>  4`5
    tp   =>  5`5
    s0   =>  6`5
    s1   =>  7`5
    a0   =>  8`5
    a1   =>  9`5
    a2   => 10`5
    a3   => 11`5
    a4   => 12`5
    a5   => 13`5
    a6   => 14`5
    a7   => 15`5
    s2   => 16`5
    s3   => 17`5
    s4   => 18`5
    s5   => 19`5
    s6   => 20`5
    s7   => 21`5
    s8   => 22`5
    s9   => 23`5
    t0   => 24`5
    t1   => 25`5
    t2   => 26`5
    t3   => 27`5
    t4   => 28`5
    t5   => 29`5
    t6   => 30`5
    t7   => 31`5
}

#subruledef Condition {
    eq    => 0b000
    ne    => 0b001
    lt    => 0b010
    ge    => 0b011
    lts   => 0b100
    ges   => 0b101
    true  => 0b110
    false => 0b111
}

#subruledef BranchCondition {
    eq   => 0b000
    ne   => 0b001
    lt   => 0b010
    ge   => 0b011
    lts  => 0b100
    ges  => 0b101
    true => 0b110
    link => 0b111
}

#subruledef AluOp {
    add => 0b000
    sub => 0b001
    and => 0b010
    or  => 0b011
    xor => 0b100
    shl => 0b101
    lsr => 0b110
    asr => 0b111
}

#subruledef LoadOp {
    32  => 0b000
    8u  => 0b010
    8s  => 0b011
    16u => 0b100
    16s => 0b101
    in  => 0b110
}

#subruledef StoreOp {
    32  => 0b000
    8   => 0b010
    16  => 0b100
    out => 0b110
}

#ruledef Jump {
    jump {rd: Reg} , {rb: Reg} , {imm: s14} => {
        assert(rd == 0)
        assert(rb <= 15)
        assert(imm % 2 == 0)
        assert(imm >= -512)
        assert(imm <= 511)
        le(rb`4 @ imm[4:1] @ imm[5:5] @ imm[8:6] @ imm[9:9] @ 0b001)
    }
    jump {rd: Reg} , {rb: Reg} , {imm: s14} => {
        assert(rd == 1)
        assert(rb <= 15)
        assert(imm % 2 == 0)
        assert(imm >= -512)
        assert(imm <= 511)
        le(rb`4 @ imm[4:1] @ imm[5:5] @ imm[8:6] @ imm[9:9] @ 0b101)
    }
    jump {rd: Reg} , {rb: Reg} , {imm: s14} => {
        assert(imm % 2 == 0)
        le(imm[13:13] @ imm[8:5] @ imm[12:10] @ 0b00 @ rb @ rd @ imm[4:1] @ imm[9:9] @ 0b0111111)
    }
}

#ruledef Branch {
    branch.{cond: BranchCondition} {imm: u32} => {
        assert(imm % 2 == 0)
        offset = imm - $ - 2
        assert(offset >= -512)
        assert(offset <= 511)
        le(offset[5:5] @ cond @ offset[4:1] @ 0b0 @ offset[8:6] @ offset[9:9] @ 0b011)
    }
    branch.{cond: BranchCondition} {imm: u32} => {
        assert(imm % 2 == 0)
        offset = imm - $ - 4
        assert(offset >= -1048576)
        assert(offset <= 1048575)
        le(offset[20:20] @ offset[8:5] @ offset[12:10] @ 0b00 @ offset[19:13] @ cond @ offset[4:1] @ offset[9:9] @ 0b1111111)
    }
}

#ruledef Alu {
    alu.i.{op: AluOp} {rd: Reg} , {rs1: Reg} , {imm: s10} => {
        assert(op == 0b000)
        assert(rd <= 15)
        assert(rd == rs1)
        le(rd`4 @ imm[4:0] @ imm[8:6] @ imm[9:9] @ imm[5:5] @ 0b10)
    }
    alu.i.{op: AluOp} {rd: Reg} , {rs1: Reg} , {imm: s10} => {
        assert(op == 0b001)
        assert(rd <= 15)
        assert(rd == rs1)
        nimm = -imm
        assert(nimm >= -512)
        assert(nimm <= 511)
        le(rd`4 @ nimm[4:0] @ nimm[8:6] @ nimm[9:9] @ nimm[5:5] @ 0b10)
    }
    alu.i.{op: AluOp} {rd: Reg} , {rs1: Reg} , {imm: s10} => {
        assert((op == 0b101) || (op == 0b110) || (op == 0b111))
        assert(rd <= 15)
        assert(rd == rs1)
        assert((imm >= 0) && (imm <= 31))
        le(rd`4 @ imm[4:0] @ op`2 @ 0b10111)
    }
    alu.i.{op: AluOp} {rd: Reg} , {rs1: Reg} , {imm: s10} => {
        assert((op == 0b101) || (op == 0b110) || (op == 0b111))
        assert((imm >= 0) && (imm <= 31))
        le(0b00000 @ op @ 0b01 @ rs1 @ rd @ imm[4:0] @ 0b0111111)
    }
    alu.i.{op: AluOp} {rd: Reg} , {rs1: Reg} , {imm: s10} => {
        assert((op != 0b101) && (op != 0b110) && (op != 0b111))
        le(imm[9:9] @ imm[8:5] @ op @ 0b01 @ rs1 @ rd @ imm[4:0] @ 0b0111111)
    }

    alu.{op: AluOp} {rd: Reg} , {rs1: Reg} , {rs2: Reg} => {
        assert(rd <= 15)
        assert(rd == rs1)
        assert(rs2 <= 15)
        le(rd`4 @ rs2`4 @ op @ 0b00111)
    }
    alu.{op: AluOp} {rd: Reg} , {rs1: Reg} , {rs2: Reg} => {
        assert(op == 0b001)
        assert(rd == 0)
        assert(rs1 <= 15)
        assert(rs2 <= 15)
        le(rs1`4 @ rs2`4 @ 0b00010111)
    }
    alu.{op: AluOp} {rd: Reg} , {rs1: Reg} , {rs2: Reg} => {
        le(0b00000 @ op @ 0b11 @ rs1 @ rd @ rs2[3:0] @ rs2[4:4] @ 0b0111111)
    }
}

#ruledef Move {
    move.i.{cond: Condition} {rd: Reg} , {rs1: Reg} , {imm: s10} => {
        assert(cond == 0b110)
        assert(rd <= 15)
        le(rd`4 @ imm[4:0] @ imm[8:6] @ imm[9:9] @ imm[5:5] @ 0b00)
    }
    move.i.{cond: Condition} {rd: Reg} , {rs1: Reg} , {imm: s10} => {
        le(imm[9:9] @ imm[8:5] @ cond @ 0b01 @ rs1 @ rd @ imm[4:0] @ 0b1111111)
    }

    move.{cond: Condition} {rd: Reg} , {rs1: Reg} , {rs2: Reg} => {
        assert(rd <= 15)
        assert(rd == rs1)
        assert(rs2 <= 15)
        le(rd`4 @ rs2`4 @ cond @ 0b01111)
    }
    move.{cond: Condition} {rd: Reg} , {rs1: Reg} , {rs2: Reg} => {
        le(0b00000 @ cond @ 0b11 @ rs1 @ rd @ rs2[3:0] @ rs2[4:4] @ 0b1111111)
    }
}

#ruledef Load {
    load.{op: LoadOp} {rd: Reg} , [ {rb: Reg} , {imm: s10} ] => {
        assert(op == 0b000)
        assert(rd <= 15)
        assert(rb == 2)
        assert(imm & 0x3 == 0)
        assert(imm >= 0)
        assert(imm <= 127)
        le(rd`4 @ imm[4:2] @ imm[6:5] @ 0b0011111)
    }
    load.{op: LoadOp} {rd: Reg} , [ {rb: Reg} , {imm: s10} ] => {
        le(imm[9:9] @ imm[8:5] @ op @ 0b10 @ rb @ rd @ imm[4:0] @ 0b0111111)
    }
}

#ruledef Store {
    store.{op: StoreOp} [ {rb: Reg} , {imm: s10} ] , {rs: Reg} => {
        assert(op == 0b000)
        assert(rb == 2)
        assert(rs <= 15)
        assert(imm & 0x3 == 0)
        assert(imm >= 0)
        assert(imm <= 127)
        le(rs`4 @ imm[4:2] @ imm[6:5] @ 0b1011111)
    }
    store.{op: StoreOp} [ {rb: Reg} , {imm: s10} ] , {rs: Reg} => {
        le(imm[9:9] @ imm[8:5] @ op @ 0b10 @ rb @ imm[4:0] @ rs[3:0] @ rs[4:4] @ 0b1111111)
    }
}

#ruledef Instructions {
    ldui  {rd: Reg} , {imm: i32} => {
        assert(imm & 0x3FF == 0)
        le(imm[31:31] @ imm[30:27] @ imm[12:10] @ imm[14:13] @ imm[19:15] @ rd @ imm[26:23] @ 0b1 @ imm[22:20] @ 0b0011)
    }
    apcui {rd: Reg} , {imm: i32} => {
        assert(imm & 0x3FF == 0)
        le(imm[31:31] @ imm[30:27] @ imm[12:10] @ imm[14:13] @ imm[19:15] @ rd @ imm[26:23] @ 0b1 @ imm[22:20] @ 0b1011)
    }

    ret             => le(0x0 @ 0x0 @ 0b10010111)
    sysret          => le(0x0 @ 0x1 @ 0b10010111)
    fence           => le(0xF @ 0x2 @ 0b10010111)
    ifence          => le(0x0 @ 0x3 @ 0b10010111)
    envcall {n: u4} => le(n   @ 0xE @ 0b10010111)
    syscall {n: u4} => le(n   @ 0xF @ 0b10010111)

    j              {rb: Reg} , {imm: s14} => asm { jump zero, {rb}, {imm} }
    jl {rd: Reg} , {rb: Reg} , {imm: s14} => asm { jump {rd}, {rb}, {imm} }

    br.eq  {imm: u32} => asm { branch.eq   {imm} }
    br.ne  {imm: u32} => asm { branch.ne   {imm} }
    br.lt  {imm: u32} => asm { branch.lt   {imm} }
    br.ge  {imm: u32} => asm { branch.ge   {imm} }
    br.lts {imm: u32} => asm { branch.lts  {imm} }
    br.ges {imm: u32} => asm { branch.ges  {imm} }
    jr     {imm: u32} => asm { branch.true {imm} }
    jrl    {imm: u32} => asm { branch.link {imm} }

    addi {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { alu.i.add {rd}, {rs1}, {imm} }
    subi {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { alu.i.sub {rd}, {rs1}, {imm} }
    andi {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { alu.i.and {rd}, {rs1}, {imm} }
    ori  {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { alu.i.or  {rd}, {rs1}, {imm} }
    xori {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { alu.i.xor {rd}, {rs1}, {imm} }
    shli {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { alu.i.shl {rd}, {rs1}, {imm} }
    lsri {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { alu.i.lsr {rd}, {rs1}, {imm} }
    asri {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { alu.i.asr {rd}, {rs1}, {imm} }

    movi.eq  {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { move.i.eq   {rd}, {rs1}, {imm} }
    movi.ne  {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { move.i.ne   {rd}, {rs1}, {imm} }
    movi.lt  {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { move.i.lt   {rd}, {rs1}, {imm} }
    movi.ge  {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { move.i.ge   {rd}, {rs1}, {imm} }
    movi.lts {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { move.i.lts  {rd}, {rs1}, {imm} }
    movi.ges {rd: Reg} , {rs1: Reg} , {imm: s10} => asm { move.i.ges  {rd}, {rs1}, {imm} }
    ldi      {rd: Reg} ,              {imm: s10} => asm { move.i.true {rd}, {rd} , {imm} }

    ld.32  {rd: Reg} , [ {rb: Reg} , {imm: s10} ] => asm { load.32  {rd}, [{rb}, {imm}] }
    ld.8u  {rd: Reg} , [ {rb: Reg} , {imm: s10} ] => asm { load.8u  {rd}, [{rb}, {imm}] }
    ld.8s  {rd: Reg} , [ {rb: Reg} , {imm: s10} ] => asm { load.8s  {rd}, [{rb}, {imm}] }
    ld.16u {rd: Reg} , [ {rb: Reg} , {imm: s10} ] => asm { load.16u {rd}, [{rb}, {imm}] }
    ld.16s {rd: Reg} , [ {rb: Reg} , {imm: s10} ] => asm { load.16s {rd}, [{rb}, {imm}] }
    in     {rd: Reg} , [ {rb: Reg} , {imm: s10} ] => asm { load.in  {rd}, [{rb}, {imm}] }

    st.32 [ {rb: Reg} , {imm: s10} ] , {rs: Reg} => asm { store.32  [{rb}, {imm}], {rs} }
    st.8  [ {rb: Reg} , {imm: s10} ] , {rs: Reg} => asm { store.8   [{rb}, {imm}], {rs} }
    st.16 [ {rb: Reg} , {imm: s10} ] , {rs: Reg} => asm { store.16  [{rb}, {imm}], {rs} }
    out   [ {rb: Reg} , {imm: s10} ] , {rs: Reg} => asm { store.out [{rb}, {imm}], {rs} }

    add {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { alu.add {rd}, {rs1}, {rs2} }
    sub {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { alu.sub {rd}, {rs1}, {rs2} }
    and {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { alu.and {rd}, {rs1}, {rs2} }
    or  {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { alu.or  {rd}, {rs1}, {rs2} }
    xor {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { alu.xor {rd}, {rs1}, {rs2} }
    shl {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { alu.shl {rd}, {rs1}, {rs2} }
    lsr {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { alu.lsr {rd}, {rs1}, {rs2} }
    asr {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { alu.asr {rd}, {rs1}, {rs2} }

    mov.eq  {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { move.eq    {rd}, {rs1}, {rs2} }
    mov.ne  {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { move.ne    {rd}, {rs1}, {rs2} }
    mov.lt  {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { move.lt    {rd}, {rs1}, {rs2} }
    mov.ge  {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { move.ge    {rd}, {rs1}, {rs2} }
    mov.lts {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { move.lts   {rd}, {rs1}, {rs2} }
    mov.ges {rd: Reg} , {rs1: Reg} , {rs2: Reg} => asm { move.ges   {rd}, {rs1}, {rs2} }
    mov     {rd: Reg} ,              {rs2: Reg} => asm { move.true  {rd}, {rd} , {rs2} }
    nop                                         => asm { move.false zero, zero , zero  }

    addc {rd: Reg} , {rs1: Reg} , {rs2: Reg} => le(0b0000100011 @ rs1 @ rd @ rs2[3:0] @ rs2[4:4] @ 0b0111111)
    subc {rd: Reg} , {rs1: Reg} , {rs2: Reg} => le(0b0000100111 @ rs1 @ rd @ rs2[3:0] @ rs2[4:4] @ 0b0111111)
}
