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

toDec :: String -> Int
toDec = foldl' (\acc x -> acc * 2 + digitToInt x) 0

-- part 1

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

-- part 2

oxygen :: Int -> [String] -> String
oxygen _ [x] = x
oxygen col l
  | z > o = oxygen (col + 1) $ filter (('0' ==) . (!! col)) l
  | z <= o = oxygen (col + 1) $ filter (('1' ==) . (!! col)) l
  where
    (z, o) = count $ (!! col) $ transpose l

dioxide :: Int -> [String] -> String
dioxide _ [x] = x
dioxide col l
  | z > o = dioxide (col + 1) $ filter (('1' ==) . (!! col)) l
  | z <= o = dioxide (col + 1) $ filter (('0' ==) . (!! col)) l
  where
    (z, o) = count $ (!! col) $ transpose l

-- main

main :: IO ()
main = do
  contents <- readFile "input.txt"
  let gm = gamma $ map count $ transpose $ lines contents
  let ep = invert gm
  printf "Part 1: %d\n" (toDec gm * toDec ep)
  let ox = oxygen 0 $ lines contents
  let di = dioxide 0 $ lines contents
  printf "Part 2: %d\n" (toDec ox * toDec di)
