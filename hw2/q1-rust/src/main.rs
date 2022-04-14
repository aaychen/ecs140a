struct Stack {
    stack: Vec<i32>
}

impl Stack {
    // initializer
    fn new() -> Stack {
        Stack {
            stack: Vec::new()
        }
    }

    // Adds an integer onto the stack
    fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    // Removes the most recently added item from stack and returns it
    // Assumes stack is non-empty
    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    // Returns most recently added item to stack without removing it
    // Assumes stack is non-empty
    fn peek(&self) -> i32 {
        self.stack[self.stack.len()-1]
    }
}

fn main() {
    println!("test 1");
    let mut st = Stack::new();
    st.push(3);
    println!("{}", st.peek());
    st.push(4);
    st.push(5);
    println!("{}", st.pop().unwrap());
    println!("{}", st.peek());
    st.push(6);
    println!("{}", st.pop().unwrap());

    println!("\ntest 2");
    let mut st2 = Stack::new();
    st2.push(1);
    st2.push(2);
    st2.push(3);
    println!("{}", st2.pop().unwrap());
    println!("{}", st2.pop().unwrap());
    println!("{}", st2.pop().unwrap());

    println!("\ntest 3");
    let mut st3 = Stack::new();
    st3.push(1);
    st3.push(2);
    println!("{}", st3.peek());
    println!("{}", st3.peek());

    println!("\ntest 4");
    let mut st4 = Stack::new();
    st4.push(1);
    println!("{}", st4.peek());
    println!("{}", st4.pop().unwrap());
    st4.push(2);
    println!("{}", st4.peek());
    st4.push(5);
    println!("{}", st4.peek());
    st4.push(10);
    println!("{}", st4.pop().unwrap());
    println!("{}", st4.pop().unwrap());
    println!("{}", st4.pop().unwrap());
}
