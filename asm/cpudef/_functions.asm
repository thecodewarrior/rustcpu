#fun _8(value) -> { value[7:0] }
#fun _16(value) -> { value[15:0] }
#fun _32(value) -> { value[31:0] }

#fun _load_insn(reg, value) ->
{
    0x0102 @ _8(reg) @ value[31:0]
}