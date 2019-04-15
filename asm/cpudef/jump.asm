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
    0x0300 @ _32(addr)
}

; jump if unsigned comparison against zero passes
jmpif.u @{addr}, {cmp: jmp_cmp} ( %{lhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0301 @ _32(addr) @ _8(cmp) @ _8(lhs) 
}

; jump if signed comparison against zero passes
jmpif.s @{addr}, {cmp: jmp_cmp} ( %{lhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0302 @ _32(addr) @ _8(cmp) @ _8(lhs) 
}

; jump if unsigned comparison against zero passes, using the result of an unsigned unary operation
jmpif.u @{addr}, {cmp: jmp_cmp} calc {op: alu_1op} ( %{lhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0302 @ _32(addr) @ _8(cmp) @ _8(op) @ _8(lhs) 
}

; jump if unsigned comparison against zero passes, using the result of an unsigned `reg ? reg` operation
jmpif.u @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( %{lhs}, %{rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0303 @ _32(addr) @ _8(cmp) @ _8(op) @ _8(lhs) @ _8(rhs)
}

; jump if unsigned comparison against zero passes, using the result of an unsigned `const ? reg` operation
jmpif.u @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( {lhs}, %{rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0102 @ _8(_tmp0) @ _32(lhs) @
    0x0303 @ _32(addr) @ _8(cmp) @ _8(op) @ _8(_tmp0) @ _8(rhs)
}

; jump if unsigned comparison against zero passes, using the result of an unsigned `reg ? const` operation
jmpif.u @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( %{lhs}, {rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs >= 0)
    0x0102 @ _8(_tmp0) @ _32(rhs) @
    0x0303 @ _32(addr) @ _8(cmp) @ _8(op) @ _8(lhs) @ _8(_tmp0)
}

; jump if signed comparison against zero passes, using the result of an signed unary operation
jmpif.s @{addr}, {cmp: jmp_cmp} calc {op: alu_1op} ( %{lhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0304 @ _32(addr) @ _8(cmp) @ _8(op) @ _8(lhs) 
}

; jump if signed comparison against zero passes, using the result of an signed `reg ? reg` operation
jmpif.s @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( %{lhs}, %{rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0304 @ _32(addr) @ _8(cmp) @ _8(op) @ _8(lhs) @ _8(rhs)
}

; jump if signed comparison against zero passes, using the result of an signed `const ? reg` operation
jmpif.s @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( {lhs}, %{rhs} ) -> {
    assert(addr >= 0)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0102 @ _8(_tmp0) @ _32(lhs) @
    0x0304 @ _32(addr) @ _8(cmp) @ _8(op) @ _8(_tmp0) @ _8(rhs)
}

; jump if signed comparison against zero passes, using the result of an signed `reg ? const` operation
jmpif.s @{addr}, {cmp: jmp_cmp} calc {op: alu_2op} ( %{lhs}, {rhs} ) -> {
    assert(addr >= 0)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0102 @ _8(_tmp0) @ _32(rhs) @
    0x0304 @ _32(addr) @ _8(cmp) @ _8(op) @ _8(lhs) @ _8(_tmp0)
}

; jump if unsigned `reg ? reg` comparison passes
jmpif.u @{addr}, ( %{lhs}, {op: alu_cmpop}, %{rhs} ) -> {
    0x0305 @ _32(addr) @ _8(op) @ _8(lhs) @ _8(rhs)
}

; jump if unsigned `const ? reg` comparison passes
jmpif.u @{addr}, ( {lhs}, {op: alu_cmpop}, %{rhs} ) -> {
    0x0102 @ _8(_tmp0) @ _32(lhs) @
    0x0305 @ _32(addr) @ _8(op) @ _8(_tmp0) @ _8(rhs)
}

; jump if unsigned `reg ? const` comparison passes
jmpif.u @{addr}, ( %{lhs}, {op: alu_cmpop}, {rhs} ) -> {
    0x0102 @ _8(_tmp0) @ _32(rhs) @
    0x0305 @ _32(addr) @ _8(op) @ _8(lhs) @ _8(_tmp0)
}

; jump if signed `reg ? reg` comparison passes
jmpif.s @{addr}, ( %{lhs}, {op: alu_cmpop}, %{rhs} ) -> {
    0x0306 @ _32(addr) @ _8(op) @ _8(lhs) @ _8(rhs)
}

; jump if signed `const ? reg` comparison passes
jmpif.s @{addr}, ( {lhs}, {op: alu_cmpop}, %{rhs} ) -> {
    0x0102 @ _8(_tmp0) @ _32(lhs) @
    0x0306 @ _32(addr) @ _8(op) @ _8(_tmp0) @ _8(rhs)
}

; jump if signed `reg ? const` comparison passes
jmpif.s @{addr}, ( %{lhs}, {op: alu_cmpop}, {rhs} ) -> {
    0x0102 @ _8(_tmp0) @ _32(rhs) @
    0x0306 @ _32(addr) @ _8(op) @ _8(lhs) @ _8(_tmp0)
}