main:
    push '0
    push '0

    loop:
        .latest = 0
        .index = 4
        call 'get_fib -> ^.latest
        print.u ^.latest
        print_str
        #str " \0"
        calc.u add (^.index, '1) -> ^.index
        jump 'loop
    halt

get_fib:
    .i = param_start-4

    jumpif.u ( ^.i, eq, '0 ) -> '.terminal
    jumpif.u ( ^.i, eq, '1 ) -> '.terminal
    jump '.recurse
    .terminal:
        return '1
    .recurse:
        .minus_one = 0
        .minus_two = 4
        push '0
        push '0

        calc.u sub (^.i, '1) -> ^.minus_one
        calc.u sub (^.i, '2) -> ^.minus_two

        push ^.minus_one
        call 'get_fib -> ^.minus_one
        drop '4

        push ^.minus_two
        call 'get_fib -> ^.minus_two
        drop '4

        .sum = 8
        push '0
        calc.u add (^.minus_one, ^.minus_two) -> ^.sum
        return ^.sum
