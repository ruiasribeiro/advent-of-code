defmodule Aoc2022.Day05 do
  defmodule Move do
    alias Aoc2022.Day05.Move

    defstruct [:count, :src, :dest]

    def parse(str) do
      [^str | captures] = Regex.run(~r/^move (\d+) from (\d+) to (\d+)$/, str)
      [count, src, dest] = Enum.map(captures, &String.to_integer/1)

      %Move{count: count, src: src - 1, dest: dest - 1}
    end
  end

  @file_path "data/day05/input.txt"

  def run do
    {stacks, [_ | moves]} =
      @file_path
      |> File.stream!()
      |> Enum.split_while(&(&1 != "\n"))

    stacks = stacks |> Enum.reverse() |> stacks()
    moves = moves |> Enum.map(&String.trim/1) |> Enum.map(&Move.parse/1)

    part1 = apply_moves(moves, stacks, true)
    part2 = apply_moves(moves, stacks, false)

    {part1, part2}
  end

  defp stacks([header | lines]) do
    count = div(String.length(header) + 1, 4)

    Enum.reduce(lines, List.duplicate([], count), fn line, stacks ->
      line
      |> parse_crates()
      |> Enum.zip(stacks)
      |> Enum.map(fn
        {nil, stack} -> stack
        {crate, stack} -> [crate | stack]
      end)
    end)
  end

  defp parse_crates(crates),
    do:
      Stream.unfold(crates, fn
        "" -> nil
        <<crate::bytes-size(4)>> <> rest -> {crate, rest}
      end)
      |> Stream.map(&String.trim/1)
      |> Stream.map(&String.at(&1, 1))
      |> Enum.to_list()

  def apply_moves(moves, stacks, part1),
    do:
      Enum.reduce(moves, stacks, fn move, acc ->
        {crates, src_stack} = acc |> Enum.at(move.src) |> Enum.split(move.count)
        acc = List.replace_at(acc, move.src, src_stack)

        dest_stack =
          if part1 do
            Enum.reverse(crates) ++ Enum.at(acc, move.dest)
          else
            crates ++ Enum.at(acc, move.dest)
          end

        List.replace_at(acc, move.dest, dest_stack)
      end)
      |> Enum.map_join(fn
        [] -> ""
        [head | _] -> head
      end)
end
