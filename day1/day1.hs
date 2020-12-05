-- day 1 of advent of code


-----------------------------------------------------------------


part1 = do
   content <- readFile ("input")                                  -- read input file
   let linesOfFile = map (read :: String -> Int) (lines content)  -- split content into its lines 
                                                                  -- and convert each one of them to integer
   print(twoNumbers linesOfFile linesOfFile)                      -- show the result


twoNumbers :: [Int] -> [Int] -> Maybe Int
twoNumbers []    l2 = Nothing
twoNumbers (h:t) l2 | elem (2020 - h) l2 = Just (h * (2020 - h))
                    | otherwise          = twoNumbers t l2

                     
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
threeNumbersAux x (h:t) l3 | elem (2020 - x - h) l3 = Just (x * h * (2020 - x - h))
                           | otherwise              = threeNumbersAux x t l3