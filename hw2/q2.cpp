#include <iostream>
#include <string>
#include <fstream>
#include <sstream>

class CStream {
    public:
        std::string fname;
        int line_num;
        int char_pos;
        std::string content;
        int content_pos;

        // Constructor
        CStream(std::string file_name) {
            fname = file_name;
            line_num = -1;
            char_pos = -1;

            // Source: https://stackoverflow.com/questions/2602013/read-whole-ascii-file-into-c-stdstring
            std::ifstream t(fname);
            std::stringstream buffer;
            buffer << t.rdbuf();
            content = buffer.str();
            content_pos = -1;
        }

        // Return true if have more contents to read in file
        bool more_available() {
            // if current position not last character
            if (content_pos == content.length() - 1)
                return false;
            return true;
        }

        // Return character at current position in file
        char get_cur_char() {
            return content[content_pos];
        }

        // Get next character in file and move position
        char get_next_char() {
            if (line_num == -1) { // check initial value of line_num
                line_num++;
            }
            char_pos++;
            content_pos++;
            char ch = ' ';
            while (1) {
                ch = content[content_pos];
                if (ch == '\n') {
                    char_pos = 0; // reset char position to 0 when on new line
                    line_num++;
                    content_pos++;
                } else {
                    return ch;
                }
            }
        }

        // Get next character in file without changing position in file
        char peek_next_char() {
            // peek_ahead_char(0) is same as peek_next_char()
            return peek_ahead_char(0);
        }

        // Get kth character ahead in file without changing position
        char peek_ahead_char(int k) {
            int count = 0; // keep track of non-newline chars
            int i = 1; // keep track of newline chars; default is 1 to make k=0 next char
            char ch = ' ';
            while (count <= k) {
                ch = content[content_pos + count + i]; // check character ahead
                if (ch == '\n') {
                    i++;
                } else {
                    count++;
                }
            }
            return ch;
        }
};

int main() {
    std::cout << std::boolalpha;

    CStream f = CStream("./q2_test1.txt");
    std::cout << "TEST 1" << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_ahead_char(4) << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_cur_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.more_available() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;

    f = CStream("./q2_test2.txt");
    std::cout << "\nTEST 2" << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_ahead_char(7) << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_ahead_char(6) << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_cur_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.more_available() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;

    f = CStream("./q2_test3.txt");
    std::cout << "\nTEST 3" << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.more_available() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.more_available() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;

    f = CStream("./q2_test4.txt");
    std::cout << "\nTEST 4" << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_ahead_char(0) << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_ahead_char(4) << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.get_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_next_char() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.peek_ahead_char(3) << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;
    std::cout << f.more_available() << std::endl;
    std::cout << "line_num = " << f.line_num << ", char_pos = " << f.char_pos << std::endl;

    return 0;
}