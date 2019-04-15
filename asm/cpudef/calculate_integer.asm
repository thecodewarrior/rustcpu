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
    assert(out >= 0)
    assert(out < 32)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0200 @ out[7:0] @ { op[7:0] @ lhs[7:0] }
}

; unsigned `reg ? reg` operation
calc.u %{out}, {op: alu_2op} ( %{lhs}, %{rhs} ) -> {
    assert(out >= 0)
    assert(out < 32)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs >= 0)
    assert(rhs < 32)
    0x0200 @ out[7:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
}

; unsigned `const ? reg` operation
calc.u %{out}, {op: alu_2op} ( {lhs}, %{rhs} ) -> {
    assert(out >= 0)
    assert(out < 32)
    assert(lhs >= 0)
    assert(rhs < 32)
    assert(rhs >= 0)
    0x0102 @ _tmp0[7:0] @ lhs[31:0] @
    0x0200 @ out[7:0] @ op[7:0] @ _tmp0[7:0] @ rhs[7:0]
}

; unsigned `reg ? const` operation
calc.u %{out}, {op: alu_2op} ( %{lhs}, {rhs} ) -> {
    assert(out >= 0)
    assert(out < 32)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs >= 0)
    0x0102 @ _tmp0[7:0] @ rhs[31:0] @
    0x0200 @ out[7:0] @ op[7:0] @ lhs[7:0] @ _tmp0[7:0]
}

; signed unary operation on register
calc.s %{out}, {op: alu_1op} ( %{lhs} ) -> {
    assert(out >= 0)
    assert(out < 32)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0201 @ out[7:0] @ op[7:0] @ lhs[7:0] 
}

; signed `reg ? reg` operation
calc.s %{out}, {op: alu_2op} ( %{lhs}, %{rhs} ) -> {
    assert(out >= 0)
    assert(out < 32)
    assert(lhs >= 0)
    assert(lhs < 32)
    assert(rhs < 32)
    assert(rhs >= 0)
    0x0201 @ out[7:0] @ op[7:0] @ lhs[7:0] @ rhs[7:0]
}

; signed `const ? reg` operation
calc.s %{out}, {op: alu_2op} ( {lhs}, %{rhs} ) -> {
    assert(out >= 0)
    assert(out < 32)
    assert(rhs < 32)
    assert(rhs >= 0)
    0x0102 @ _tmp0[7:0] @ lhs[31:0] @
    0x0201 @ out[7:0] @ op[7:0] @ _tmp0[7:0] @ rhs[7:0]
}

; signed `reg ? const` operation
calc.s %{out}, {op: alu_2op} ( %{lhs}, {rhs} ) -> {
    assert(out >= 0)
    assert(out < 32)
    assert(lhs >= 0)
    assert(lhs < 32)
    0x0102 @ _tmp0[7:0] @ rhs[31:0] @
    0x0201 @ out[7:0] @ op[7:0] @ lhs[7:0] @ _tmp0[7:0]
}