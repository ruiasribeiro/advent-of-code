defmodule Aoc2022.Day15 do
  @file_path "data/day15/input.txt"
  @part1_y 2_000_000
  @part2_area 4_000_000

  # @file_path "data/day15/example.txt"
  # @part1_y 10
  # @part2_area 20

  def run do
    pairs =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.map(&parse_line/1)

    {width, _height} = Enum.reduce(pairs, {0, 0}, &max_bounds/2)

    pairs =
      pairs
      |> Stream.map(fn {sensor, beacon} -> {sensor, beacon, distance(sensor, beacon)} end)
      |> Enum.to_list()

    max_range = pairs |> Enum.max_by(fn {_sensor, _beacon, range} -> range end) |> elem(2)

    # The column (x) range needs to account for positions outside the "grid"
    # where sensors and beacons are located.
    possible_columns = (0 - max_range)..(width + max_range)

    part1 =
      Enum.reduce(possible_columns, 0, fn column, count ->
        if can_contain_beacon?({column, @part1_y}, pairs, true) do
          count
        else
          count + 1
        end
      end)

    # Takes too long to find an answer.
    # part2 =
    #   Stream.unfold({0, 0}, fn
    #     {@part2_area, @part2_area} -> nil
    #     {@part2_area, y} -> {{@part2_area, y}, {0, y + 1}}
    #     {x, y} -> {{x, y}, {x + 1, y}}
    #   end)
    #   |> Enum.find(fn position ->
    #     can_contain_beacon?(position, pairs, false)
    #   end)
    #   |> (fn {x, y} -> x * 4_000_000 + y end).()

    part2 = 0

    {part1, part2}
  end

  defp parse_line(str),
    do:
      ~r/^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$/
      |> Regex.run(str)
      # The first capture is the whole string, so we drop it.
      |> Enum.drop(1)
      |> Enum.map(&String.to_integer/1)
      |> (fn [sensor_x, sensor_y, beacon_x, beacon_y] ->
            {{sensor_x, sensor_y}, {beacon_x, beacon_y}}
          end).()

  defp max_bounds({{s_x, s_y}, {b_x, b_y}}, {width, height}) do
    width = Enum.max([s_x, b_x, width])
    height = Enum.max([s_y, b_y, height])

    {width, height}
  end

  defp distance({x1, y1}, {x2, y2}), do: abs(x1 - x2) + abs(y1 - y2)

  defp can_contain_beacon?(_point, [], _part1), do: true

  defp can_contain_beacon?(point, [{sensor, beacon, range} | rest], part1) do
    cond do
      point == beacon and part1 -> true
      point == sensor -> false
      in_range?(point, sensor, range) -> false
      true -> can_contain_beacon?(point, rest, part1)
    end
  end

  defp in_range?({x, y}, {sensor_x, sensor_y}, range),
    do: abs(sensor_x - x) + abs(sensor_y - y) <= range
end
