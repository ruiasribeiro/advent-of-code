defmodule Mix.Tasks.Solve do
  use Mix.Task

  def run([day]) do
    module = "Elixir.Aoc2022.Day#{String.pad_leading(day, 2, "0")}" |> String.to_existing_atom()

    IO.puts("Solving day #{day}...")

    {u_secs, {part1, part2}} = :timer.tc(module, :run, [])

    IO.puts("Part 1: #{part1}")
    IO.puts("Part 2: #{part2}")
    IO.puts("Took #{u_secs / 1000} ms")
  end
end
