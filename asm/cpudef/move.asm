mov8 {src_type: read_location} {src} -> {dest_type: write_location} {dest} => {
    0x0100 @ _locations(src_type, src, dest_type, dest)
}

mov16 {src_type: read_location} {src} -> {dest_type: write_location} {dest} => {
    0x0101 @ _locations(src_type, src, dest_type, dest)
}

mov32 {src_type: read_location} {src} -> {dest_type: write_location} {dest} => {
    0x0102 @ _locations(src_type, src, dest_type, dest)
}

mov %{src} -> %{dest} => {
    0x0102 @ _locations(_location_register, src, _location_register, dest)
}

mov '{src} -> %{dest} => {
    0x0102 @ _locations(_location_const, src, _location_register, dest)
}