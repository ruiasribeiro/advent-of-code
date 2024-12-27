defmodule Aoc2022.Day17 do
  alias Aoc2022.Day17.Rock

  @file_path "data/day17/input.txt"

  # Some notes:
  # - The (0, 0) point is the lower left corner of the chamber (inside the
  #   wall).
  # - Any point with x = 0 or x = 8 is inside the wall, left and right
  #   respectively.

  def run do
    move_stream =
      @file_path
      |> File.read!()
      |> String.trim()
      |> String.to_charlist()
      |> Stream.cycle()

    part1 = run_until(move_stream, 2022)

    # Not optimized enough for part 2.
    # part2 = run_until(move_stream, 1_000_000_000_000)
    part2 = 0

    {part1, part2}
  end

  defp run_until(move_stream, rock_limit) do
    move_stream
    |> Enum.reduce_while({MapSet.new(), Rock.new(), 0}, fn
      _push, {chamber, _next_rock, ^rock_limit} ->
        {:halt, chamber}

      push, {chamber, next_rock, fallen_rocks} ->
        case move(push, chamber, next_rock) do
          {:rest, chamber, next_rock} ->
            {:cont, {chamber, next_rock, fallen_rocks + 1}}

          {:falling, same_rock} ->
            {:cont, {chamber, same_rock, fallen_rocks}}
        end
    end)
    |> calculate_chamber_height()
  end

  defp move(push, chamber, rock) do
    rock = Rock.push(rock, chamber, push)

    if Rock.collides?(rock, chamber, :bottom) do
      chamber = MapSet.union(chamber, Rock.points(rock))
      next_rock = Rock.next(rock, calculate_chamber_height(chamber))
      {:rest, chamber, next_rock}
    else
      rock = Rock.push(rock, chamber, :down)
      {:falling, rock}
    end
  end

  defp calculate_chamber_height(chamber),
    do:
      chamber
      |> Enum.max_by(fn {_x, y} -> y end, fn -> 0 end)
      |> (fn {_x, y} -> y end).()
end
