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
    lt = 0
    leq = 1
    eq = 2
    geq = 3
    gt = 4
    ne = 5
}

; unsigned unary operation on register
calc.u %{out}, {op: alu_1op} ( %{lhs} ) -> {
    _valid_reg(out, lhs)
    0x0200 @ _8(out, op, lhs)
}

; unsigned `reg ? reg` operation
calc.u %{out}, {op: alu_2op} ( %{lhs}, %{rhs} ) -> {
    _valid_reg(out, lhs, rhs)
    0x0200 @ _8(out, op, lhs, rhs)
}

; unsigned `const ? reg` operation
calc.u %{out}, {op: alu_2op} ( {lhs}, %{rhs} ) -> {
    _valid_reg(out, rhs)
    assert(lhs >= 0)

    _tmp0(lhs) @
    0x0200 @ _8(out, op, _tmp0, rhs)
}

; unsigned `reg ? const` operation
calc.u %{out}, {op: alu_2op} ( %{lhs}, {rhs} ) -> {
    _valid_reg(out, lhs)
    assert(rhs >= 0)

    _tmp0(rhs) @
    0x0200 @ _8(out, op, lhs, _tmp0)
}

; signed unary operation on register
calc.s %{out}, {op: alu_1op} ( %{lhs} ) -> {
    _valid_reg(out, lhs)
    0x0201 @ _8(out, op, lhs)
}

; signed `reg ? reg` operation
calc.s %{out}, {op: alu_2op} ( %{lhs}, %{rhs} ) -> {
    _valid_reg(out, lhs, rhs)
    0x0201 @ _8(out, op, lhs, rhs)
}

; signed `const ? reg` operation
calc.s %{out}, {op: alu_2op} ( {lhs}, %{rhs} ) -> {
    _valid_reg(out, rhs)
    _tmp0(lhs) @
    0x0201 @ _8(out, op, _tmp0, rhs)
}

; signed `reg ? const` operation
calc.s %{out}, {op: alu_2op} ( %{lhs}, {rhs} ) -> {
    _valid_reg(out, lhs)
    _tmp0(rhs) @
    0x0201 @ _8(out, op, lhs, tmp0)
}