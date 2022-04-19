class CStream:
    def __init__(self, fname):
        ''' Initializer '''
        self.fname = fname
        self.line_num = -1
        self.char_pos = -1
        self.content = ""
        with open(fname, 'r') as fp:
            self.content = fp.read() # read entire file into string
        self.content_pos = -1 # current index in string

    def more_available(self):
        ''' Return true if have more contents to read in file '''
        if self.content_pos == len(self.content) - 1: # if current position not last character
            return False
        return True

    def get_cur_char(self):
        ''' Return character at current position in file '''
        return self.content[self.content_pos] # return character at current position
    
    def get_next_char(self):
        ''' Get next character in file and move position '''
        if self.line_num == -1: # check initial value of line_num
            self.line_num += 1
        self.char_pos += 1
        self.content_pos += 1
        while True:
            ch = self.content[self.content_pos]
            if ch == '\n':
                self.char_pos = 0 # reset char position to 0 when on new line
                self.line_num += 1
                self.content_pos += 1
            else:
                return ch

    def peek_next_char(self):
        ''' Get next character in file without changing position in file '''
        return self.peek_ahead_char(0) # peek_ahead_char(0) is same as peek_next_char()

    def peek_ahead_char(self, k):
        ''' Get kth character ahead in file without changing position '''
        count = 0 # keep track of non-newline char
        i = 1 # keep track of newline char; default is 1 to make k=0 next char
        ch = ''
        while count <= k:
            ch = self.content[self.content_pos + count + i] # check character ahead
            if ch == '\n':
                i += 1
            else:
                count += 1
        return ch


if __name__ == '__main__':
    print("TEST 1")
    f = CStream('./q2_test1.txt')
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_ahead_char(4))
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_cur_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.more_available())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")

    print("\nTEST 2")
    f = CStream('./q2_test2.txt')
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_ahead_char(7))
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_ahead_char(6))
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_cur_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.more_available())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")

    print("\nTEST 3")
    f = CStream('./q2_test3.txt')
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.more_available())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.more_available())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")

    print("\nTEST 4")
    f = CStream('./q2_test4.txt')
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_ahead_char(0))
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_ahead_char(4))
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.get_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_next_char())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.peek_ahead_char(3))
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
    print(f.more_available())
    print(f"line_num = {f.line_num}, char_pos = {f.char_pos}")
