use std::fs;

struct CStream {
    fname: String,
    line_num: i32,
    char_pos: i32,
    content: String,
    content_pos: i32
}

impl CStream {
    // Initializer
    // Source for reading file into a string:
    // https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
    fn new(file_name: String) -> CStream {
        return CStream {
            fname: file_name.clone(),
            line_num: -1,
            char_pos: -1,
            content: fs::read_to_string(file_name.clone()).expect("Unable to read file"),
            content_pos: -1
        };
    }

    // Return true if have more contents to read in file
    fn more_available(&self) -> bool {
        // if current position not last character
        if self.content_pos == (self.content.len() - 1) as i32 {
            return false;
        }
        return true;
    }

    // Return character at current position in file
    fn get_cur_char(&self) -> char {
        return self.content.chars().nth(self.content_pos as usize).unwrap();
    }
    
    // Get next character in file and move position
    fn get_next_char(&mut self) -> char {
        if self.line_num == -1 { // check initial value of line_num
            self.line_num += 1;
        }
        self.char_pos += 1;
        self.content_pos += 1;
        let mut ch = ' ';
        loop { // warning message in compiler suggested using loop for infinite loop
            ch = self.content.chars().nth(self.content_pos as usize).unwrap();
            if ch == '\n' {
                self.char_pos = 0; // reset char position to 0 when on new line
                self.line_num += 1;
                self.content_pos += 1
            } else {
                return ch;
            }
        }
        return ch;
    }

    // Get next character in file without changing position in file
    fn peek_next_char(&mut self) -> char {
        // peek_ahead_char(0) same as peek_next_char()
        return self.peek_ahead_char(0);
    }

    // Get kth character ahead in file without changing position
    fn peek_ahead_char(&mut self, k: i32) -> char {
        let mut count = 0; // track non-newline chars
        let mut i = 1; // track newline chars
        let mut ch = ' ';
        while count <= k {
            // check character ahead
            ch = self.content.chars().nth((self.content_pos + count + i) as usize).unwrap();
            if ch == '\n' {
                i += 1;
            } else {
                count += 1;
            }
        }
        return ch;
    }
}

fn main() {
    let mut f = CStream::new("/Users/annachen/ecs140a/hw2/q2_test1.txt".to_string());
    println!("TEST 1");
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_ahead_char(4));
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_cur_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.more_available());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);

    let mut f = CStream::new("/Users/annachen/ecs140a/hw2/q2_test2.txt".to_string());
    println!("\nTEST 2");
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_ahead_char(7));
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_ahead_char(6));
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_cur_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.more_available());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);

    let mut f = CStream::new("/Users/annachen/ecs140a/hw2/q2_test3.txt".to_string());
    println!("\nTEST 3");
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.more_available());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.more_available());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);

    let mut f = CStream::new("/Users/annachen/ecs140a/hw2/q2_test4.txt".to_string());
    println!("\nTEST 4");
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_ahead_char(0));
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_ahead_char(4));
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.get_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_next_char());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.peek_ahead_char(3));
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
    println!("{}", f.more_available());
    println!("line_num = {}, char_pos = {}", f.line_num, f.char_pos);
}
