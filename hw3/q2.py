class SimpleParser:
    def __init__(self, str_in):
        '''' Initializer '''
        self.string = str_in # string to parse
        self.char_pos = -1 # current char position in string

    def fun_s(self):
        ''' Handles grammar rule for S. Prints message according to if grammar rule is satisfied '''
        if self.peek_next_char() == None: # handle if no characters left to read
            ch = None
        elif self.char_pos == -1 and self.peek_next_char() not in "ab": # don't move position if string[0] is not 'a' or 'b' and let fun_x() handle
            ch = self.peek_next_char()
        else: # if char is 'a' or 'b'
            ch = self.get_next_char()

        if ch == 'a' and ch == self.peek_next_char(): # handle repeated 'a' chars
            self.fun_s()
        elif self.fun_x():
            print("Input is valid")
        else:
            print(f"Syntax error at character position {self.char_pos}")
    
    def fun_x(self):
        ''' Handles grammar rule for X. Return True if grammar rule is satisfied; else False '''
        ch = self.get_next_char()
        if (ch == 'c' or ch == 'd') and self.get_next_char() == None:
            return True
        return False

    def get_next_char(self):
        ''' Moves forward 1 position and returns character at new position. Returns None if out of bounds '''
        try:
            self.char_pos += 1
            return self.string[self.char_pos]
        except IndexError:
            return None

    def peek_next_char(self):
        ''' Returns character 1 position ahead. Returns None if out of bounds '''
        try:
            return self.string[self.char_pos + 1]
        except IndexError:
            return None

def main():
    # tests = ["bc", "acd", "aaad", "c", "2yz", ""]
    tests = [
        # given
        "bc", # valid
        "acd", # invalid at 2
        "aaad", # valid
        "c", # valid
        "2yz", # invalid at 0
        "", # invalid at 0
        
        # additional
        # "ac", # valid
        # "ad", # valid
        "aaaaac", # valid
        # "aaaaad", # valid
        "bd", # valid
        "aaaaa", # invalid at 5
        "b", # invalid at 1
        "bbbc", # invalid at 1
        "cccc", # invalid at 1
        "aaazzz", # invalid at 3
        # "cbb", # invalid at 1
        "ybb" # invalid at 0
    ]
    for i, t in enumerate(tests):
        sp = SimpleParser(t)
        print(f'Test {i+1}: "{t}" ')
        sp.fun_s()
        print()

if __name__ == '__main__':
    main()