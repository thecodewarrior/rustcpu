#tokendef alu_2op 
{
    add : 0
    sub : 1
    mul : 2
    div : 3
    mod : 4
    shl : 6
    shr : 7
    rotl : 8
    rotr : 9
    bit_and : 10
    bit_or : 11
    bit_xor : 12
    bool_and : 13
    bool_or : 14
    bool_xor : 15
    cmp_eq : 16
    cmp_ne : 17
    cmp_gt : 18
    cmp_lt : 19
    cmp_geq : 20
    cmp_leq : 21
}

#tokendef alu_1op 
{
    neg : 22
    bit_not : 23
    bool_not : 24
}

#tokendef alu_cmpop
{
    lt : 0
    leq : 1
    eq : 2
    geq : 3
    gt : 4
    ne : 5
}

calc.u {op: alu_1op} ( {in_loc: read_location} {in} ) -> {out_loc: write_location} {out} => {
    0x0200 @ _8(op) @ _location(in_loc, in) @ _location(out_loc, out)
}

calc.s {op: alu_1op} ( {in_loc: read_location} {in} ) -> {out_loc: write_location} {out} => {
    0x0201 @ _8(op) @ _location(in_loc, in) @ _location(out_loc, out)
}

calc.u {op: alu_2op} ( {lhs_loc: read_location} {lhs}, {rhs_loc: read_location} {rhs} ) -> {out_loc: write_location} {out} => {
    0x0200 @ _8(op) @ _location(lhs_loc, lhs) @ _location(rhs_loc, rhs) @ _location(out_loc, out)
}

calc.s {op: alu_2op} ( {lhs_loc: read_location} {lhs}, {rhs_loc: read_location} {rhs} ) -> {out_loc: write_location} {out} => {
    0x0201 @ _8(op) @ _location(lhs_loc, lhs) @ _location(rhs_loc, rhs) @ _location(out_loc, out)
}