current = 0
testi = 1

move32 '2 -> >0
move8 '1 -> %current

next_prime:
    calc.u add (%current, '1) -> %current
    move8 '0 -> %testi

    print_str
    #str "Testing \0"
    print.u %current
    print_str
    #str ": \0"

test_loop:
    jumpif.u eqz ( >%testi ) -> 'success

    jumpif.u eqz ( %testi ) -> '.after_comma
    print_str
    #str ", \0"
    .after_comma:
    print.u >%testi

    jumpif.u eqz calc mod (%current, >%testi) -> 'failure
    calc.u add (%testi, '4) -> %testi
    jump 'test_loop

failure:
    print_nl
    jump 'next_prime

success:
    print_str
    #str " - prime\0"
    print_nl

    move32 %current -> >%testi
    jump 'next_prime
halt