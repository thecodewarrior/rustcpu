#include "vars.asm"

#cpudef
{
    #bits 8

    ; Value prefixes
    ; Constant => `'___`
    ; RAM => `>___`
    ; Stack relative => `>+___`
    ; Register => `%___`
    ; Program address => `@___`
    ; Whitespace inside and around the prefix is ignored

    ; examples:
    ; '   0xff => 0xff
    ; @   0xff => rom[0xff]
    ; %   3    => registers[3]
    ; >   0xff => ram[ 0xff ]
    ; >%  3    => ram[ registers[3] ]
    ; >+  0x10 => ram[ stack_ptr + 0x10 ]
    ; >+% 3    => ram[ stack_ptr + registers[3] ]

    ; read-write: %, >, >%
    ; read-only: @, @%

    ; - Base
    nop => 0x0000
    halt => 0xffff

    #include "_functions.asm"
    #include "move.asm"
    #include "math.asm"
    #include "jump.asm"
    #include "debug.asm"
    #include "print.asm"
}