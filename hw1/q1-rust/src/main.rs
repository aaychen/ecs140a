/// Takes 2 characters as input (a close parenthesis and an open parenthesis) and checks to see if
/// they are of the same parenthesis type. If they are, this function returns True; otherwise,
/// returns False
fn is_matching_type(open_paren: char, close_paren: char) -> bool {
    (open_paren == '(' && close_paren == ')') ||
    (open_paren == '{' && close_paren == '}') ||
    (open_paren == '[' && close_paren == ']')
}

/// Takes a string as input and checks to see if all parentheses in it match. If all parentheses 
/// match, this function returns True; otherwise, returns False.
fn matching_parentheses(s: String) -> bool {
    let mut stack = Vec::new(); // using a vector for stack
    for ch in s.chars() {
        if "({[".contains(ch) { // open parenthesis case
            stack.push(ch);
        } else if ")}]".contains(ch) { // close parenthesis case
            if stack.len() == 0 || !is_matching_type(stack[stack.len()-1], ch) {
                return false;
            }
            else { // if ch and last item pushed to stack match parenthesis type
                stack.pop();
            }
        }
    }
    return if stack.len() == 0 { true } else { false };
}

fn main() {
    let test_cases = vec![
        "()", "[a(b)]", "[a(b])", "a{abc([])", "aabc([])}c", 
        "", "abc", ")", "(", "(]", "({[()]})", "({[()]}", "{[()]})"
    ];
    for tc in test_cases {
        println!("{}: {}", tc, matching_parentheses(tc.to_string()));
    }
}
