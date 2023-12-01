using System.Text.RegularExpressions;

namespace Aoc2023.Days.Day01;

public partial class Day01Solver : ISolver
{
    public static (object, object) Solve()
    {
        var lines = File.ReadAllLines("Days/Day01/input.txt");

        // Part 1.

        var part1 = 0;

        foreach (var line in lines)
        {
            var first = line.FirstOrDefault(c => char.IsAsciiDigit(c));
            var last = line.LastOrDefault(c => char.IsAsciiDigit(c));

            part1 += int.Parse($"{first}{last}");
        }

        // Part 2.

        var part2 = 0;
        var regex = DigitRegex();

        foreach (var line in lines)
        {
            var matches = regex.Matches(line);

            var firstMatch = GetDigitMatch(matches.First());
            var lastMatch = GetDigitMatch(matches.Last());

            var first = ConvertStringDigitToCharDigit(firstMatch);
            var last = ConvertStringDigitToCharDigit(lastMatch);

            part2 += int.Parse($"{first}{last}");
        }

        return (part1, part2);
    }

    // This regular expression uses lookahead (?=) to ensure strings like
    // "oneight" are detected as "18".
    [GeneratedRegex(
        @"(?=(zero|one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9))",
        RegexOptions.IgnoreCase | RegexOptions.Compiled,
        "en-GB"
    )]
    private static partial Regex DigitRegex();

    // I don't understand enough of regular expressions to explain this
    // correctly, but, from what I've gathered, the usage of lookahead will
    // result on the matching of empty strings, as lookahead does not consume
    // anything. To really read the value that matched, we need to perform the
    // following workaround.
    private static string GetDigitMatch(Match match) => match.Groups.Values.Last().Value;

    private static char ConvertStringDigitToCharDigit(string input)
    {
        return input switch
        {
            "zero" => '0',
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            "0" or "1" or "2" or "3" or "4" or "5" or "6" or "7" or "8" or "9" => input.First(),
            _
                => throw new ArgumentException(
                    $"unexpected value for input: '{input}' is not a digit"
                )
        };
    }
}
