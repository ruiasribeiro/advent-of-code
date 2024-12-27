defmodule Aoc2022.Day18 do
  @file_path "data/day18/input.txt"

  def run do
    cubes =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.map(&parse_point/1)
      |> MapSet.new()

    part1 =
      Enum.reduce(cubes, 0, fn cube, count ->
        count + count_uncovered_sides(cube, cubes)
      end)

    part2 = 0

    {part1, part2}
  end

  defp parse_point(str),
    do: str |> String.split(",") |> Enum.map(&String.to_integer/1) |> List.to_tuple()

  defp count_uncovered_sides(cube, cubes),
    do:
      cube
      |> get_adjacent()
      |> Enum.filter(fn adjacent_cube -> not MapSet.member?(cubes, adjacent_cube) end)
      |> Enum.count()

  defp get_adjacent({x, y, z}),
    do: [{x + 1, y, z}, {x - 1, y, z}, {x, y + 1, z}, {x, y - 1, z}, {x, y, z + 1}, {x, y, z - 1}]
end
