-- day 1 of advent of code


-----------------------------------------------------------------


part1 = do
   content <- readFile ("input")                                  -- read input file
   let linesOfFile = map (read :: String -> Int) (lines content)  -- split content into its lines 
                                                                  -- and convert each one of them to integer
   print(twoNumbers linesOfFile linesOfFile)                      -- show the result


twoNumbers :: [Int] -> [Int] -> Maybe Int
twoNumbers []    l2 = Nothing
twoNumbers (h:t) l2 | r == Nothing = twoNumbers t l2
                    | otherwise    = r
                    where r = twoNumbersAux h l2


twoNumbersAux :: Int -> [Int] -> Maybe Int
twoNumbersAux x []    = Nothing
twoNumbersAux x (h:t) | x + h == 2020 = Just (x * h)
                      | otherwise     = twoNumbersAux x t

                     
-----------------------------------------------------------------


part2 = do
   content <- readFile ("input")                                  -- read input file
   let linesOfFile = map (read :: String -> Int) (lines content)  -- split content into its lines 
                                                                  -- and convert each one of them to integer
   print(threeNumbers linesOfFile linesOfFile linesOfFile)        -- show the result


threeNumbers :: [Int] -> [Int] -> [Int] -> Maybe Int
threeNumbers []    l2 l3 = Nothing
threeNumbers (h:t) l2 l3 | r == Nothing = threeNumbers t l2 l3
                         | otherwise    = r
                         where r = threeNumbersAux h l2 l3


threeNumbersAux :: Int -> [Int] -> [Int] -> Maybe Int
threeNumbersAux x []    l3 = Nothing
threeNumbersAux x (h:t) l3 | r == Nothing = threeNumbersAux x t l3
                           | otherwise    = r
                           where r = threeNumbersAux2 x h l3

threeNumbersAux2 :: Int -> Int -> [Int] -> Maybe Int
threeNumbersAux2 x y []    = Nothing
threeNumbersAux2 x y (h:t) | x + y + h == 2020 = Just (x * y * h)
                           | otherwise         = threeNumbersAux2 x y t