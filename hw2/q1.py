class Stack:
    def __init__(self):
        self.nums = [] # stack initialized as empty

    def push(self, val):
        ''' Adds an item onto the stack '''
        self.nums.append(val)

    def pop(self):
        ''' Removes most recently added item from stack and returns it. Assumes stack is not empty. '''
        return self.nums.pop()

    def peek(self):
        ''' Returns most recently added item from stack without removing it. Assumes stack is not empty. '''
        return self.nums[-1]

if __name__ == '__main__':
    print("test 1")
    st = Stack()
    st.push(3)
    print(st.peek())
    st.push(4)
    st.push(5)
    print(st.pop())
    print(st.peek())
    st.push(6)
    print(st.pop())

    print("\ntest 2")
    st2 = Stack()
    st2.push(1)
    st2.push(2)
    st2.push(3)
    print(st2.pop())
    print(st2.pop())
    print(st2.pop())

    print("\ntest 3")
    st3 = Stack()
    st3.push(1)
    st3.push(2)
    print(st3.peek())
    print(st3.peek())

    print("\ntest 4")
    st4 = Stack()
    st4.push(1)
    print(st4.peek())
    print(st4.pop())
    st4.push(2)
    print(st4.peek())
    st4.push(5)
    print(st4.peek())
    st4.push(10)
    print(st4.pop())
    print(st4.pop())
    print(st4.pop())