using System.Diagnostics.Tracing;

namespace Aoc2023.Days.Day03;

public class Day03Solver : ISolver
{
    public static (object, object) Solve()
    {
        var lines = File.ReadAllLines("Days/Day03/input.txt");

        var part1 = SolvePart1(lines);
        var part2 = SolvePart2(lines);

        return (part1, part2);
    }

    private static int SolvePart1(string[] lines)
    {
        var (numbers, symbols) = ParseSchematicPart1(lines);

        var sum = 0;

        foreach (var number in numbers)
        {
            if (number.Points.Any(point => point.HasAdjacentSymbols(symbols)))
            {
                sum += number.Value;
            }
        }

        return sum;
    }

    private static (HashSet<Number>, HashSet<Point>) ParseSchematicPart1(string[] lines)
    {
        var numbers = new HashSet<Number>();
        var symbols = new HashSet<Point>();

        for (var line = 0; line < lines.Length; ++line)
        {
            Number? number = null;

            for (var column = 0; column < lines[line].Length; ++column)
            {
                var currentChar = lines[line][column];

                if (char.IsDigit(currentChar))
                {
                    number ??= new Number() { Value = 0, Points = new HashSet<Point>() };

                    number.Value = number.Value * 10 + (currentChar - '0');
                    number.Points.Add(new(line, column));
                }
                else
                {
                    if (number is not null)
                    {
                        numbers.Add(number);
                        number = null;
                    }

                    if (currentChar != '.')
                    {
                        symbols.Add(new(line, column));
                    }
                }
            }

            if (number is not null)
            {
                numbers.Add(number);
            }
        }

        return (numbers, symbols);
    }

    private class Number()
    {
        public int Value { get; set; }
        public ISet<Point> Points { get; set; } = null!;
    };

    private record Point(int Line, int Column)
    {
        public bool HasAdjacentSymbols(IEnumerable<Point> symbolPositions)
        {
            foreach (var symbol in symbolPositions)
            {
                if (Math.Abs(Line - symbol.Line) <= 1 && Math.Abs(Column - symbol.Column) <= 1)
                {
                    return true;
                }
            }

            return false;
        }
    };

    private static int SolvePart2(string[] lines)
    {
        var (numbers, symbols) = ParseSchematicPart2(lines);

        var sum = 0;

        foreach (var symbol in symbols)
        {
            sum += symbol.GetGearRatio(numbers) ?? 0;
        }

        return sum;
    }

    private static (HashSet<ExtendedNumber>, HashSet<ExtendedPoint>) ParseSchematicPart2(
        string[] lines
    )
    {
        var numbers = new HashSet<ExtendedNumber>();
        var symbols = new HashSet<ExtendedPoint>();

        for (var line = 0; line < lines.Length; ++line)
        {
            ExtendedNumber? number = null;

            for (var column = 0; column < lines[line].Length; ++column)
            {
                var currentChar = lines[line][column];

                if (char.IsDigit(currentChar))
                {
                    number ??= new ExtendedNumber()
                    {
                        Value = 0,
                        Points = new HashSet<ExtendedPoint>()
                    };

                    number.Value = number.Value * 10 + (currentChar - '0');
                    number.Points.Add(new(line, column, false));
                }
                else
                {
                    if (number is not null)
                    {
                        numbers.Add(number);
                        number = null;
                    }

                    if (currentChar != '.')
                    {
                        symbols.Add(new(line, column, currentChar == '*'));
                    }
                }
            }

            if (number is not null)
            {
                numbers.Add(number);
            }
        }

        return (numbers, symbols);
    }

    private class ExtendedNumber()
    {
        public int Value { get; set; }
        public ISet<ExtendedPoint> Points { get; set; } = null!;
    };

    private record ExtendedPoint(int Line, int Column, bool IsAsterisk)
    {
        public int? GetGearRatio(IEnumerable<ExtendedNumber> numbers)
        {
            if (!IsAsterisk)
            {
                return null;
            }

            List<ExtendedNumber> neighbors = [];

            foreach (var number in numbers)
            {
                foreach (var point in number.Points)
                {
                    if (Math.Abs(Line - point.Line) <= 1 && Math.Abs(Column - point.Column) <= 1)
                    {
                        if (neighbors.Count == 2)
                        {
                            return null;
                        }

                        neighbors.Add(number);
                        break;
                    }
                }
            }

            if (neighbors.Count != 2)
            {
                return null;
            }

            return neighbors[0].Value * neighbors[1].Value;
        }
    };
}
