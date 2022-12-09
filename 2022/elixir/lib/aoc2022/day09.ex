defmodule Aoc2022.Day09 do
  @file_path "data/day09/input.txt"

  def run do
    moves =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.split/1)
      |> Stream.flat_map(fn [direction, amount] ->
        List.duplicate(direction, String.to_integer(amount))
      end)
      # potential optimization: avoid conversion to list and exploit laziness of stream
      |> Enum.to_list()

    IO.puts("Part 1: #{step(moves, 2)}")
    IO.puts("Part 2: #{step(moves, 10)}")
  end

  defp step(moves, num_knots), do: step(moves, List.duplicate({0, 0}, num_knots), MapSet.new())

  defp step([], _rope, visited), do: Enum.count(visited)

  defp step([direction | moves], [head | knots], visited) do
    knots =
      Enum.reduce(knots, [next_position(head, direction)], fn
        knot, [prev_knot | rest] ->
          knot = follow_head(prev_knot, knot)
          [knot, prev_knot | rest]
      end)

    [tail | _] = knots
    visited = MapSet.put(visited, tail)

    knots = Enum.reverse(knots)

    step(moves, knots, visited)
  end

  defp next_position({line, column}, "U"), do: {line + 1, column}
  defp next_position({line, column}, "D"), do: {line - 1, column}
  defp next_position({line, column}, "L"), do: {line, column - 1}
  defp next_position({line, column}, "R"), do: {line, column + 1}

  defp follow_head({head_line, head_column} = head_pos, {tail_line, tail_column} = tail_pos) do
    if touching?(head_pos, tail_pos) do
      tail_pos
    else
      tail_line = bring_closer(head_line, tail_line)
      tail_column = bring_closer(head_column, tail_column)

      {tail_line, tail_column}
    end
  end

  defp touching?({head_line, head_column}, {tail_line, tail_column}),
    do:
      tail_line in [head_line - 1, head_line, head_line + 1] and
        tail_column in [head_column - 1, head_column, head_column + 1]

  defp bring_closer(point, follower) do
    cond do
      follower < point -> follower + 1
      follower > point -> follower - 1
      true -> follower
    end
  end
end
