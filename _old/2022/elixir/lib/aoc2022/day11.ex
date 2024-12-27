defmodule Aoc2022.Day11 do
  @file_path "data/day11/input.txt"

  def run do
    monkeys =
      @file_path
      |> File.stream!()
      |> Stream.map(&String.trim/1)
      |> Stream.chunk_by(&Kernel.==(&1, ""))
      |> Stream.filter(fn list -> list != [""] end)
      |> Stream.map(&parse_monkey/1)
      |> Enum.reduce(&Map.merge/2)

    part1 =
      1..20
      |> Enum.reduce(monkeys, fn _, acc -> run_round(acc) end)
      |> Map.to_list()
      |> Enum.map(fn {_key, val} -> val.inspections end)
      |> Enum.sort(:desc)
      |> Enum.take(2)
      |> Enum.product()

    # This avoids storing larger and larger numbers for the worry levels of the
    # items, while maintaining the divisible test correct.
    modulo = monkeys |> Enum.map(fn {_key, val} -> val.test end) |> Enum.product()

    part2 =
      1..10_000
      |> Enum.reduce(monkeys, fn _, acc -> run_round_pt2(acc, modulo) end)
      |> Map.to_list()
      |> Enum.map(fn {_key, val} -> val.inspections end)
      |> Enum.sort(:desc)
      |> Enum.take(2)
      |> Enum.product()

    {part1, part2}
  end

  # Parsing

  defp parse_monkey([monkey, starting, operation, test, throw_true, throw_false]) do
    monkey =
      ~r/^Monkey (\d+):$/
      |> Regex.run(monkey)
      |> Enum.at(1)
      |> String.to_integer()

    starting =
      starting
      |> String.split(":")
      |> Enum.at(1)
      |> String.split(",")
      |> Enum.map(&String.trim/1)
      |> Enum.map(&String.to_integer/1)

    operation =
      operation
      |> String.split(" = ")
      |> Enum.at(1)
      |> parse_operation()

    test =
      ~r/^Test: divisible by (\d+)$/
      |> Regex.run(test)
      |> Enum.at(1)
      |> String.to_integer()

    throw_true =
      ~r/^If true: throw to monkey (\d+)$/
      |> Regex.run(throw_true)
      |> Enum.at(1)
      |> String.to_integer()

    throw_false =
      ~r/^If false: throw to monkey (\d+)$/
      |> Regex.run(throw_false)
      |> Enum.at(1)
      |> String.to_integer()

    %{
      monkey => %{
        items: starting,
        operation: operation,
        test: test,
        throw_true: throw_true,
        throw_false: throw_false,
        inspections: 0
      }
    }
  end

  defp parse_operation(str) do
    [left, op, right] = String.split(str)
    {parse_number(left), op, parse_number(right)}
  end

  defp parse_number("old"), do: :old
  defp parse_number(num), do: String.to_integer(num)

  # Run part 1

  defp run_round(monkeys), do: run_round(0, monkeys)
  defp run_round(monkey_num, monkeys) when map_size(monkeys) == monkey_num, do: monkeys

  defp run_round(monkey_num, monkeys) do
    monkey = monkeys[monkey_num]

    case monkey.items do
      [] ->
        run_round(monkey_num + 1, monkeys)

      [head | tail] ->
        {worry_level, target_num} = do_inspect(head, monkey)

        monkeys =
          monkeys
          |> put_in([monkey_num, :items], tail)
          |> update_in([monkey_num, :inspections], &(&1 + 1))
          |> update_in([target_num, :items], &(&1 ++ [worry_level]))

        run_round(monkey_num, monkeys)
    end
  end

  defp do_inspect(
         item,
         %{operation: op, test: test, throw_true: throw_t, throw_false: throw_f}
       ) do
    worry_level =
      item
      |> calculate_worry_level(op)
      |> div(3)

    target = if rem(worry_level, test) == 0, do: throw_t, else: throw_f

    {worry_level, target}
  end

  # Run part 2

  defp run_round_pt2(monkeys, modulo), do: run_round_pt2(0, monkeys, modulo)

  defp run_round_pt2(monkey_num, monkeys, _modulo) when map_size(monkeys) == monkey_num,
    do: monkeys

  defp run_round_pt2(monkey_num, monkeys, modulo) do
    monkey = monkeys[monkey_num]

    case monkey.items do
      [] ->
        run_round_pt2(monkey_num + 1, monkeys, modulo)

      [head | tail] ->
        {worry_level, target_num} = do_inspect_pt2(head, monkey, modulo)

        monkeys =
          monkeys
          |> put_in([monkey_num, :items], tail)
          |> update_in([monkey_num, :inspections], &(&1 + 1))
          |> update_in([target_num, :items], &(&1 ++ [worry_level]))

        run_round_pt2(monkey_num, monkeys, modulo)
    end
  end

  defp do_inspect_pt2(
         item,
         %{operation: op, test: test, throw_true: throw_t, throw_false: throw_f},
         modulo
       ) do
    worry_level =
      item
      |> calculate_worry_level(op)
      |> rem(modulo)

    target = if rem(worry_level, test) == 0, do: throw_t, else: throw_f

    {worry_level, target}
  end

  # Utilities

  defp calculate_worry_level(old, {left, "*", right}),
    do: get_number(left, old) * get_number(right, old)

  defp calculate_worry_level(old, {left, "+", right}),
    do: get_number(left, old) + get_number(right, old)

  defp get_number(:old, old), do: old
  defp get_number(num, _old), do: num
end
