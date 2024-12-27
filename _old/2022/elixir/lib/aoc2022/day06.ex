defmodule Aoc2022.Day06 do
  @file_path "data/day06/input.txt"

  def run do
    part1 = get_start_marker(4)
    part2 = get_start_marker(14)

    {part1, part2}
  end

  def get_start_marker(num_distinct) do
    @file_path
    |> File.read!()
    |> String.graphemes()
    |> Stream.chunk_every(num_distinct, 1)
    |> Stream.with_index(num_distinct)
    |> Stream.filter(fn {list, _marker} ->
      list
      |> Enum.uniq()
      |> Enum.count()
      |> Kernel.==(Enum.count(list))
    end)
    |> Enum.at(0)
    |> (fn {_list, marker} -> marker end).()
  end
end
