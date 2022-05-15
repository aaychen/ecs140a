-- References:
-- https://www.tutorialspoint.com/haskell/haskell_functions.htm

-- Find the nth number in the Fibonacci sequence
findNthNumber :: Int -> Int
findNthNumber n
    | n < 0     = 0 -- if n < 0, return 0
    | n < 2     = n -- if 0 <= n <= 1, return n
    | otherwise = findNthNumber(n-1) + findNthNumber(n-2) -- use recursion to get sum of last 2 numbers

fibSeq :: Int -> [Int]
fibSeq n = map findNthNumber [0..n] -- apply findNthNumber to array [0..n]

main = do
    -- given cases
    print (fibSeq 0) -- [0]
    print (fibSeq 1) -- [0, 1]
    print (fibSeq 9) -- [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

    -- additional cases
    print (fibSeq 2) -- [0, 1, 1]
    print (fibSeq 7) -- [0, 1, 1, 2, 3, 5, 8, 13]
    print (fibSeq 12) -- [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
    print (fibSeq 15) -- [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610]
    print (fibSeq 20) -- [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765]
