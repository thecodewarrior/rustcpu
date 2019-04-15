; load constant into reg
load %{reg}, {value} -> {
    _valid_reg(reg)
    _load(reg, value)
}