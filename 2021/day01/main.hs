module Main where

import Data.Maybe (mapMaybe)
import Text.Printf (printf)
import Text.Read (readMaybe)

-- utils

parseNumbers :: String -> [Int]
parseNumbers = mapMaybe readMaybe . lines

-- part 1

measureCount :: [Int] -> Int
measureCount [] = 0
measureCount [x] = 0
measureCount (x : y : t) = (if x < y then 1 else 0) + measureCount (y : t)

-- part 2

windowSums :: [Int] -> [Int]
windowSums [] = []
windowSums [x] = []
windowSums [x, y] = []
windowSums (x : y : z : t) = (x + y + z) : windowSums (y : z : t)

-- main

main :: IO ()
main = do
  contents <- readFile "input.txt"
  let nums = parseNumbers contents
  printf "Part 1: %d\n" (measureCount nums)
  printf "Part 2: %d\n" (measureCount (windowSums nums))
