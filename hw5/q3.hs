-- Check if open parenthesis character matches close parenthesis character
isMatch :: Char -> Char -> Bool
isMatch open close =
    (open == '(' && close == ')') ||
    (open == '{' && close == '}') ||
    (open == '[' && close == ']')

-- Helper function that recurses on str[1:] and updates stack of open parentheses as needed
matchingHelper :: String -> [Char] -> Bool
matchingHelper str stack = do
    -- empty str check
    if str == [] && stack == [] then True
    else if str == [] && stack /= [] then False 
    else do
        let (x:xs) = str
        if x == '(' || x == '{' || x == '[' -- open parenthesis char check
            then matchingHelper xs (stack ++ [x]) -- append to stack and recurse
        else if x == ')' || x == '}' || x == ']' then do -- close parenthesis char check
            if stack == [] then False -- extra closing
            else if isMatch (last stack) x
                then matchingHelper xs (take (length stack - 1) stack) -- pop stack and recurse
            else False
        else matchingHelper xs stack -- if other chars, "do nothing"

-- Uses helper function in order to include stack
matching :: String -> Bool
matching str = matchingHelper str []   

main = do
    -- given cases
    print (matching "()") -- true
    print (matching "[a(b])") -- false

    -- additional cases
    print (matching "") -- true
    print (matching "[a(b)]") -- true
    print (matching "a{abc([])") -- false
    print (matching "aabc([])}c")  -- false
    print (matching "abc") -- true
    print (matching ")") -- false
    print (matching "(") -- false
    print (matching "(]") -- false
    print (matching "({[()]})") -- true
    print (matching "({[()]}") -- false
    print (matching "{[()]})") -- false