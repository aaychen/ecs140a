def find_lcp(arr):
    '''
    Takes an array of strings as input and finds the longest common prefix of all of the strings
    '''
    if len(arr) == 0:
        return ""
    lcp = arr[0] # treat first item as current longest common prefix
    for s in arr[1:]:
        temp = "" # to keep track of common prefix in current 2-string comparison
        for i in range(0, len(s)):
            if i >= len(lcp) or s[i] != lcp[i]: # if index out of range or characters not the same
                break
            else:
                temp += s[i]
        if len(temp) < len(lcp): # update lcp if new word compared results in a shorter one
            lcp = temp
    return lcp

if __name__ == '__main__': # main module check
    test_cases = [
        ["apple", "app", "aple", "appl"],
        [],
        [""],
        ["abc"],
        ["abc", "xyz"],
        ["zzzzz", "zz", "zzzz"],
        ["bamboo", "bamboozled"],
        ["bamboo", "bamboozled", "bambam"]
    ]
    for tc in test_cases:
        print(f"{tc}: {find_lcp(tc)}")