class SimpleParser:
    initializer(s):
        # store input string and call it str_in
        # store position of current character and call it char_pos

    fun_s():
        # code for grammar rule of S
        # call get_next_char() and compare to grammar rule of S
        # if don't see 'a' or 'b', call fun_x()
            # if fun_x() returns error, print error
        # print "Input is valid" if nothing goes wrong
    
    fun_x():
        # code for the grammar rule of X
        # call get_next_char() and compare to grammar rule of X
        # if char is not 'c' or 'd', return error

    get_next_char():
        # increment char_pos and return character at char_pos in str_in

main():
    test = "bc"
    sp = new SimpleParser(test) # create an object of SimpleParser with the test string
    sp.fun_s() # should print "Input is valid"