defmodule Aoc2022.Day01 do
  @file_path "data/day01/input.txt"

  def run do
    calories =
      File.stream!(@file_path)
      |> Stream.map(&String.trim/1)
      |> Stream.chunk_by(&Kernel.==(&1, ""))
      |> Stream.filter(&inventory?/1)
      |> Stream.map(&total_calories/1)

    part1 = Enum.max(calories)
    part2 = calories |> Enum.sort(:desc) |> Enum.take(3) |> Enum.sum()

    {part1, part2}
  end

  defp inventory?([""]), do: false
  defp inventory?(_), do: true

  defp total_calories(inv), do: inv |> Enum.map(&String.to_integer/1) |> Enum.sum()
end
