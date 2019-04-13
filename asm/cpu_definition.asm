#cpudef
{
    #bits 8

    #tokendef alu_2op 
    {
        add = 0
        sub = 1
        mul = 2
        div = 3
        mod = 4
        shl = 6
        shr = 7
        rotl = 8
        rotr = 9
        bit_and = 10
        bit_or = 11
        bit_xor = 12
        bool_and = 13
        bool_or = 14
        bool_xor = 15
        cmp_eq = 16
        cmp_ne = 17
        cmp_gt = 18
        cmp_lt = 19
        cmp_geq = 20
        cmp_leq = 21
    }

    #tokendef alu_1op 
    {
        neg = 22
        bit_not = 23
        bool_not = 24
    }

    #tokendef alu_cmpop
    {
        eq = 0
        ne = 1
        gt = 2
        lt = 3
        geq = 4
        leq = 5
    }


    #tokendef jmp_cmp
    {
        ltz = 0
        lez = 1
        eqz = 2
        gez = 3
        gtz = 4
        nez = 5
    }

    ; - Base
    nop -> 0x0000
    halt -> 0xffff

    ; load constant into reg
    ldc ~{reg}, {value} -> {
        assert(reg >= 0)
        assert(reg < 32)
        0x0102 @ reg[7:0] @ value[31:0]
    }

    ; unsigned unary operation on register
    calc.u ~{out}, {op: alu_1op} ( ~{lhs} ) -> {
        assert(out >= 0)
        assert(out < 32)
        assert(lhs >= 0)
        assert(lhs < 32)
        0x0200 @ out[7:0] @ op[7:0] @ lhs[7:0] 
    }

    ; unsigned `reg ? reg` operation
    calc.u ~{out}, {op: alu_2op} ( ~{lhs}, ~{rhs} ) -> {
        assert(out >= 0)
        assert(out < 32)
        assert(lhs >= 0)
        assert(lhs < 32)
        assert(rhs >= 0)
        assert(rhs < 32)
        0x0200 @ out[7:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
    }

    ; unsigned `const ? reg` operation
    calc.u ~{out}, {op: alu_2op} ( {lhs}, ~{rhs} ) -> {
        assert(out >= 0)
        assert(out < 32)
        assert(lhs >= 0)
        assert(rhs < 32)
        assert(rhs >= 0)
        0x0102 @ 0x20 @ lhs[31:0] @
        0x0200 @ out[7:0] @ op[7:0] @ 0x20 @ rhs[7:0]
    }

    ; unsigned `reg ? const` operation
    calc.u ~{out}, {op: alu_2op} ( ~{lhs}, {rhs} ) -> {
        assert(out >= 0)
        assert(out < 32)
        assert(lhs >= 0)
        assert(lhs < 32)
        assert(rhs >= 0)
        0x0102 @ 0x20 @ rhs[31:0] @
        0x0200 @ out[7:0] @ op[7:0] @ lhs[7:0] @ 0x20
    }

    ; signed unary operation on register
    calc.s ~{out}, {op: alu_1op} ( ~{lhs} ) -> {
        assert(out >= 0)
        assert(out < 32)
        assert(lhs >= 0)
        assert(lhs < 32)
        0x0201 @ out[7:0] @ op[7:0] @ lhs[7:0] 
    }

    ; signed `reg ? reg` operation
    calc.s ~{out}, {op: alu_2op} ( ~{lhs}, ~{rhs} ) -> {
        assert(out >= 0)
        assert(out < 32)
        assert(lhs >= 0)
        assert(lhs < 32)
        assert(rhs < 32)
        assert(rhs >= 0)
        0x0201 @ out[7:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
    }

    ; signed `const ? reg` operation
    calc.s ~{out}, {op: alu_2op} ( {lhs}, ~{rhs} ) -> {
        assert(out >= 0)
        assert(out < 32)
        assert(rhs < 32)
        assert(rhs >= 0)
        0x0102 @ 0x20 @ lhs[31:0] @
        0x0201 @ out[7:0] @ op[7:0] @ 0x20 @ rhs[7:0]
    }

    ; signed `reg ? const` operation
    calc.s ~{out}, {op: alu_2op} ( ~{lhs}, {rhs} ) -> {
        assert(out >= 0)
        assert(out < 32)
        assert(lhs >= 0)
        assert(lhs < 32)
        0x0102 @ 0x20 @ rhs[31:0] @
        0x0201 @ out[7:0] @ op[7:0] @ lhs[7:0] @ 0x20
    }

    ; jump to passed address/label
    jmp {addr} -> {
        assert(addr >= 0)
        0x0300 @ addr[31:0]
    }

    ; jump if unsigned comparison against zero passes
    jmpif.u {addr}, {cmp: jmp_cmp} ( ~{lhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(lhs < 32)
        0x0301 @ addr[31:0] @ cmp[7:0] @ lhs[7:0] 
    }

    ; jump if signed comparison against zero passes
    jmpif.s {addr}, {cmp: jmp_cmp} ( ~{lhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(lhs < 32)
        0x0302 @ addr[31:0] @ cmp[7:0] @ lhs[7:0] 
    }

    ; jump if unsigned comparison against zero passes, using the result of an unsigned unary operation
    jmpif.u {addr}, {cmp: jmp_cmp} calc {op: alu_1op} ( ~{lhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(lhs < 32)
        0x0302 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] 
    }

    ; jump if unsigned comparison against zero passes, using the result of an unsigned `reg ? reg` operation
    jmpif.u {addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( ~{lhs}, ~{rhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(lhs < 32)
        assert(rhs >= 0)
        assert(rhs < 32)
        0x0303 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
    }

    ; jump if unsigned comparison against zero passes, using the result of an unsigned `const ? reg` operation
    jmpif.u {addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( {lhs}, ~{rhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(rhs >= 0)
        assert(rhs < 32)
        0x0102 @ 0x20 @ lhs[31:0] @
        0x0303 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ 0x20 @ rhs[7:0]
    }

    ; jump if unsigned comparison against zero passes, using the result of an unsigned `reg ? const` operation
    jmpif.u {addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( ~{lhs}, {rhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(lhs < 32)
        assert(rhs >= 0)
        0x0102 @ 0x20 @ rhs[31:0] @
        0x0303 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] @ 0x20
    }

    ; jump if signed comparison against zero passes, using the result of an signed unary operation
    jmpif.s {addr}, {cmp: jmp_cmp} calc {op: alu_1op} ( ~{lhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(lhs < 32)
        0x0304 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] 
    }

    ; jump if signed comparison against zero passes, using the result of an signed `reg ? reg` operation
    jmpif.s {addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( ~{lhs}, ~{rhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(lhs < 32)
        assert(rhs >= 0)
        assert(rhs < 32)
        0x0304 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
    }

    ; jump if signed comparison against zero passes, using the result of an signed `const ? reg` operation
    jmpif.s {addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( {lhs}, ~{rhs} ) -> {
        assert(addr >= 0)
        assert(rhs >= 0)
        assert(rhs < 32)
        0x0102 @ 0x20 @ lhs[31:0] @
        0x0304 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ 0x20 @ rhs[7:0]
    }

    ; jump if signed comparison against zero passes, using the result of an signed `reg ? const` operation
    jmpif.s {addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( ~{lhs}, {rhs} ) -> {
        assert(addr >= 0)
        assert(lhs >= 0)
        assert(lhs < 32)
        0x0102 @ 0x20 @ rhs[31:0] @
        0x0304 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] @ 0x20
    }

    ; dump program counter and registers
    debug -> 0x0f00

    ; print the value of the passed register
    debug ~{reg} -> {
        0x0f01 @ reg[7:0]
    }

    ; jmpif {addr}, {op: alu_cmpop} ( ~{lhs}, ~{rhs} ) -> {
    ; }

    ; jmpifn {addr}, {op: alu_cmpop} ( ~{lhs}, ~{rhs} ) -> {
    ; }
}

R0 = 0
R1 = 1
R2 = 2
R3 = 3
R4 = 4
R5 = 5
R6 = 6
R7 = 7
R8 = 8
R9 = 9
R10 = 10
R11 = 11
R12 = 12
R13 = 13
R14 = 14
R15 = 15
R16 = 16
R17 = 17
R18 = 18
R19 = 19
R20 = 20
R21 = 21
R22 = 22
R23 = 23
R24 = 24
R25 = 25
R26 = 26
R27 = 27
R28 = 28
R29 = 29
R30 = 30
R31 = 31
