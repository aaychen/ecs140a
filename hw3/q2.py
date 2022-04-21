class SimpleParser:
    def __init__(self, str_in):
        self.string = str_in
        self.char_pos = -1

    def fun_s(self):
        ch = self.get_next_char()
        if ch == 'a' and ch == self.peek_next_char():
            self.fun_s()
        elif self.fun_x():
            print("Input is valid")
        else:
            print(f"Syntax error at character position {self.char_pos}")
    
    def fun_x(self):
        ch = self.get_next_char()
        if ch == 'c' or ch == 'd':
            try:
                self.get_next_char()
            except IndexError:
                return True
        return False

    def get_next_char(self):
        self.char_pos += 1
        return self.string[self.char_pos]

    def peek_next_char(self):
        return self.string[self.char_pos + 1]

def main():
    tests = ["bc", "acd", "aaad", "c", "2yz", ""]
    for i, t in enumerate(tests):
        sp = SimpleParser(t)
        print(f'Test {i+1}: "{t}" ')
        sp.fun_s()
        print()

if __name__ == '__main__':
    main()