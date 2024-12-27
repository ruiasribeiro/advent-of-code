defmodule Aoc2022.Day14 do
  @file_path "data/day14/input.txt"

  def run do
    rock_points =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.map(&String.split(&1, " -> "))
      |> Stream.map(fn path -> Enum.map(path, &parse_coordinates/1) end)
      |> Stream.flat_map(&draw_lines/1)
      |> MapSet.new()

    lowest_rock_height = rock_points |> Enum.max_by(fn {_x, y} -> y end) |> elem(1)

    part1 = iterate(rock_points, lowest_rock_height, 0)
    part2 = iterate_pt2(rock_points, lowest_rock_height + 2, 0)

    {part1, part2}
  end

  # Part 1

  defp iterate(points, lowest_height, sand_units) do
    result = place_sand({500, 0}, points, lowest_height)

    case result do
      :abyss -> sand_units
      {:rest, points} -> iterate(points, lowest_height, sand_units + 1)
    end
  end

  defp place_sand({_sand_x, sand_y}, _points, lowest_height) when sand_y == lowest_height,
    do: :abyss

  defp place_sand({sand_x, sand_y}, points, lowest_height) do
    case check_below({sand_x, sand_y}, points) do
      :below -> place_sand({sand_x, sand_y + 1}, points, lowest_height)
      :left -> place_sand({sand_x - 1, sand_y + 1}, points, lowest_height)
      :right -> place_sand({sand_x + 1, sand_y + 1}, points, lowest_height)
      :stop -> {:rest, MapSet.put(points, {sand_x, sand_y})}
    end
  end

  # Part 2

  defp iterate_pt2(points, floor, sand_units) do
    result = place_sand_pt2({500, 0}, points, floor)

    case result do
      :blocked -> sand_units + 1
      {:rest, points} -> iterate_pt2(points, floor, sand_units + 1)
    end
  end

  defp place_sand_pt2({sand_x, sand_y}, points, floor) when sand_y == floor - 1,
    do: {:rest, MapSet.put(points, {sand_x, sand_y})}

  defp place_sand_pt2({sand_x, sand_y}, points, floor) do
    case check_below({sand_x, sand_y}, points) do
      :below -> place_sand_pt2({sand_x, sand_y + 1}, points, floor)
      :left -> place_sand_pt2({sand_x - 1, sand_y + 1}, points, floor)
      :right -> place_sand_pt2({sand_x + 1, sand_y + 1}, points, floor)
      :stop when {sand_x, sand_y} == {500, 0} -> :blocked
      :stop -> {:rest, MapSet.put(points, {sand_x, sand_y})}
    end
  end

  # Utilities

  defp parse_coordinates(str),
    do: str |> String.split(",") |> Enum.map(&String.to_integer/1) |> List.to_tuple()

  defp draw_lines(points),
    do:
      points
      |> Enum.chunk_every(2, 1, :discard)
      |> Enum.flat_map(fn [{x1, y1}, {x2, y2}] ->
        for x <- x1..x2, y <- y1..y2, do: {x, y}
      end)

  defp check_below({sand_x, sand_y}, points) do
    case MapSet.member?(points, {sand_x, sand_y + 1}) do
      false -> :below
      true -> check_left({sand_x, sand_y}, points)
    end
  end

  defp check_left({sand_x, sand_y}, points) do
    case MapSet.member?(points, {sand_x - 1, sand_y + 1}) do
      false -> :left
      true -> check_right({sand_x, sand_y}, points)
    end
  end

  defp check_right({sand_x, sand_y}, points) do
    case MapSet.member?(points, {sand_x + 1, sand_y + 1}) do
      false -> :right
      true -> :stop
    end
  end
end
