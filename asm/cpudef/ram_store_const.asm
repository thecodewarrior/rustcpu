; store constant byte in ram
store.b >{addr}, {value} -> {
    _tmp0(addr) @
    0x0103 @ _8(_tmp0, value)
}

; store constant byte in ram
store.b >%{addr}, {value} -> {
    _valid_reg(addr)
    0x0103 @ _8(addr, value)
}

; store constant short in ram
store.s >{addr}, {value} -> {
    _tmp0(addr) @
    0x0104 @ _8(_tmp0) @ _16(value)
}

; store constant byte in ram
store.s >%{addr}, {value} -> {
    _valid_reg(addr)
    0x0104 @ _8(addr) @ _16(value)
}

; store constant int in ram
store.i >{addr}, {value} -> {
    _tmp0(addr) @
    0x0105 @ _8(_tmp0) @ _32(value)
}

; store constant byte in ram
store.i >%{addr}, {value} -> {
    _valid_reg(addr)
    0x0105 @ _8(addr) @ _32(value)
}

; store constant data in ram
store.d >{addr}, @{loc}, {len} -> {
    _tmp0(addr) @
    0x0106 @ _8(_tmp0) @ _32(loc, len)
}

; store constant data in ram
store.d >%{addr}, @{loc}, {len} -> {
    _valid_reg(addr)
    0x0106 @ _8(addr) @ _32(loc, len)
}

; store constant subsequent data in ram
store.d >{addr}, @{end} -> {
    assert(end >= _insn_end)
    _tmp0(addr) @
    0x0106 @ _8(_tmp0) @ _32(_insn_end, end - _insn_end) @
    0x0300 @ _32(end)
}

; store constant subsequent data in ram
store.d >%{addr}, @{end} -> {
    assert(end >= _insn_end)
    _valid_reg(addr)
    0x0106 @ _8(addr) @ _32(_insn_end, end - _insn_end) @
    0x0300 @ _32(end)
}

; store constant string data in ram
store.s >{addr} -> {
    _tmp0(addr) @
    0x0107 @ _8(_tmp0) @ _32(_insn_end)
}

; store constant string data in ram
store.s >%{addr} -> {
    _valid_reg(addr)
    0x0107 @ _8(addr) @ _32(_insn_end)
}
