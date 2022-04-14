#include <iostream>
#include <string>
#include <vector>

/*
Takes an array of strings as input and finds the longest common prefix of all of the strings
*/
std::string find_lcp(std::vector<std::string>& arr) {
    if (arr.size() == 0) 
        return "";
    std::string lcp = arr[0]; // treat first item as current longest common prefix
    for (std::string s : arr) {
        std::string temp = ""; // to keep track of common prefix in current 2-string comparison
        for (size_t i = 0; i < s.length(); i++) {
            if (i >= lcp.length() || s[i] != lcp[i]) // if index out of range or characters not the same
                break;
            else
                temp += s[i];
        }
        if (temp.length() < lcp.length()) // update lcp if new word compared results in a shorter one
            lcp = temp;
    }
    return lcp;
}

int main() {
    std::vector<std::string> test_cases[] = {
        {"apple", "app", "aple", "appl"},
        {},
        {""},
        {"abc"},
        {"abc", "xyz"},
        {"zzzzz", "zz", "zzzz"},
        {"bamboo", "bamboozled"},
        {"bamboo", "bamboozled", "bambam"}
    };
    for (std::vector<std::string> tc : test_cases) {
        std::cout << "[";
        for (size_t i = 0; i < tc.size(); i++) {
            std::cout << tc[i];
            if (i == tc.size()-1) break;
            std::cout << ", ";
        }
        std::cout << "]: " << find_lcp(tc) << std::endl;
    }
    return 0;
}