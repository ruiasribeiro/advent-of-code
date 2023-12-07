using System.Security.Cryptography;

namespace Aoc2023.Days.Day04;

public class Day04Solver : ISolver
{
    public static (object, object) Solve()
    {
        var lines = File.ReadAllLines("Days/Day04/input.txt");

        var part1 = SolvePart1(lines);
        var part2 = SolvePart2(lines);

        return (part1, part2);
    }

    private static int SolvePart1(string[] lines)
    {
        var cards = ParseCards(lines);

        var sum = 0;

        foreach (var card in cards)
        {
            var matchingCards = card.WinningNumbers.Intersect(card.DrawnNumbers).Count();
            sum += (int)Math.Pow(2, matchingCards - 1);
        }

        return sum;
    }

    private static List<Card> ParseCards(string[] lines)
    {
        List<Card> cards = [];

        foreach (var line in lines)
        {
            var cardContents = line.Split(':');
            var numberSets = cardContents[1].Split('|');

            cards.Add(
                new(
                    numberSets[0]
                        .Split(' ')
                        .Select(number =>
                        {
                            _ = int.TryParse(number.Trim(), out var result);
                            return result;
                        })
                        .Except([0])
                        .ToHashSet(),
                    numberSets[1]
                        .Split(' ')
                        .Select(number =>
                        {
                            _ = int.TryParse(number.Trim(), out var result);
                            return result;
                        })
                        .Except([0])
                        .ToHashSet()
                )
            );
        }

        return cards;
    }

    private record Card(ISet<int> WinningNumbers, ISet<int> DrawnNumbers);

    private static int SolvePart2(string[] lines)
    {
        var cards = ParseCards(lines);

        var sum = cards.Count;
        var cardQueue = new Queue<int>(Enumerable.Range(0, cards.Count));

        while (cardQueue.Count != 0)
        {
            var cardId = cardQueue.Dequeue();
            var card = cards[cardId];

            var matchingCards = card.WinningNumbers.Intersect(card.DrawnNumbers).Count();
            sum += matchingCards;

            for (var id = cardId + 1; id <= cardId + matchingCards; ++id)
            {
                cardQueue.Enqueue(id);
            }
        }

        return sum;
    }
}
