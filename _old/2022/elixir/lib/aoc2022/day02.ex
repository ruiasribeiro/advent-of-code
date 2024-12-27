defmodule Aoc2022.Day02 do
  @file_path "data/day02/input.txt"

  def run do
    rounds =
      File.stream!(@file_path)
      |> Stream.map(&String.split/1)

    part1 =
      rounds
      |> Stream.map(&outcome_pt1/1)
      |> Enum.sum()

    part2 =
      rounds
      |> Stream.map(&outcome_pt2/1)
      |> Enum.sum()

    {part1, part2}
  end

  # Part 1

  defp outcome_pt1(["A", "X"]), do: 1 + 3
  defp outcome_pt1(["A", "Y"]), do: 2 + 6
  defp outcome_pt1(["A", "Z"]), do: 3 + 0

  defp outcome_pt1(["B", "X"]), do: 1 + 0
  defp outcome_pt1(["B", "Y"]), do: 2 + 3
  defp outcome_pt1(["B", "Z"]), do: 3 + 6

  defp outcome_pt1(["C", "X"]), do: 1 + 6
  defp outcome_pt1(["C", "Y"]), do: 2 + 0
  defp outcome_pt1(["C", "Z"]), do: 3 + 3

  # Part 2

  defp outcome_pt2(["A", "X"]), do: 3 + 0
  defp outcome_pt2(["A", "Y"]), do: 1 + 3
  defp outcome_pt2(["A", "Z"]), do: 2 + 6

  defp outcome_pt2(["B", "X"]), do: 1 + 0
  defp outcome_pt2(["B", "Y"]), do: 2 + 3
  defp outcome_pt2(["B", "Z"]), do: 3 + 6

  defp outcome_pt2(["C", "X"]), do: 2 + 0
  defp outcome_pt2(["C", "Y"]), do: 3 + 3
  defp outcome_pt2(["C", "Z"]), do: 1 + 6
end
