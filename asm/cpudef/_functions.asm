#fun _8(value) -> { value[7:0] }
#fun _16(value) -> { value[15:0] }
#fun _32(value) -> { value[31:0] }

#fun _8(value1, value2) -> { _8(value1) @ _8(value2) }
#fun _16(value1, value2) -> { _16(value1) @ _16(value2) }
#fun _32(value1, value2) -> { _32(value1) @ _32(value2) }

#fun _8(value1, value2, value3) -> { _8(value1) @ _8(value2) @ _8(value3) }
#fun _16(value1, value2, value3) -> { _16(value1) @ _16(value2) @ _16(value3) }
#fun _32(value1, value2, value3) -> { _32(value1) @ _32(value2) @ _32(value3) }

#fun _8(value1, value2, value3, value4) -> { _8(value1) @ _8(value2) @ _8(value3) @ _8(value4) }
#fun _16(value1, value2, value3, value4) -> { _16(value1) @ _16(value2) @ _16(value3) @ _16(value4) }
#fun _32(value1, value2, value3, value4) -> { _32(value1) @ _32(value2) @ _32(value3) @ _32(value4) }

#fun _8(value1, value2, value3, value4, value5) -> { _8(value1) @ _8(value2) @ _8(value3) @ _8(value4) @ _8(value5) }
#fun _16(value1, value2, value3, value4, value5) -> { _16(value1) @ _16(value2) @ _16(value3) @ _16(value4) @ _16(value5) }
#fun _32(value1, value2, value3, value4, value5) -> { _32(value1) @ _32(value2) @ _32(value3) @ _32(value4) @ _32(value5) }

#fun _valid_reg(reg) -> { 
    assert(reg >= 0)
    assert(reg < 32)
}
#fun _valid_reg(reg1, reg2) -> { 
    _valid_reg(reg1)
    _valid_reg(reg2)
}
#fun _valid_reg(reg1, reg2, reg3) -> { 
    _valid_reg(reg1)
    _valid_reg(reg2)
    _valid_reg(reg3)
}
#fun _valid_reg(reg1, reg2, reg3, reg4) -> { 
    _valid_reg(reg1)
    _valid_reg(reg2)
    _valid_reg(reg3)
    _valid_reg(reg4)
}


#fun _load(reg, value) ->
{
    0x0102 @ _8(reg) @ _32(value)
}

#fun _tmp0(value) -> _load(_tmp0, value)
#fun _tmp1(value) -> _load(_tmp1, value)
#fun _tmp2(value) -> _load(_tmp2, value)
#fun _tmp3(value) -> _load(_tmp3, value)
#fun _tmp4(value) -> _load(_tmp4, value)
#fun _tmp5(value) -> _load(_tmp5, value)
#fun _tmp6(value) -> _load(_tmp6, value)
#fun _tmp7(value) -> _load(_tmp7, value)