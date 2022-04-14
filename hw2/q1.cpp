#include <iostream>
#include <vector>

class Stack {
    public:
        std::vector<int> nums; // empty vector
        
        /* Push item onto stack */
        void push(int val) {
            nums.push_back(val);
        }

        /**
         * Remove most recently added item from stack and return it. 
         * Assumes stack is non-empty. 
         */
        int pop() {
            int last_val = nums.back();
            nums.pop_back();
            return last_val;
        }

        /** 
         * Returns most recently added item from stack without removing it. 
         * Assumes stack is non-empty.
         */
        int peek() {
            return nums.back();
        }
};

int main() {
    std::cout << "test 1" << std::endl;
    Stack st;
    st.push(3);
    std::cout << st.peek() << std::endl;
    st.push(4);
    st.push(5);
    std::cout << st.pop() << std::endl;
    std::cout << st.peek() << std::endl;
    st.push(6);
    std::cout << st.pop() << std::endl;

    std::cout << "\ntest 2" << std::endl;
    Stack st2;
    st2.push(1);
    st2.push(2);
    st2.push(3);
    std::cout << st2.pop() << std::endl;
    std::cout << st2.pop() << std::endl;
    std::cout << st2.pop() << std::endl;

    std::cout << "\ntest 3" << std::endl;
    Stack st3;
    st3.push(1);
    st3.push(2);
    std::cout << st3.peek() << std::endl;
    std::cout << st3.peek() << std::endl;

    std::cout << "\ntest 4" << std::endl;
    Stack st4;
    st4.push(1);
    std::cout << st4.peek() << std::endl;
    std::cout << st4.pop() << std::endl;
    st4.push(2);
    std::cout << st4.peek() << std::endl;
    st4.push(5);
    std::cout << st4.peek() << std::endl;
    st4.push(10);
    std::cout << st4.pop() << std::endl;
    std::cout << st4.pop() << std::endl;
    std::cout << st4.pop() << std::endl;

    return 0;
}