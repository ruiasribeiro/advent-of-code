defmodule Aoc2022.Day13 do
  @file_path "data/day13/input.txt"

  def run do
    pairs =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.filter(&(!Kernel.==(&1, "")))
      |> Stream.map(&Code.eval_string/1)
      |> Stream.map(&elem(&1, 0))
      |> Stream.chunk_every(2)

    part1 =
      pairs
      |> Stream.map(&ordered?/1)
      |> Stream.with_index(1)
      |> Stream.filter(fn {ordered, _index} -> ordered end)
      |> Stream.map(&elem(&1, 1))
      |> Enum.sum()

    {part1, 0}
  end

  defp ordered?([left, right]) do
    compare({left, right}) == :right
  end

  defp compare({left, right}) when is_integer(left) and is_integer(right) do
    cond do
      left < right -> :right
      left > right -> :wrong
      true -> :same
    end
  end

  defp compare({[], []}), do: :same
  defp compare({[], right}) when is_list(right), do: :right
  defp compare({left, []}) when is_list(left), do: :wrong

  defp compare({[left_head | left], [right_head | right]}) do
    case compare({left_head, right_head}) do
      :same -> compare({left, right})
      other -> other
    end
  end

  defp compare({left, right}) when is_integer(left), do: compare({[left], right})
  defp compare({left, right}) when is_integer(right), do: compare({left, [right]})
end
