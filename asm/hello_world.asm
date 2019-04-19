move8 >+0x00 <- '100
move16 >0x02 <- '200
move32 >0x08 <- '300

store.bytes >0x10 <- @d1, d1_len
store.bytes >0x30 <- @s_end
    #str "Such inline, many cool\0"
s_end:

store.string >0x40 <- 
    #str "Such inline, many cool\0"
debug
halt

d1:
    #str "Wow much cool\0"
d1_len = pc - d1


current = R0
test = R1

load %current <- 1
next_prime:
    calc.u %current <- add ( %current, 1 )
    load %test <- 1
test_loop:
    calc.u %test <- add ( %test, 1 )
    jmpif.u @success, ( %current, eq, %test ) ; jumpif direct comparisons coming soon
    jmpif.u @next_prime, eqz calc mod ( %current, %test )
    jmp @test_loop
success:
    debug %current
    jmp @next_prime
halt
