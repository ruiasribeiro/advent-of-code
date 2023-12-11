namespace Aoc2023.Days.Day07;

public class Day07Solver : ISolver
{
    public static (object, object) Solve()
    {
        var lines = File.ReadAllLines("Days/Day07/input.txt");

        var part1 = Solve<Hand>(lines);
        var part2 = Solve<HandWithJoker>(lines);

        return (part1, part2);
    }

    private static long Solve<T>(string[] lines)
        where T : IHand, new()
    {
        SortedSet<T> hands = [];

        foreach (var line in lines)
        {
            var info = line.Split(
                ' ',
                StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries
            );

            hands.Add(new() { Cards = info[0], Bid = long.Parse(info[1]) });
        }

        var handsList = hands.ToList();
        var total = 0L;

        for (var i = 0; i < handsList.Count; ++i)
        {
            total += handsList[i].Bid * (i + 1);
        }

        return total;
    }

    interface IHand
    {
        public string Cards { get; set; }
        public long Bid { get; set; }

        public int CalculateScore();

        public static abstract int GetCardStrength(char card);
    }

    class Hand : IComparable<Hand>, IHand
    {
        public string Cards { get; set; } = null!;
        public long Bid { get; set; }

        public int CompareTo(Hand? other)
        {
            if (other is null)
            {
                return 1;
            }

            var score = CalculateScore();
            var otherScore = other.CalculateScore();

            if (score != otherScore)
            {
                return score - otherScore;
            }

            for (var i = 0; i <= Cards.Length; ++i)
            {
                var cardStrength = GetCardStrength(Cards[i]);
                var otherCardStrength = GetCardStrength(other.Cards[i]);

                if (cardStrength != otherCardStrength)
                {
                    return cardStrength - otherCardStrength;
                }
            }

            return 0;
        }

        public int CalculateScore()
        {
            var counts = Cards.GroupBy(c => c).Select(elem => elem.Count()).ToList();

            return counts switch
            {
                _ when counts.Contains(5) => 7,
                _ when counts.Contains(4) => 6,
                _ when counts.Contains(3) && counts.Contains(2) => 5,
                _ when counts.Contains(3) => 4,
                _ when counts.Where(count => count == 2).Count() == 2 => 3,
                _ when counts.Contains(2) => 2,
                _ => 1,
            };
        }

        public static int GetCardStrength(char card) =>
            card switch
            {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' or '8' or '7' or '6' or '5' or '4' or '3' or '2' => card - '0',
                _ => throw new NotImplementedException()
            };
    }

    class HandWithJoker : IComparable<HandWithJoker>, IHand
    {
        public string Cards { get; set; } = null!;
        public long Bid { get; set; }

        public int CompareTo(HandWithJoker? other)
        {
            if (other is null)
            {
                return 1;
            }

            var score = CalculateScore();
            var otherScore = other.CalculateScore();

            if (score != otherScore)
            {
                return score - otherScore;
            }

            for (var i = 0; i <= Cards.Length; ++i)
            {
                var cardStrength = GetCardStrength(Cards[i]);
                var otherCardStrength = GetCardStrength(other.Cards[i]);

                if (cardStrength != otherCardStrength)
                {
                    return cardStrength - otherCardStrength;
                }
            }

            return 0;
        }

        public int CalculateScore()
        {
            var counts = Cards
                .GroupBy(c => c)
                .Select(elem => new { IsJoker = elem.First() == 'J', Count = elem.Count() })
                .ToList();

            var joker = counts.FirstOrDefault(count => count.IsJoker);
            var jokerCount = joker is null ? 0 : joker.Count;

            var otherCounts = counts
                .Where(count => !count.IsJoker)
                .Select(count => count.Count)
                .ToList();

            var maxCount = 0;
            try
            {
                maxCount = otherCounts.Max();
                otherCounts.Remove(maxCount);
            }
            catch (InvalidOperationException) { }

            var maxCountWithJoker = maxCount + jokerCount;

            return maxCountWithJoker switch
            {
                5 => 7,
                4 => 6,
                3 when otherCounts.Contains(2) => 5,
                3 => 4,
                2 when otherCounts.Contains(2) => 3,
                2 => 2,
                _ => 1,
            };
        }

        public static int GetCardStrength(char card) =>
            card switch
            {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 11,
                '9' or '8' or '7' or '6' or '5' or '4' or '3' or '2' => card - '0' + 1,
                'J' => 1,
                _ => throw new NotImplementedException()
            };
    }
}
