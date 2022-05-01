# Sources:
# https://docs.python.org/3/library/enum.html
# https://stackoverflow.com/questions/24487405/getting-value-of-enum-on-string-conversion

from enum import Enum

class TokenType(Enum):
    ''' Enum types given string values for easy token type printing '''
    CONSTANT = "constant"
    OPERATOR = "operator"
    VARIABLE = "variable"
    SPECIAL = "special symbol"

class Token:
    def __init__(self, tok_text, tok_type):
        ''' Initializer '''
        self.text = tok_text 
        self.token_type = tok_type

def get_tokens(text):
    ''' 
    Assumes program is syntactically correct (no non-sensical tokens)
    Creates tokens (constant, variable, special, or operator) while parsing text 
    and returns an array of the tokens created
    '''
    text = text.replace(" ", "") # replace all spaces in text
    toks = []
    i = 0
    while i < len(text): # parse char by char
        if text[i] == '0' or text[i] == '1':
            toks.append(Token(text[i], TokenType.CONSTANT))
        elif 'a' <= text[i] <= 'd':
            toks.append(Token(text[i], TokenType.VARIABLE))
        elif text[i] == ';':
            toks.append(Token(";", TokenType.SPECIAL))
        elif text[i] == ':' and text[i+1] == '=':
            toks.append(Token(":=", TokenType.SPECIAL))
            i += 1 # skip '=' to not double count it
        else:
            if text[i] in "=<>!" and text[i+1] == '=':
                toks.append(Token(text[i:i+2], TokenType.OPERATOR))
                i += 1 # skip '=' to not double count it
            else:
                toks.append(Token(text[i], TokenType.OPERATOR))
        i += 1
    return toks

def print_tokens(toks):
    ''' Prints token text and token type for every token in token array '''
    for i, tok in enumerate(toks):
        print(f"Token {i} = {tok.text}")
        print(f"Token type: {tok.token_type.value}")
        print()

def main():
    tests = [
        "a := 0 + 1;",
        "b:=1;",
        "b          :=    1;",
        "d := 0 / 1;",
        "d := a % c;",
        "c := 1 > b;",
        "d := 0 <= a;",
        "c := (1 * d) != (0 / c);"
    ]
    for i, test in enumerate(tests):
        print(f"TEST {i+1}")
        print_tokens(get_tokens(test))
        print("==============================")
        print()
    

if __name__ == "__main__":
    main()