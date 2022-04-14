#include <iostream>
#include <string>
#include <vector>

/*  
Takes 2 characters as input (a close parenthesis and an open parenthesis) and checks to see if
they are of the same parenthesis type. If they are, this function returns True; otherwise,
returns False
*/
bool is_matching_type(char open_paren, char close_paren) {
    return (open_paren == '(' && close_paren == ')') 
        || (open_paren == '{' && close_paren == '}') 
        || (open_paren == '[' && close_paren == ']');
}

/*
Takes a string as input and checks to see if all parentheses in it match. If all parentheses 
match, this function returns True; otherwise, returns False.
*/
bool matching_parentheses(std::string str) {
    std::vector<char> stack;
    for (char ch : str) {
        if (ch == '(' || ch == '{' || ch == '[') { // open parenthesis case
            stack.push_back(ch);
        } else if (ch == ')' || ch == '}' || ch == ']') { // close parenthesis case
            if (stack.size() == 0 or !is_matching_type(stack.back(), ch))
                return false;
            else // if ch and last item pushed in stack are matching parenthesis type
                stack.pop_back();
        }
    }
    return stack.size() == 0 ? true : false;
}

int main() {
    std::string test_cases[] = {
        "()", "[a(b)]", "[a(b])", "a{abc([])", "aabc([])}c", 
        "", "abc", ")", "(", "(]", "({[()]})", "({[()]}", "{[()]})"
    };
    std::cout << std::boolalpha;
    for (std::string s : test_cases) {
        std::cout << s << ": " << matching_parentheses(s) << std::endl;
    }
    return 0;
}
