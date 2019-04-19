print.u {loc: read_location} {data} => {
    0x0400 @ _location(loc, data)
}

print.s {loc: read_location} {data} => {
    0x0401 @ _location(loc, data)
}

print_block {loc: read_location} {data}, {len_loc: read_location} {len} => 0x0402 @ _location(loc, data) @ _location(len_loc, len)

print_str {loc: read_location} {data} => 0x0403 @ _location(loc, data)
print_str => 0x0403 @ _8(0b0111) @ _insn_end[32:0]

print_nl => 0x0404

print_dyn {foo} => {
    foo == 0 ? 0x00 : 0x00ff
}