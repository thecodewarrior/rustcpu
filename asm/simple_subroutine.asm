push '0  ; reserve and clear a slot on the stack for the output
.out = 0

push '4 ; arg 1
push '1 ; arg 2

; call print_sum, then pop its return value off the stack and put it in ^.out
; this is one instruction for convenience, you could just as well do a plain `call 'print_sum` then `pop ^.out`
call 'print_sum -> ^.out 

drop '8 ; drop the two arguments

print_str ; print "Print return "
#str "Print return \0"
print.u ^.out ; print the unsigned int at ^.out
print_nl ; newline

halt ; done

print_sum:
    .a = param_start-8 ; parameters by convention are before the frame start
    .b = param_start-4 ; These are just variables for the assembler, they don't become instructions
                       ; param_start is just -8, and corresponds to the amount of header info added by the `call` insn
                       ; In this case the return address and return frame address

    push '0 ; reserve a and clear a slot on the stack for the result
    .result = 0

    calc.u add (^.a, ^.b) -> ^.result ; ^___ is relative to the stack frame. ^^___ is relative to the tip of the stack

    print_str
    #str "Print \0"
    print.u ^.result
    print_nl

    calc.u add (^.result, ^.b) -> ^.result ; add one to the result

    return ^.result ; return, restoring the frame address, then push ^.result onto the stack
