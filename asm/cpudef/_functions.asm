#fun _8(value) => { 
    assert(value < 256)
    assert(value >= -128)
    value[8:0] 
}
#fun _16(value) => {
    assert(value < 65_536)
    assert(value >= -32_768)
    value[16:0] 
}
#fun _32(value) => {
    assert(value < 4_294_967_296)
    assert(value >= -2_147_483_648)
    value[32:0] 
}

#fun _8(value1, value2) => { _8(value1) @ _8(value2) }
#fun _16(value1, value2) => { _16(value1) @ _16(value2) }
#fun _32(value1, value2) => { _32(value1) @ _32(value2) }

#fun _8(value1, value2, value3) => { _8(value1) @ _8(value2) @ _8(value3) }
#fun _16(value1, value2, value3) => { _16(value1) @ _16(value2) @ _16(value3) }
#fun _32(value1, value2, value3) => { _32(value1) @ _32(value2) @ _32(value3) }

#fun _8(value1, value2, value3, value4) => { _8(value1) @ _8(value2) @ _8(value3) @ _8(value4) }
#fun _16(value1, value2, value3, value4) => { _16(value1) @ _16(value2) @ _16(value3) @ _16(value4) }
#fun _32(value1, value2, value3, value4) => { _32(value1) @ _32(value2) @ _32(value3) @ _32(value4) }

#fun _8(value1, value2, value3, value4, value5) => { _8(value1) @ _8(value2) @ _8(value3) @ _8(value4) @ _8(value5) }
#fun _16(value1, value2, value3, value4, value5) => { _16(value1) @ _16(value2) @ _16(value3) @ _16(value4) @ _16(value5) }
#fun _32(value1, value2, value3, value4, value5) => { _32(value1) @ _32(value2) @ _32(value3) @ _32(value4) @ _32(value5) }

; sub-byte data only available in whole-byte lengths
#fun _4(value1, value2) => { value1[4:0] @ value2[4:0] }
#fun _4(value1, value2, value3, value4) => { _4(value1, value2) @ _4(value3, value4) }
#fun _2(value1, value2, value3, value4) => { value1[2:0] @ value2[2:0] @ value3[2:0] @ value4[2:0] }

#fun _valid_reg(reg) => { 
    assert(reg >= 0)
    assert(reg < 32)
}
#fun _valid_special_reg(reg) => { 
    assert(reg >= 0)
    assert(reg < 2)
}
#fun _valid_reg(reg1, reg2) => { 
    _valid_reg(reg1)
    _valid_reg(reg2)
}
#fun _valid_reg(reg1, reg2, reg3) => { 
    _valid_reg(reg1)
    _valid_reg(reg2)
    _valid_reg(reg3)
}
#fun _valid_reg(reg1, reg2, reg3, reg4) => { 
    _valid_reg(reg1)
    _valid_reg(reg2)
    _valid_reg(reg3)
    _valid_reg(reg4)
}


#fun _set_reg(reg, value) => {
    0x0102 @ _8(reg) @ _32(value)
}

#fun _save_ram(ram_addr, reg) => {
    0x0401 @ _8(addr_reg) @ _8(value_reg)
}

#fun _read_ram(reg, ram_addr) => {
    0x0402 @ _8(addr_reg) @ _8(value_reg)
}

#fun _tmp0(value) => _set_reg(_tmp0, value)
#fun _tmp1(value) => _set_reg(_tmp1, value)
#fun _tmp2(value) => _set_reg(_tmp2, value)
#fun _tmp3(value) => _set_reg(_tmp3, value)
#fun _tmp4(value) => _set_reg(_tmp4, value)
#fun _tmp5(value) => _set_reg(_tmp5, value)
#fun _tmp6(value) => _set_reg(_tmp6, value)
#fun _tmp7(value) => _set_reg(_tmp7, value)

#tokendef read_location {
    '    : 0b0000
    >    : 0b0001
    ^    : 0b0010
    %    : 0b0011
    >%   : 0b0100
    ^%   : 0b0101
    @    : 0b0111
    @>   : 0b1000
    @^   : 0b1001
    @%   : 0b1010
    @>%  : 0b1011
    @^%  : 0b1100
    ::   : 0b1101
    ^^   : 0b1110
}

#tokendef write_location {
    >   : 0b0001
    ^   : 0b0010
    %   : 0b0011
    >%  : 0b0100
    ^%  : 0b0101
    ::  : 0b1101
    ^^  : 0b1110
}

#tokendef read_block_location {
    >    : 0b0001
    ^    : 0b0010
    >%   : 0b0100
    ^%   : 0b0101
    @    : 0b0111
    @>   : 0b1000
    @^   : 0b1001
    @%   : 0b1010
    @>%  : 0b1011
    @^%  : 0b1100
    ^^   : 0b1110
}

#tokendef write_block_location {
    >    : 0b0001
    ^    : 0b0010
    >%   : 0b0100
    ^%   : 0b0101
    ^^   : 0b1110
}

#tokendef not_rom_location {
    '    : 0b0000
    >    : 0b0001
    ^    : 0b0010
    %    : 0b0011
    >%   : 0b0100
    ^%   : 0b0101
    ::   : 0b1101
    ^^   : 0b1110
}

#fun _location(loc, value) => {
    assert(loc >= 0)
    assert(loc <= 0b1110)
    _8(loc) @ { 
        (
            loc == 0b0011 || loc == 0b0100 || loc == 0b0101 || loc == 0b1010 || loc == 0b1011 || loc == 0b1100
        ) ? { 
            _valid_reg(value)
            _8(value) 
        } : (loc == 0b1101) ? {
            _valid_special_reg(value)
            _8(value) 
        } : {
            _32(value) 
        }
    }
}

; always 5 bytes / 40 bits
#fun _wide_location(loc, value) => {
    assert(loc >= 0)
    assert(loc <= 6)
    _8(loc + 0b1000_0000) @ { 
        (
            loc == 0b0011 || loc == 0b0100 || loc == 0b0101 || loc == 0b1010 || loc == 0b1011 || loc == 0b1100
        ) ? { 
            _valid_reg(value)
            _32(value) 
        } : (loc == 0b1101) ? {
            _valid_special_reg(value)
            _8(value) 
        }:{
            _32(value) 
        }
    }
}
