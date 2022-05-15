-- (40 points) Solve Problem 2 from Homework 01. Specifically, given a list of strings, write a function
-- lcp that returns the longest common prefix of the strings.
-- Examples:
-- • lcp["apple", "app", "aple", "appl"] should return "ap"

-- cmpStr (string, string) -> string
--     find lcp of the two strings passed
--     check if char from both strings are equal
--         if equal, add onto resulting prefix
--         if not, current prefix is longest between the two strings

-- lcp (string array) -> string
--     use fold function to apply cmpStrings to and reduce string array to lcp

-- actual code
-- 1
-- cmpStr :: String -> String -> String
-- cmpStr s1 s2 = do
--     (x:xs) = s1
--     (y:ys) = s2
--     if x == y
--         then x ++ cmpStr xs ys
--         else ''

-- 2
-- cmpStr :: String -> String -> String
-- cmpStr s1 s2 = do
--     let (x:xs) = s1
--     let (y:ys) = s2
--     if x == y
--         then x ++ cmpStr xs ys
--         else ''

-- 3
-- cmpStr :: String -> String -> String
-- cmpStr s1 s2 = do
--     let (x:xs) = s1
--     let (y:ys) = s2
--     if x == y
--         then x ++ cmpStr xs ys
--         else []

-- 4 (working code)
-- cmpStr :: String -> String -> String
-- cmpStr s1 s2 = do
--     let (x:xs) = s1
--     let (y:ys) = s2
--     if x == y
--         then [x] ++ cmpStr xs ys
--         else []

-- fix bug where ["a", "abb", "ab"] gives ""
-- works for given test case
cmpStr :: String -> String -> String
cmpStr s1 s2 = do
    if s1 == [] || s2 == []
        then []
    else do
        let (x:xs) = s1
        let (y:ys) = s2
        if x == y
            then [x] ++ cmpStr xs ys
        else []
    

lcp :: [String] -> String
lcp arr = do
    let dummy = head arr
    foldr cmpStr dummy arr

main = do
    print (lcp ["a", "abb", "ab"])
    print (lcp ["apple", "app", "aple", "appl"])
