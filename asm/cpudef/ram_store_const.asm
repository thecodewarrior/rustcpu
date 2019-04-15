; store constant byte in ram
store.b >{addr}, {value} -> {
    0x0103 @ addr[31:0] @ value[7:0]
}

; store constant short in ram
store.s >{addr}, {value} -> {
    0x0104 @ addr[31:0] @ value[15:0]
}

; store constant int in ram
store.i >{addr}, {value} -> {
    0x0105 @ addr[31:0] @ value[31:0]
}

; store constant data in ram
store.d >{addr}, @{loc}, {len} -> {
    0x0106 @ addr[31:0] @ loc[31:0] @ len[31:0]
}

; store constant subsequent data in ram
store.d >{addr}, @{end} -> {
    start = pc + (2 + 4*3) + (2 + 4)
    assert(end >= start)
    len = end - start
    0x0106 @ addr[31:0] @ start[31:0] @ len[31:0] @
    0x0300 @ end[31:0]
}

; store constant string data in ram
store.s >{addr} -> {
    0x0107 @ addr[31:0]
}