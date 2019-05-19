call {loc: read_location} {data} => {
    0x0500 @ _wide_location(loc, data)
}

call {loc: read_location} {data} -> {return_loc: write_location} {write} => {
    0x0500 @ _wide_location(loc, data) @
        0x0505 @ _location(return_loc, write) @ _location(0, 4)
}

call {loc: read_location} {data} -> {return_loc: write_block_location} {write}, {count_loc: read_location} {count} => {
    0x0500 @ _wide_location(loc, data) @
        0x0505 @ _location(return_loc, write) @ _location(count_loc, count)
}

return => {
    0x0501
}

return {loc: read_location} {data} => {
    0x0502 @ _location(loc, data) @ _location(0, 4)
}

return {loc: read_location} {data}, {count_loc: read_location} {count} => {
    0x0502 @ _location(loc, data) @ _location(count_loc, count)
}

push {loc: read_location} {data} => {
    0x0503 @ _location(loc, data) @ _location(0, 4)
}

push {loc: read_location} {data}, {count_loc: read_location} {count} => {
    0x0503 @ _location(loc, data) @ _location(count_loc, count)
}

drop => {
    0x0504 @ _location(0, 4)
}

drop {count_loc: read_location} {count} => {
    0x0504 @ _location(count_loc, count)
}

pop {loc: write_location} {data} => {
    0x0505 @ _location(loc, data) @ _location(0, 4)
}

pop {loc: write_block_location} {data}, {count_loc: read_location} {count_data} => {
    0x0505 @ _location(loc, data) @ _location(count_loc, count_data)
}
