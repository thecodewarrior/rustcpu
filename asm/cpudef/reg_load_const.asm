; load constant into reg
load %{reg}, {value} -> {
    assert(reg >= 0)
    assert(reg < 32)
    _load_insn(reg, value)
}