current = 0
half_current = 1
testi = 2
endi = 3

move32 '2 -> >0
move8 '4 -> %endi
move8 '2 -> %current

next_prime:
    calc.u add (%current, '1) -> %current
    calc.u add (%current, '1) -> %half_current
    calc.u div (%half_current, '2) -> %half_current

    move8 '0 -> %testi

    print_str
    #str "Testing \0"
    print.u %current
    print_str
    #str ": \0"

test_loop:
    ;jumpif.u eqz ( >%testi ) -> 'success

    jumpif.u eqz ( %testi ) -> '.after_comma
    print_str
    #str ", \0"
    .after_comma:
    print.u >%testi

    jumpif.u eqz calc mod (%current, >%testi) -> 'failure
    jumpif.u ( >%testi, geq, %half_current ) -> 'success
    calc.u add (%testi, '4) -> %testi
    jump 'test_loop

failure:
    print_nl
    jump 'next_prime

success:
    print_str
    #str " - prime\0"
    print_nl

    move32 %current -> >%endi
    calc.u add (%endi, '4) -> %endi
    jump 'next_prime
halt