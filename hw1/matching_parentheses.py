def is_matching_type(open_paren, close_paren):
    '''
    Takes 2 characters as input (a close parenthesis and an open parenthesis) and checks to see if
    they are of the same parenthesis type. If they are, this function returns True; otherwise,
    returns False
    '''
    return (open_paren == '(' and close_paren == ')') \
        or (open_paren == '{' and close_paren == '}') \
        or (open_paren == '[' and close_paren == ']')

def matching_parentheses(string):
    '''
    Takes a string as input and checks to see if all parentheses in it match. If all parentheses 
    match, this function returns True; otherwise, returns False.
    '''
    stack = []
    for ch in string:
        if ch in "({[": # open parenthesis case
            stack.append(ch)
        elif ch in ")}]": # close parenthesis case
            if len(stack) == 0 or not is_matching_type(stack[-1], ch):
                return False
            else: # if ch and stack[-1] match parenthesis type
                stack.pop()
    return True if len(stack) == 0 else False

if __name__ == "__main__": # main module check
    test_cases = [
        "()", "[a(b)]", "[a(b])", "a{abc([])", "aabc([])}c", 
        "", "abc", ")", "(", "(]", "({[()]})", "({[()]}", "{[()]})"
    ]
    for string in test_cases:
        print(f"{string}: {matching_parentheses(string)}")
