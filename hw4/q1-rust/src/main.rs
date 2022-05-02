// Sources:
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust

enum TokenType {
    CONSTANT,
    OPERATOR,
    VARIABLE,
    SPECIAL
}

impl TokenType {
    // For ease of printing token type, assign string values to the enums
    fn value(&self) -> String {
        match *self {
            TokenType::CONSTANT => "constant".to_string(),
            TokenType::OPERATOR => "operator".to_string(),
            TokenType::VARIABLE => "variable".to_string(),
            TokenType::SPECIAL => "special symbol".to_string()
        }
    }
}

struct Token {
    text: String,
    token_type: TokenType,
}

impl Token {
    // Initializer
    fn new(tok_text: String, tok_type: TokenType) -> Token {
        return Token {
            text: tok_text,
            token_type: tok_type
        };
    }
}

// Assumes program is syntactically correct (no non-sensical tokens)
// Creates tokens (constant, variable, special, or operator) while parsing text
// and returns a vector of the tokens created
fn get_tokens(mut text: String) -> Vec<Token> {
    let mut ch: char;
    let mut toks = Vec::new();
    let mut i = 0;
    text = text.replace(" ", ""); // replace all spaces in text
    while i < text.len() { // parse text char by char
        ch = text.chars().nth(i as usize).unwrap();
        if ch == '0' || ch  == '1' {
            toks.push(Token::new(ch.to_string(), TokenType::CONSTANT));
        } else if 'a' <= ch && ch <= 'd' {
            toks.push(Token::new(ch.to_string(), TokenType::VARIABLE));
        } else if ch == ';' {
            toks.push(Token::new(ch.to_string(), TokenType::SPECIAL));
        } else if ch == ':' && text.chars().nth(i+1 as usize).unwrap() == '=' {
            toks.push(Token::new(":=".to_string(), TokenType::SPECIAL));
            i += 1; // skip '=' to not double count it
        } else {
            if "=<>!".contains(ch) && text.chars().nth(i+1 as usize).unwrap() == '=' {
                toks.push(Token::new(text[i..i+2].to_string(), TokenType::OPERATOR));
                i += 1; // skip '=' to not double count it
            } else {
                toks.push(Token::new(ch.to_string(), TokenType::OPERATOR));
            }
        }
        i += 1;
    }
    return toks;
}

// Prints token text and token type for every token in token vector 
fn print_tokens(tokens: Vec<Token>) {
    for (i, tok) in tokens.iter().enumerate() {
        println!("Token {} = {}", i, tok.text);
        println!("Token type: {}", tok.token_type.value());
        println!();
    }
}

fn main() {
    let tests = vec![
        "a := 0 + 1;",
        "b:=1;",
        "b          :=    1;",
        "d := 0 / 1;",
        "d := a % c;",
        "c := 1 > b;",
        "d := 0 <= a;",
        "c := (1 * d) != (0 / c);"
    ];
    let mut test_num: i32 = 1;
    for t in tests {
        println!("TEST {}", test_num);
        print_tokens(get_tokens(t.to_string()));
        println!("==============================");
        println!();
        test_num += 1;
    }
}
