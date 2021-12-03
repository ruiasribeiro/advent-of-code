module Main where

import Data.Char (digitToInt)
import Data.List (foldl', transpose)
import Text.Printf (printf)

-- utils

count :: String -> (Int, Int)
count [] = (0, 0)
count (x : xs) =
  let (zeroes, ones) = count xs
   in case x of
        '0' -> (zeroes + 1, ones)
        '1' -> (zeroes, ones + 1)

gamma :: [(Int, Int)] -> String
gamma [] = ""
gamma ((z, o) : t)
  | z > o = '0' : gamma t
  | otherwise = '1' : gamma t

invert :: String -> String
invert [] = []
invert (x : xs) =
  case x of
    '0' -> '1' : invert xs
    '1' -> '0' : invert xs

toDec :: String -> Int
toDec = foldl' (\acc x -> acc * 2 + digitToInt x) 0

-- main

main :: IO ()
main = do
  contents <- readFile "input.txt"
  let gm = gamma $ map count $ transpose $ lines contents
  let ep = invert gm
  printf "Part 1: %d\n" (toDec gm * toDec ep)
