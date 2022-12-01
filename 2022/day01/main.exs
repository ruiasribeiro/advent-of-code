defmodule Main do
  @file_path "input.txt"

  def run do
    calories =
      File.stream!(@file_path)
      |> Stream.map(&String.trim/1)
      |> Stream.chunk_by(&Kernel.==(&1, ""))
      |> Stream.filter(&inventory?/1)
      |> Stream.map(&total_calories/1)

    part1 = Enum.max(calories)
    IO.puts("Part 1: #{part1}")

    part2 = calories |> Enum.sort(:desc) |> Enum.take(3) |> Enum.sum()
    IO.puts("Part 2: #{part2}")
  end

  def inventory?([""]), do: false
  def inventory?(_), do: true

  def total_calories(inv), do: inv |> Enum.map(&String.to_integer/1) |> Enum.sum()
end

Main.run()
