defmodule Aoc2022.Day10 do
  @file_path "data/day10/input.txt"

  def run do
    {_, _, part1} =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.map(fn
        "noop" -> {1, :noop}
        "addx " <> value -> {2, {:addx, String.to_integer(value)}}
      end)
      |> Enum.reduce({0, 1, 0}, fn op, state -> advance(op, state) end)

    part2 = "printed out above"

    {part1, part2}
  end

  defp advance({1, op}, {cycle_count, register, signal_sum}) do
    output(cycle_count, register)

    cycle_count = cycle_count + 1
    signal_sum = signal_sum + signal_strength(cycle_count, register)
    register = execute_op(op, register)

    {cycle_count, register, signal_sum}
  end

  defp advance({cycles, op}, {cycle_count, register, signal_sum}) do
    output(cycle_count, register)

    cycle_count = cycle_count + 1
    signal_sum = signal_sum + signal_strength(cycle_count, register)

    advance({cycles - 1, op}, {cycle_count, register, signal_sum})
  end

  defp execute_op(:noop, register), do: register
  defp execute_op({:addx, value}, register), do: register + value

  defp signal_strength(cycle_count, register) when rem(cycle_count + 20, 40) == 0,
    do: cycle_count * register

  defp signal_strength(_cycle_count, _register), do: 0

  defp output(cycle_count, register) do
    if rem(cycle_count, 40) in [register - 1, register, register + 1] do
      IO.write("#")
    else
      IO.write(".")
    end

    if rem(cycle_count + 1, 40) == 0 do
      IO.write("\n")
    end
  end
end
