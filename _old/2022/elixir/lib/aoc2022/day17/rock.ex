defmodule Aoc2022.Day17.Rock do
  alias Aoc2022.Day17.Rock

  defstruct [:type, :pos_x, :pos_y]

  def new, do: %Rock{type: :hor_line, pos_x: 3, pos_y: 4}

  def next(rock, highest_rock_y),
    do: %Rock{type: next_type(rock.type), pos_x: 3, pos_y: highest_rock_y + 4}

  defp next_type(:hor_line), do: :plus
  defp next_type(:plus), do: :l
  defp next_type(:l), do: :ver_line
  defp next_type(:ver_line), do: :square
  defp next_type(:square), do: :hor_line

  def push(rock, chamber, ?<),
    do: if(collides?(rock, chamber, :left), do: rock, else: %{rock | pos_x: rock.pos_x - 1})

  def push(rock, chamber, ?>),
    do: if(collides?(rock, chamber, :right), do: rock, else: %{rock | pos_x: rock.pos_x + 1})

  def push(rock, chamber, :down),
    do: if(collides?(rock, chamber, :bottom), do: rock, else: %{rock | pos_y: rock.pos_y - 1})

  def collides?(rock, chamber, :bottom),
    do:
      rock
      |> points()
      |> Enum.map(fn {x, y} -> {x, y - 1} end)
      |> Enum.any?(fn {x, y} -> y == 0 or MapSet.member?(chamber, {x, y}) end)

  def collides?(rock, chamber, :left),
    do:
      rock
      |> points()
      |> Enum.map(fn {x, y} -> {x - 1, y} end)
      |> Enum.any?(fn {x, y} -> x == 0 or MapSet.member?(chamber, {x, y}) end)

  def collides?(rock, chamber, :right),
    do:
      rock
      |> points()
      |> Enum.map(fn {x, y} -> {x + 1, y} end)
      |> Enum.any?(fn {x, y} -> x == 8 or MapSet.member?(chamber, {x, y}) end)

  def points(%Rock{type: :hor_line, pos_x: pos_x, pos_y: pos_y}),
    do:
      MapSet.new([
        {pos_x, pos_y},
        {pos_x + 1, pos_y},
        {pos_x + 2, pos_y},
        {pos_x + 3, pos_y}
      ])

  def points(%Rock{type: :plus, pos_x: pos_x, pos_y: pos_y}),
    do:
      MapSet.new([
        {pos_x, pos_y + 1},
        {pos_x + 1, pos_y},
        {pos_x + 1, pos_y + 1},
        {pos_x + 1, pos_y + 2},
        {pos_x + 2, pos_y + 1}
      ])

  def points(%Rock{type: :l, pos_x: pos_x, pos_y: pos_y}),
    do:
      MapSet.new([
        {pos_x, pos_y},
        {pos_x + 1, pos_y},
        {pos_x + 2, pos_y},
        {pos_x + 2, pos_y + 1},
        {pos_x + 2, pos_y + 2}
      ])

  def points(%Rock{type: :ver_line, pos_x: pos_x, pos_y: pos_y}),
    do:
      MapSet.new([
        {pos_x, pos_y},
        {pos_x, pos_y + 1},
        {pos_x, pos_y + 2},
        {pos_x, pos_y + 3},
      ])

  def points(%Rock{type: :square, pos_x: pos_x, pos_y: pos_y}),
    do:
      MapSet.new([
        {pos_x, pos_y},
        {pos_x + 1, pos_y},
        {pos_x, pos_y + 1},
        {pos_x + 1, pos_y + 1}
      ])
end
