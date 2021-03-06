Location prefixes:
- `'` -> constant
- `%` -> register
- `>` -> RAM
- `>+` -> stack-relative RAM (stack implementation todo)
- `@` -> ROM

Combinations (`<loc> n => <reference>`):
```
   ' n => n
   > n => ram[n]
  >+ n => ram[stack_ptr + n]
   % n => registers[n]
  >% n => ram[registers[n]]
 >+% n => ram[stack_ptr + registers[n]]
   @ n => rom[n]
  @> n => rom[ram[n]]
 @>+ n => rom[ram[stack_ptr + n]]
  @% n => rom[registers[n]]
 @>% n => rom[ram[registers[n]]]
@>+% n => rom[ram[stack_ptr + registers[n]]]
```

`<location>` = `| 8: type | 8/32: location |` (wide locations always have `| 8 | 32 |` format)

#### NOP - `0x0000`
#### HALT - `0xffff`

### Move - `0x01..`

#### Move 8/16/32 - `0x0100`/`0x0101`/`0x0102`
- **Move 8** - `| 0x0100 | source <read location> | destination <write location> |`
- **Move 16** - `| 0x0101 | source <read location> | destination <write location> |`
- **Move 32** - `| 0x0102 | source <read location> | destination <write location> |`

#### Move block - `0x0103`
`| 0x0103 | source <read location> | destination <write location> | 32: length <read location> |`
- note: if the source is the rom immediately after the instruction, this instruction will jump to the point after it

#### Move string - `0x0104`
`| 0x0104 | source <read location> | destination <write location> |`
- note: if the source is the rom immediately after the instruction, this instruction will jump to the point after it

### Calculate - `0x02..`

#### Calculate u32/i32 - `0x0200`/`0x0201`
- **u32** - `| 0x0200 | 8: op | lhs <read location> | [rhs <read location>] | out <write location> |`
- **i32** - `| 0x0201 | 8: op | lhs <read location> | [rhs <read location>] | out <write location> |`

### Branching - `0x03..`

#### Jump - `0x0300`
`| 0x0300 | target <read location> |`

#### Jump if compare to zero - `0x0301`/`0x0302`
- **u32** - `| 0x0301 | target <read location> | 8: compare op | value <read location> |`
- **i32** - `| 0x0302 | target <read location> | 8: compare op | value <read location> |`

#### Jump if compare calculation to zero - `0x0303`/`0x0304`
- **u32** - `| 0x0303 | target <read location> | 8: compare op | 8: math op | lhs <read location> | [rhs <read location>] |`
- **i32** - `| 0x0304 | target <read location> | 8: compare op | 8: math op | lhs <read location> | [rhs <read location>] |`

#### Jump if compare to another - `0x0305`/`0x0306`
- **u32** - `| 0x0305 | target <read location> | 8: compare op | lhs <read location> | rhs <read location> |`
- **i32** - `| 0x0306 | target <read location> | 8: compare op | lhs <read location> | rhs <read location> |`

### Printing - `0x04..`

#### Print value - `0x0400`/`0x0401`
- **u32** - `| 0x0400 | value <read location> |`
- **i32** - `| 0x0401 | value <read location> |`

#### Print block - `0x0402`
`| 0x0402 | source <read location> | 32: length <read location> |`
- note: if the source is the rom immediately after the instruction, this instruction will jump to the point after it

#### Print string - `0x0403`
`| 0x0403 | source <read location> |`
- note: if the source is the rom immediately after the instruction, this instruction will jump to the point after it

#### Print newline - `0x0404`
`| 0x0404 |`

### Stack - `0x05..`

#### Set stack - `0x0500`
`| 0x0500 | value <wide read location> |`

#### Push frame - `0x0501`
`| 0x0500 | target <wide read location> |`

#### Reserve frame - `0x0502`

### Debug - `0x0f..`

#### Debug dump - `0x0f00`

#### Debug value - `0x0f01`
`| 0x0f01 | value <read location> |`

