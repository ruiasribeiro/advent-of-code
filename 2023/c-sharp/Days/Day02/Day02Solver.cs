namespace Aoc2023.Days.Day02;

public class Day02Solver : ISolver
{
    public static (object, object) Solve()
    {
        var lines = File.ReadAllLines("Days/Day02/input.txt");

        var part1 = SolvePart1(lines);
        var part2 = SolvePart2(lines);

        return (part1, part2);
    }

    private static int SolvePart1(string[] lines)
    {
        int sum = 0;

        foreach (var line in lines)
        {
            var substrings = line.Split(':', ';');
            var id = int.Parse(substrings[0].Split(' ').Last());

            var isPossible = true;

            foreach (var set in substrings.Skip(1))
            {
                var cubes = set.Split(',');

                foreach (var cube in cubes)
                {
                    var cubeData = cube.Trim().Split(' ');
                    var quantity = int.Parse(cubeData[0]);
                    var color = cubeData[1];

                    var validCube = color switch
                    {
                        "red" => quantity <= 12,
                        "green" => quantity <= 13,
                        "blue" => quantity <= 14,
                        _ => throw new NotImplementedException()
                    };

                    isPossible = isPossible && validCube;
                    if (!isPossible)
                    {
                        break;
                    }
                }

                if (!isPossible)
                {
                    break;
                }
            }

            sum += isPossible ? id : 0;
        }

        return sum;
    }

    private static int SolvePart2(string[] lines)
    {
        int sum = 0;

        foreach (var line in lines)
        {
            var substrings = line.Split(':', ';');
            var id = int.Parse(substrings[0].Split(' ').Last());

            int[] minColor = [0, 0, 0];

            foreach (var set in substrings.Skip(1))
            {
                var cubes = set.Split(',');

                foreach (var cube in cubes)
                {
                    var cubeData = cube.Trim().Split(' ');
                    var quantity = int.Parse(cubeData[0]);
                    var color = cubeData[1];

                    var index = color switch
                    {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => throw new NotImplementedException()
                    };

                    minColor[index] = minColor[index] >= quantity ? minColor[index] : quantity;
                }
            }

            sum += minColor.Aggregate(1, (a, b) => a * b);
        }

        return sum;
    }
}
