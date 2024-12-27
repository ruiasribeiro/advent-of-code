defmodule Aoc2022.Day08 do
  @file_path "data/day08/input.txt"

  def run do
    grid =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.map(&String.graphemes/1)
      |> Stream.map(fn line -> Enum.map(line, &String.to_integer/1) end)
      |> Enum.to_list()

    part1 = count_visible_trees(grid)
    part2 = get_best_tree(grid)

    {part1, part2}
  end

  # Utils

  defp edge_position?({line, column}, {num_lines, num_columns}),
    do: line == 0 or column == 0 or line == num_lines - 1 or column == num_columns - 1

  defp get_height({line, column}, grid), do: grid |> Enum.at(line) |> Enum.at(column)

  defp next_position({line, column}, :up), do: {line - 1, column}
  defp next_position({line, column}, :down), do: {line + 1, column}
  defp next_position({line, column}, :left), do: {line, column - 1}
  defp next_position({line, column}, :right), do: {line, column + 1}

  # Part 1

  defp count_visible_trees(grid) do
    num_lines = length(grid)
    num_columns = grid |> Enum.at(0) |> length()

    positions = for line <- 0..(num_lines - 1), column <- 0..(num_columns - 1), do: {line, column}

    positions |> Stream.filter(&visible?(&1, {num_lines, num_columns}, grid)) |> Enum.count()
  end

  defp visible?(pos, dimensions, grid) do
    edge_position?(pos, dimensions) or
      visible?(next_position(pos, :up), get_height(pos, grid), dimensions, grid, :up) or
      visible?(next_position(pos, :down), get_height(pos, grid), dimensions, grid, :down) or
      visible?(next_position(pos, :left), get_height(pos, grid), dimensions, grid, :left) or
      visible?(next_position(pos, :right), get_height(pos, grid), dimensions, grid, :right)
  end

  defp visible?(position, height, dimensions, grid, direction) do
    get_height(position, grid) < height and
      (edge_position?(position, dimensions) or
         visible?(next_position(position, direction), height, dimensions, grid, direction))
  end

  # Part 2

  defp get_best_tree(grid) do
    num_lines = length(grid)
    num_columns = grid |> Enum.at(0) |> length()

    positions = for line <- 0..(num_lines - 1), column <- 0..(num_columns - 1), do: {line, column}

    positions |> Stream.map(&scenic_score(&1, {num_lines, num_columns}, grid)) |> Enum.max()
  end

  defp scenic_score(position, dimensions, grid) do
    if edge_position?(position, dimensions) do
      0
    else
      height = get_height(position, grid)

      count_visible(next_position(position, :up), height, dimensions, grid, :up, 1) *
        count_visible(next_position(position, :down), height, dimensions, grid, :down, 1) *
        count_visible(next_position(position, :left), height, dimensions, grid, :left, 1) *
        count_visible(next_position(position, :right), height, dimensions, grid, :right, 1)
    end
  end

  defp count_visible(pos, height, dimensions, grid, direction, count) do
    if get_height(pos, grid) >= height or edge_position?(pos, dimensions) do
      count
    else
      count_visible(next_position(pos, direction), height, dimensions, grid, direction, count + 1)
    end
  end
end
