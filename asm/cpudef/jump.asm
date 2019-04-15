#tokendef jmp_cmp
{
    ltz = 0
    lez = 1
    eqz = 2
    gez = 3
    gtz = 4
    nez = 5
}

; jump to passed address/label
jmp @{addr} -> {
    assert(addr >= 0)
    0x0300 @ addr[31:0]
}

; jump if unsigned comparison against zero passes
jmpif.u @{addr}, {cmp: jmp_cmp} ( %{lhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0301 @ addr[31:0] @ cmp[7:0] @ lhs[7:0] 
}

; jump if signed comparison against zero passes
jmpif.s @{addr}, {cmp: jmp_cmp} ( %{lhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0302 @ addr[31:0] @ cmp[7:0] @ lhs[7:0] 
}

; jump if unsigned comparison against zero passes, using the result of an unsigned unary operation
jmpif.u @{addr}, {cmp: jmp_cmp} calc {op: alu_1op} ( %{lhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0302 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] 
}

; jump if unsigned comparison against zero passes, using the result of an unsigned `reg ? reg` operation
jmpif.u @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( %{lhs}, %{rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0303 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
}

; jump if unsigned comparison against zero passes, using the result of an unsigned `const ? reg` operation
jmpif.u @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( {lhs}, %{rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0102 @ _tmp0[7:0] @ lhs[31:0] @
    0x0303 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ _tmp0[7:0] @ rhs[7:0]
}

; jump if unsigned comparison against zero passes, using the result of an unsigned `reg ? const` operation
jmpif.u @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( %{lhs}, {rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs >= 0)
    0x0102 @ _tmp0[7:0] @ rhs[31:0] @
    0x0303 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] @ _tmp0[7:0]
}

; jump if signed comparison against zero passes, using the result of an signed unary operation
jmpif.s @{addr}, {cmp: jmp_cmp} calc {op: alu_1op} ( %{lhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0304 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] 
}

; jump if signed comparison against zero passes, using the result of an signed `reg ? reg` operation
jmpif.s @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( %{lhs}, %{rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0304 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
}

; jump if signed comparison against zero passes, using the result of an signed `const ? reg` operation
jmpif.s @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( {lhs}, %{rhs} ) -> {
    assert(addr >= 0)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0102 @ _tmp0[7:0] @ lhs[31:0] @
    0x0304 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ _tmp0[7:0] @ rhs[7:0]
}

; jump if signed comparison against zero passes, using the result of an signed `reg ? const` operation
jmpif.s @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( %{lhs}, {rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0102 @ _tmp0[7:0] @ rhs[31:0] @
    0x0304 @ addr[31:0] @ cmp[7:0] @ op[7:0] @ lhs[7:0] @ _tmp0[7:0]
}

; jump if unsigned `reg ? reg` comparison passes
jmpif.u @{addr}, ( %{lhs}, {op: alu_cmpop}, %{rhs} ) -> {
    0x0305 @ addr[31:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
}

; jump if unsigned `const ? reg` comparison passes
jmpif.u @{addr}, ( {lhs}, {op: alu_cmpop}, %{rhs} ) -> {
    0x0102 @ _tmp0[7:0] @ lhs[31:0] @
    0x0305 @ addr[31:0] @ op[7:0] @ _tmp0[7:0] @ rhs[7:0]
}

; jump if unsigned `reg ? const` comparison passes
jmpif.u @{addr}, ( %{lhs}, {op: alu_cmpop}, {rhs} ) -> {
    0x0102 @ _tmp0[7:0] @ rhs[31:0] @
    0x0305 @ addr[31:0] @ op[7:0] @ lhs[7:0] @ _tmp0[7:0]
}

; jump if signed `reg ? reg` comparison passes
jmpif.s @{addr}, ( %{lhs}, {op: alu_cmpop}, %{rhs} ) -> {
    0x0306 @ addr[31:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
}

; jump if signed `const ? reg` comparison passes
jmpif.s @{addr}, ( {lhs}, {op: alu_cmpop}, %{rhs} ) -> {
    0x0102 @ _tmp0[7:0] @ lhs[31:0] @
    0x0306 @ addr[31:0] @ op[7:0] @ _tmp0[7:0] @ rhs[7:0]
}

; jump if signed `reg ? const` comparison passes
jmpif.s @{addr}, ( %{lhs}, {op: alu_cmpop}, {rhs} ) -> {
    0x0102 @ _tmp0[7:0] @ rhs[31:0] @
    0x0306 @ addr[31:0] @ op[7:0] @ lhs[7:0] @ _tmp0[7:0]
}