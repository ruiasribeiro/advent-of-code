defmodule Main do
  @file_path "input.txt"

  def run do
    pairs =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.map(&String.split(&1, ","))
      |> Stream.map(&parse_pairs/1)

    part1 =
      pairs
      |> Stream.map(&fully_contain?/1)
      |> Enum.count(&Function.identity/1)

    IO.puts("Part 1: #{part1}")

    part2 =
      pairs
      |> Stream.map(&overlap?/1)
      |> Enum.count(&Function.identity/1)

    IO.puts("Part 2: #{part2}")
  end

  def parse_pairs(list), do: list |> Enum.map(&parse_interval/1) |> List.to_tuple()

  def parse_interval(str),
    do: str |> String.split("-") |> Enum.map(&String.to_integer/1) |> List.to_tuple()

  def fully_contain?({{beg1, end1}, {beg2, end2}}),
    do: (beg1 >= beg2 and end1 <= end2) or (beg2 >= beg1 and end2 <= end1)

  def overlap?({{beg1, end1}, {beg2, end2}}),
    do:
      (beg1 >= beg2 and beg1 <= end2) or (end1 >= beg2 and end1 <= end2) or
        (beg2 >= beg1 and beg2 <= end1) or (end2 >= beg1 and end2 <= end1)
end

{u_secs, :ok} = :timer.tc(Main, :run, [])
IO.puts("Took #{u_secs / 1000} ms")
