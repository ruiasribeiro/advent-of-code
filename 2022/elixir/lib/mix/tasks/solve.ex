defmodule Mix.Tasks.Solve do
  use Mix.Task

  def run([day]) do
    module = "Elixir.Aoc2022.Day#{String.pad_leading(day, 2, "0")}" |> String.to_existing_atom()

    IO.puts("Solving day #{day}...")
    {u_secs, :ok} = :timer.tc(module, :run, [])
    IO.puts("Took #{u_secs / 1000} ms")
  end
end
