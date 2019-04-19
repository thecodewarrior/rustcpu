; load constant into reg
load %{reg} <- {value} => {
    _valid_reg(reg)
    _set_reg(reg, value)
}

; store constant byte in ram
store8 {to: data_location} {dest} <- {from: data_location} {source} => {
    0x0103 @ _locations(to, dest, from, source)
}

; store constant byte in ram
; store8 >%{addr} <- {value} => {
;     _valid_reg(addr)
;     0x0103 @ _8(addr, value)
; }

; store constant short in ram
store16 >{addr} <- {value} => {
    _tmp0(addr) @
    0x0104 @ _8(_tmp0) @ _16(value)
}

; store constant byte in ram
store16 >%{addr} <- {value} => {
    _valid_reg(addr)
    0x0104 @ _8(addr) @ _16(value)
}

; store constant int in ram
store32 >{addr} <- {value} => {
    _tmp0(addr) @
    0x0105 @ _8(_tmp0) @ _32(value)
}

; store constant byte in ram
store32 >%{addr} <- {value} => {
    _valid_reg(addr)
    0x0105 @ _8(addr) @ _32(value)
}

; store constant data in ram
store.bytes >{addr} <- @{loc}, {len} => {
    _tmp0(addr) @
    0x0106 @ _8(_tmp0) @ _32(loc, len)
}

; store constant data in ram
store.bytes >%{addr} <- @{loc}, {len} => {
    _valid_reg(addr)
    0x0106 @ _8(addr) @ _32(loc, len)
}

; store constant subsequent data in ram
store.bytes >{addr} <- @{end} => {
    assert(end >= _insn_end)
    _tmp0(addr) @
    0x0106 @ _8(_tmp0) @ _32(_insn_end, end - _insn_end) @
    0x0300 @ _32(end)
}

; store constant subsequent data in ram
store.bytes >%{addr} <- @{end} => {
    assert(end >= _insn_end)
    _valid_reg(addr)
    0x0106 @ _8(addr) @ _32(_insn_end, end - _insn_end) @
    0x0300 @ _32(end)
}

; store constant string data in ram
store.string >{addr} <- => {
    _tmp0(addr) @
    0x0107 @ _8(_tmp0) @ _32(_insn_end)
}

; store constant string data in ram
store.string >%{addr} <- => {
    _valid_reg(addr)
    0x0107 @ _8(addr) @ _32(_insn_end)
}

var_width {v} => {
    v < 256 ? _8(v) :
    v < 65536 ? _16(v) :
    v < 4294967296 ? _32(v) :
    assert(false)
}