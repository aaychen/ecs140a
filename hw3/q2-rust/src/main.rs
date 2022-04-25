// Sources:
// https://crates.io/crates/custom_error
// https://docs.rs/custom_error/latest/custom_error/macro.custom_error.html
// https://doc.rust-lang.org/std/result/

extern crate custom_error;
use custom_error::custom_error;

custom_error! { ParseError 
    Syntax{pos: i32} = "Syntax error at character position {pos}"
}

struct SimpleParser {
    str_in: String,
    char_pos: i32,
}

// Assuming string input has no whitespace
// No null value in Rust; so using ' ' if nothing left in string to parse
impl SimpleParser {
    // Initializer
    fn new(s: String) -> SimpleParser {
        return SimpleParser {
            str_in: s, // string input
            char_pos: -1 // current position in string input
        };
    }

    // Handles grammar rule for S
    // Prints message according to if grammar rule is satistfied
    fn fun_s(&mut self) {
        let ch: char;
        // don't move position if string[0] is not 'a' or 'b' and let fun_x() handle
        if self.char_pos == -1 && self.peek_next_char() != 'a' 
            && self.peek_next_char() != 'b' {
            ch = self.peek_next_char();
        } else { // if char is 'a' or 'b'
            ch = self.get_next_char();
        }

        if ch == 'a' && ch == self.peek_next_char() { // handle repeated 'a' chars
            self.fun_s();
        } else {
            match self.fun_x() {
                Ok(()) => println!("Input is valid"),
                Err(e) => println!("{}", e.to_string())
            }
        }
    }

    // Handles grammar rule for X
    // Returns an error if grammar rule not satisfied
    fn fun_x(&mut self) -> Result<(), ParseError> {
        let ch = self.get_next_char();
        if (ch == 'c' || ch == 'd') && self.get_next_char() == ' ' {
            return Ok(()); // return success
        }
        return Err(ParseError::Syntax{pos: self.char_pos}); // return fail
    }

    // Move forward 1 position and return char at new position
    // Return ' ' if out of bounds
    fn get_next_char(&mut self) -> char {
        self.char_pos += 1;
        if self.char_pos < self.str_in.len() as i32 {
            return self.str_in.chars().nth(self.char_pos as usize).unwrap();
        }
        return ' ';
    }

    // Returns char 1 position ahead of current
    // Returns ' ' if out of bounds
    fn peek_next_char(&self) -> char {
        if self.char_pos + 1 < self.str_in.len() as i32 {
            return self.str_in.chars().nth((self.char_pos + 1) as usize).unwrap();
        }
        return ' ';
    }
}

fn main() {
    let tests = vec![
        // given
        "bc", // valid
        "acd", // invalid at 2
        "aaad", // valid
        "c", // valid
        "2yz", // invalid at 0
        "", // invalid at 0
        
        // additional
        "aaaaac", // valid
        "bd", // valid
        "aaaaa", // invalid at 5
        "b", // invalid at 1
        "bbbc", // invalid at 1
        "cccc", // invalid at 1
        "aaazzz", // invalid at 3
        "ybb", // invalid at 0

        // additional on top of python
        "ac", // valid
        "aaaaad", // valid
        "cbb", // invalid at 1
        "abd", // invalid at 1
        "abc", // invalid at 1
        "aaaaaacd", // invalid at 7
    ];
    let mut test_num: i32 = 1;
    for t in tests {
        let mut sp = SimpleParser::new(t.to_string());
        println!("Test {}: {}", test_num, t);
        sp.fun_s();
        println!("");
        test_num += 1;
    }
}
