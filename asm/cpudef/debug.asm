; dump program counter and registers
debug -> 0x0f00

; print the value of the passed register
debug %{reg} -> {
    0x0f01 @ _8(reg)
}