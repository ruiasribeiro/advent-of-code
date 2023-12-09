namespace Aoc2023.Days.Day06;

public class Day06Solver : ISolver
{
    public static (object, object) Solve()
    {
        var lines = File.ReadAllLines("Days/Day06/input.txt");

        var part1 = SolvePart1(lines);
        var part2 = SolvePart2(lines);

        return (part1, part2);
    }

    private static long SolvePart1(string[] lines)
    {
        var times = lines[0]
            .Split(':')[1]
            .Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(int.Parse)
            .ToList();

        var distances = lines[1]
            .Split(':')[1]
            .Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(int.Parse)
            .ToList();

        var beats = Enumerable.Repeat(0, times.Count).ToList();

        for (var i = 0; i < times.Count; ++i)
        {
            for (var hold = 1; hold < times[i]; ++hold)
            {
                if (hold * (times[i] - hold) > distances[i])
                {
                    ++beats[i];
                }
            }
        }

        return beats.Aggregate(1, (acc, num) => acc * num);
    }

    private static long SolvePart2(string[] lines)
    {
        var time = long.Parse(
            lines[0]
                .Split(':')[1]
                .Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
                .Aggregate("", (acc, number) => acc + number)
        );

        var distance = long.Parse(
            lines[1]
                .Split(':')[1]
                .Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
                .Aggregate("", (acc, number) => acc + number)
        );

        var beat = 0L;

        for (var hold = 1L; hold < time; ++hold)
        {
            if (hold * (time - hold) > distance)
            {
                ++beat;
            }
        }

        return beat;
    }
}
