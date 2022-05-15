-- Find the longest common prefix between 2 strings
cmpStr :: String -> String -> String
cmpStr s1 s2 = do
    if s1 == [] || s2 == [] -- empty string check
        then []
    else do
        -- compare first chars of both strings
        let (x:xs) = s1
        let (y:ys) = s2
        if x == y
            then [x] ++ cmpStr xs ys -- store char and recurse on rest of both strings
        else [] -- no chars to add on if mismatch

-- Uses helper function to get longest common prefix of all strings
lcp :: [String] -> String
lcp arr = do
    if arr == [] -- empty list check
        then []
    else do
        let dummy = head arr
        foldr cmpStr dummy arr -- use fold to apply helper function on the list of strings

main = do
    -- given case
    print (lcp ["apple", "app", "aple", "appl"])

    -- additional cases
    print (lcp []) -- ""
    print (lcp [""]) -- ""
    print (lcp ["abc"]) -- "abc"
    print (lcp ["abc", "xyz"]) -- ""
    print (lcp ["zzzzz", "zz", "zzzz"]) -- "zz"
    print (lcp ["bamboo", "bamboozled"]) -- "bamboo"
    print (lcp ["bamboo", "bamboozled", "bambam"]) -- "bamb"
