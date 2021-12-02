module Main where

import Data.List (foldl')
import Text.Printf (printf)

-- utils

data Command = Forward Int | Down Int | Up Int deriving (Show)

parseCommand :: String -> Command
parseCommand x =
  let [c, v] = words x
      value = read v
   in case c of
        "forward" -> Forward value
        "down" -> Down value
        "up" -> Up value

parseCommands :: String -> [Command]
parseCommands = map parseCommand . lines

-- part 1

part1 :: [Command] -> Int
part1 = uncurry (*) . foldr f (0, 0)
  where
    f comm (hor, dep) =
      case comm of
        Forward v -> (hor + v, dep)
        Down v -> (hor, dep + v)
        Up v -> (hor, dep - v)

-- part 2

part2 :: [Command] -> Int
part2 l = h * d
  where
    -- foldl must be used here because of command order
    -- foldr would give an incorrect result
    (h, d, _) = foldl' f (0, 0, 0) l
    f (hor, dep, aim) comm =
      case comm of
        Forward v -> (hor + v, dep + (aim * v), aim)
        Down v -> (hor, dep, aim + v)
        Up v -> (hor, dep, aim - v)

-- main

main :: IO ()
main = do
  contents <- readFile "input.txt"
  let commands = parseCommands contents
  printf "Part 1: %d\n" (part1 commands)
  printf "Part 2: %d\n" (part2 commands)
