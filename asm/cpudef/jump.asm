#tokendef jump_cmpz
{
    ltz : 0
    lez : 1
    eqz : 2
    gez : 3
    gtz : 4
    nez : 5
}

#tokendef jump_cmp
{
    lt  : 0
    leq : 1
    eq  : 2
    geq : 3
    gt  : 4
    neq : 5
}

; jump to passed address
jump {target_loc: not_rom_location} {target} => {
    0x0300 @ _location(target_loc, target)
}

; jump if unsigned comparison against zero passes
jumpif.u {cmp: jump_cmpz} ( {value_loc: read_location} {value} ) -> {target_loc: not_rom_location} {target} => {
    0x0301 @ _location(target_loc, target) @ _8(cmp) @ _location(value_loc, value)
}

; jump if signed comparison against zero passes
jumpif.s {cmp: jump_cmpz} ( {value_loc: read_location} {value} ) -> {target_loc: not_rom_location} {target} => {
    0x0302 @ _location(target_loc, target) @ _8(cmp) @ _location(value_loc, value)
}

; jump if unsigned comparison against zero passes, using the result of an unsigned unary operation
jumpif.u {cmp: jump_cmpz} calc {op: alu_1op} ( {value_loc: read_location} {value} ) -> {target_loc: not_rom_location} {target} => {
    0x0303 @ _location(target_loc, target) @ _8(cmp) @ _8(op) @ _location(value_loc, value)
}

; jump if unsigned comparison against zero passes, using the result of an signed unary operation
jumpif.s {cmp: jump_cmpz} calc {op: alu_1op} ( {value_loc: read_location} {value} ) -> {target_loc: not_rom_location} {target} => {
    0x0304 @ _location(target_loc, target) @ _8(cmp) @ _8(op) @ _location(value_loc, value)
}

; jump if unsigned comparison against zero passes, using the result of an unsigned binary operation
jumpif.u {cmp: jump_cmpz} calc {op: alu_2op} ( {lhs_loc: read_location} {lhs}, {rhs_loc: read_location} {rhs} ) -> {target_loc: not_rom_location} {target} => {
    0x0303 @ _location(target_loc, target) @ _8(cmp) @ _8(op) @ _location(lhs_loc, lhs) @ _location(rhs_loc, rhs)
}

; jump if signed comparison against zero passes, using the result of an signed binary operation
jumpif.s {cmp: jump_cmpz} calc {op: alu_2op} ( {lhs_loc: read_location} {lhs}, {rhs_loc: read_location} {rhs} ) -> {target_loc: not_rom_location} {target} => {
    0x0304 @ _location(target_loc, target) @ _8(cmp) @ _8(op) @ _location(lhs_loc, lhs) @ _location(rhs_loc, rhs)
}

; jump if unsigned comparison passes
jumpif.u ( {lhs_loc: read_location} {lhs}, {cmp: jump_cmp}, {rhs_loc: read_location} {rhs} ) -> {target_loc: not_rom_location} {target} => {
    0x0305 @ _location(target_loc, target) @ _8(cmp) @ _location(lhs_loc, lhs) @ _location(rhs_loc, rhs)
}

; jump if signed comparison passes
jumpif.s ( {lhs_loc: read_location} {lhs}, {cmp: jump_cmp}, {rhs_loc: read_location} {rhs} ) -> {target_loc: not_rom_location} {target} => {
    0x0306 @ _location(target_loc, target) @ _8(cmp) @ _location(lhs_loc, lhs) @ _location(rhs_loc, rhs)
}
