; dump program counter and registers
debug => 0x0f00

; print the value of the passed register
debug {loc: read_location} {data} => {
    0x0f01 @ _location(loc, data)
}