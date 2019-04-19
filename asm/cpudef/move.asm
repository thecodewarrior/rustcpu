move8 {src_type: read_location} {src} -> {dest_type: write_location} {dest} => {
    0x0100 @ _location(src_type, src) @ _location(dest_type, dest)
}

move16 {src_type: read_location} {src} -> {dest_type: write_location} {dest} => {
    0x0101 @ _location(src_type, src) @ _location(dest_type, dest)
}

move32 {src_type: read_location} {src} -> {dest_type: write_location} {dest} => {
    0x0102 @ _location(src_type, src) @ _location(dest_type, dest)
}

move_block {src_type: read_block_location} {src}, {len_type: read_location} {len} -> {dest_type: write_block_location} {dest} => {
    0x0103 @ _location(src_type, src) @ _location(dest_type, dest) @ _location(len_type, len)
}

move_str {src_type: read_block_location} {src} -> {dest_type: write_block_location} {dest} => {
    0x0104 @ _location(src_type, src) @ _location(dest_type, dest)
}