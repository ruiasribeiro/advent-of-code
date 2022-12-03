defmodule Main do
  @file_path "input.txt"

  def run do
    file_stream =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)

    part1 =
      file_stream
      |> Stream.map(&split_in_half/1)
      |> Stream.map(&get_shared_pt1/1)
      |> Stream.map(&priority/1)
      |> Enum.sum()

    IO.puts("Part 1: #{part1}")

    part2 =
      file_stream
      |> Stream.chunk_every(3)
      |> Stream.map(&get_shared_pt2/1)
      |> Stream.map(&priority/1)
      |> Enum.sum()

    IO.puts("Part 2: #{part2}")
  end

  # Utils

  defp split_in_half(str), do: String.split_at(str, div(String.length(str), 2))

  defp priority(item) do
    case item do
      item when ?a <= item and item <= ?z -> item - ?a + 1
      item when ?A <= item and item <= ?Z -> item - ?A + 27
      _ -> raise "invalid item: #{item}"
    end
  end

  # Part 1

  defp get_shared_pt1({left, right}),
    do: get_shared_pt1(String.to_charlist(left), String.to_charlist(right))

  defp get_shared_pt1([head | tail], right) do
    if Enum.member?(right, head) do
      head
    else
      get_shared_pt1(tail, right)
    end
  end

  # Part 2

  defp get_shared_pt2([fst, _snd, _trd] = list) when is_bitstring(fst),
    do: list |> Enum.map(&String.to_charlist/1) |> get_shared_pt2()

  defp get_shared_pt2([[head | tail], snd, trd]) do
    if Enum.member?(snd, head) and Enum.member?(trd, head) do
      head
    else
      get_shared_pt2([tail, snd, trd])
    end
  end
end

Main.run()
