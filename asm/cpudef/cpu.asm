#include "vars.asm"

#cpudef
{
    #bits 8

    ; Value prefixes
    ; RAM => `>___`
    ; Register => `%___`
    ; Program address => `@___`
    ; Whitespace can appear between prefixes and numbers

    ; singular examples:
    ; % 3    => registers[3]
    ; > 0xff => ram[0xff]
    ; @ 0xff => rom[0x10]
    ; >% 3   => ram[ registers[3] ]
    ; @% 3   => rom[ registers[3] ]

    ; read-write: %, >, >%
    ; read-only: @, @%

    ; - Base
    nop -> 0x0000
    halt -> 0xffff

    #include "_functions.asm"
    #include "reg_load_const.asm"
    #include "ram_store_const.asm"
    #include "move_reg2ram.asm"
    #include "calculate_integer.asm"
    #include "jump.asm"
    #include "debug.asm"
}