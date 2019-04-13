current = R0
test = R1

ldc ~current, 1
next_prime:
    calc.u ~current, add ( ~current, 1 )
    ldc ~test, 1
test_loop:
    calc.u ~test, add ( ~test, 1 )
    jmpif.u success, ( ~current, eq, ~test ) ; jumpif direct comparisons coming soon
    jmpif.u next_prime, eqz calc mod ( ~current, ~test )
    jmp test_loop
success:
    debug ~current
    jmp next_prime
halt
