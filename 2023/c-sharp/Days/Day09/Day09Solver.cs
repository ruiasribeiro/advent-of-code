namespace Aoc2023.Days.Day09;

public partial class Day09Solver : ISolver
{
    public static (object, object) Solve()
    {
        var lines = File.ReadAllLines("Days/Day09/input.txt");

        var part1 = SolvePart1(lines);
        var part2 = SolvePart2(lines);

        return (part1, part2);
    }

    private static long SolvePart1(string[] lines)
    {
        List<List<List<int>>> histories = [];

        foreach (var line in lines)
        {
            var history = line.Split(' ').Select(num => int.Parse(num.Trim())).ToList();

            List<List<int>> differences = [history];

            var last_differences = differences.Last();

            while (!last_differences.All(number => number == 0))
            {
                List<int> current_differences = [];

                for (var i = 1; i < last_differences.Count; ++i)
                {
                    current_differences.Add(last_differences[i] - last_differences[i - 1]);
                }

                differences.Add(current_differences);
                last_differences = current_differences;
            }

            differences.Last().Add(0);

            for (var i = differences.Count - 2; i >= 0; --i)
            {
                differences[i].Add(differences[i].Last() + differences[i + 1].Last());
            }

            histories.Add(differences);
        }

        var sum = 0L;

        foreach (var history in histories)
        {
            sum += history.First().Last();
        }

        return sum;
    }

    private static long SolvePart2(string[] lines)
    {
         List<List<List<int>>> histories = [];

        foreach (var line in lines)
        {
            var history = line.Split(' ').Select(num => int.Parse(num.Trim())).ToList();

            List<List<int>> differences = [history];

            var last_differences = differences.Last();

            while (!last_differences.All(number => number == 0))
            {
                List<int> current_differences = [];

                for (var i = 1; i < last_differences.Count; ++i)
                {
                    current_differences.Add(last_differences[i] - last_differences[i - 1]);
                }

                differences.Add(current_differences);
                last_differences = current_differences;
            }

            differences.Last().Insert(0, 0);

            for (var i = differences.Count - 2; i >= 0; --i)
            {
                differences[i].Insert(0, differences[i].First() - differences[i + 1].First());
            }

            histories.Add(differences);
        }

        var sum = 0L;

        foreach (var history in histories)
        {
            sum += history.First().First();
        }

        return sum;
    }
}
