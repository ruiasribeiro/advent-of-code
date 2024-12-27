defmodule Aoc2022.Day07 do
  @file_path "data/day07/input.txt"

  def run do
    directory_sizes =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.chunk_while([], &chunk_commands/2, &after_chunk_commands/1)
      |> Enum.reduce({"/", %{}}, &map_filesystem/2)
      |> (fn {_current_dir, filesystem} -> filesystem end).()
      |> Map.to_list()
      |> Enum.sort_by(&(&1 |> elem(0) |> String.length()), :desc)
      |> get_directory_sizes()

    part1 =
      directory_sizes
      |> Map.values()
      |> Enum.filter(&(&1 < 100_000))
      |> Enum.sum()

    occupied = Map.get(directory_sizes, "/")

    part2 =
      directory_sizes
      |> Map.values()
      |> Enum.sort()
      |> Enum.find(occupied, &(occupied - &1 + 30_000_000 < 70_000_000))

    {part1, part2}
  end

  # chunk_while functions

  defp chunk_commands(line, []), do: {:cont, [line]}
  defp chunk_commands("$ cd " <> _rest = command, acc), do: {:cont, Enum.reverse(acc), [command]}
  defp chunk_commands("$ ls" = command, acc), do: {:cont, Enum.reverse(acc), [command]}
  defp chunk_commands(ls_output, acc), do: {:cont, [ls_output | acc]}

  defp after_chunk_commands(acc), do: {:cont, Enum.reverse(acc), []}

  # map the filesystem according to the given commands

  defp map_filesystem(["$ cd " <> path], {current_dir, filesystem}),
    do: {cd(current_dir, path), filesystem}

  defp map_filesystem(["$ ls" | listing], {current_dir, filesystem}),
    do: {current_dir, Map.put(filesystem, current_dir, listing)}

  defp cd(_current_dir, "/"), do: "/"

  defp cd(current_dir, ".."),
    do: current_dir |> String.split("/") |> List.pop_at(-2) |> elem(1) |> Enum.join("/")

  defp cd(current_dir, dir), do: current_dir <> dir <> "/"

  # calculate directory sizes

  defp get_directory_sizes(directories), do: get_directory_sizes(directories, %{})

  defp get_directory_sizes([], sizes), do: sizes

  defp get_directory_sizes([{path, contents} | rest], sizes) do
    path_size =
      Enum.reduce(contents, 0, fn
        "dir " <> dir, acc -> acc + Map.get(sizes, path <> dir <> "/")
        file, acc -> acc + (file |> String.split() |> Enum.at(0) |> String.to_integer())
      end)

    get_directory_sizes(rest, Map.put(sizes, path, path_size))
  end
end
