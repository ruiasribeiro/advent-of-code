using static Aoc2023.Days.Day10.Day10Solver;

namespace Aoc2023.Days.Day10;

public partial class Day10Solver : ISolver
{
    public static (object, object) Solve()
    {
        var lines = File.ReadAllLines("Days/Day10/input.txt");

        var part1 = SolvePart1(lines);
        var part2 = SolvePart2(lines);

        return (part1, part2);
    }

    private static long SolvePart1(string[] lines)
    {
        var grid = ParseLines(lines);

        RemoveUnconnectedPipes(grid);

        var startPosition = GetStartPosition(grid);

        var visited = TraversePipes(grid, startPosition);

        return visited.MaxBy(elem => elem.Value).Value;
    }

    private static Position GetStartPosition(List<List<TileType>> grid)
    {
        Position? startPosition = null;
        for (var i = 0; i < grid.Count; ++i)
        {
            for (var j = 0; j < grid.First().Count; ++j)
            {
                if (grid[i][j] == TileType.Start)
                {
                    startPosition = new(i, j);
                    break;
                }
            }

            if (startPosition is not null)
            {
                break;
            }
        }

        if (startPosition is null)
        {
            throw new Exception();
        }

        return startPosition;
    }

    private static long SolvePart2(string[] lines)
    {
        return 0;
    }

    static List<List<TileType>> ParseLines(string[] lines) =>
        lines.Select(line => line.Select(ToTileType).ToList()).ToList();

    static void RemoveUnconnectedPipes(List<List<TileType>> grid)
    {
        var changed = false;

        do
        {
            changed = false;

            for (var i = 0; i < grid.Count; ++i)
            {
                for (var j = 0; j < grid.First().Count; ++j)
                {
                    var type = grid[i][j];

                    if (type == TileType.Ground)
                    {
                        continue;
                    }

                    var neighbors = GetNeighbors(grid, new(i, j));

                    var numConnections = neighbors
                        .Where(n => Connected(new(i, j), type, n, grid[n.Line][n.Column]))
                        .Count();

                    if (numConnections < 2 && grid[i][j] != TileType.Ground)
                    {
                        changed = true;
                        grid[i][j] = TileType.Ground;
                    }
                }
            }
        } while (changed);
    }

    private static Dictionary<Position, int> TraversePipes(
        List<List<TileType>> grid,
        Position startPosition
    )
    {
        Queue<Pipe> toVisit = [];
        toVisit.Enqueue(new(startPosition, 0));

        Dictionary<Position, int> visited = [];

        while (toVisit.Count != 0)
        {
            var pipe = toVisit.Dequeue();

            var adjacentPipes = GetNeighbors(grid, pipe.Position)
                .Where(
                    n =>
                        Connected(
                            pipe.Position,
                            grid[pipe.Position.Line][pipe.Position.Column],
                            n,
                            grid[n.Line][n.Column]
                        )
                )
                .ToList();

            foreach (var adjacent in adjacentPipes)
            {
                if (!visited.ContainsKey(adjacent))
                {
                    toVisit.Enqueue(new(adjacent, pipe.Distance + 1));
                }
            }

            if (!visited.ContainsKey(pipe.Position))
            {
                visited.Add(pipe.Position, pipe.Distance);
            }
        }

        return visited;
    }

    static List<Position> GetNeighbors(List<List<TileType>> grid, Position position)
    {
        List<Position> neighbors = [];

        var i = position.Line;
        var j = position.Column;

        if (i + 1 < grid.Count)
        {
            neighbors.Add(new(i + 1, j));
        }

        if (i - 1 >= 0)
        {
            neighbors.Add(new(i - 1, j));
        }

        if (j + 1 < grid.First().Count)
        {
            neighbors.Add(new(i, j + 1));
        }

        if (j - 1 >= 0)
        {
            neighbors.Add(new(i, j - 1));
        }

        return neighbors;
    }

    static void PrintGrid(List<List<TileType>> grid)
    {
        foreach (var line in grid)
        {
            foreach (var item in line)
            {
                Console.Write(
                    item switch
                    {
                        TileType.Vertical => '|',
                        TileType.Horizontal => '-',
                        TileType.NorthEast => 'L',
                        TileType.NorthWest => 'J',
                        TileType.SouthWest => '7',
                        TileType.SouthEast => 'F',
                        TileType.Ground => '.',
                        TileType.Start => 'S',
                        _ => ' '
                    }
                );
            }
            Console.WriteLine();
        }
    }

    record Pipe(Position Position, int Distance);

    record Position(int Line, int Column);

    public enum TileType
    {
        Vertical,
        Horizontal,
        NorthEast,
        NorthWest,
        SouthWest,
        SouthEast,
        Ground,
        Start,
    }

    static TileType ToTileType(char c)
    {
        return c switch
        {
            '|' => TileType.Vertical,
            '-' => TileType.Horizontal,
            'L' => TileType.NorthEast,
            'J' => TileType.NorthWest,
            '7' => TileType.SouthWest,
            'F' => TileType.SouthEast,
            '.' => TileType.Ground,
            'S' => TileType.Start,
            _ => throw new NotImplementedException(),
        };
    }

    static bool Connected(Position aPosition, TileType aType, Position bPosition, TileType bType)
    {
        var aConnections = GetAllowedConnectionPositions(aPosition, aType);
        var bConnections = GetAllowedConnectionPositions(bPosition, bType);

        return aConnections.Contains(bPosition) && bConnections.Contains(aPosition);
    }

    static HashSet<Position> GetAllowedConnectionPositions(Position position, TileType type) =>
        type switch
        {
            TileType.Vertical
                =>
                [
                    new(position.Line - 1, position.Column),
                    new(position.Line + 1, position.Column)
                ],
            TileType.Horizontal
                =>
                [
                    new(position.Line, position.Column - 1),
                    new(position.Line, position.Column + 1)
                ],
            TileType.NorthEast
                =>
                [
                    new(position.Line - 1, position.Column),
                    new(position.Line, position.Column + 1)
                ],
            TileType.NorthWest
                =>
                [
                    new(position.Line - 1, position.Column),
                    new(position.Line, position.Column - 1)
                ],
            TileType.SouthWest
                =>
                [
                    new(position.Line + 1, position.Column),
                    new(position.Line, position.Column - 1)
                ],
            TileType.SouthEast
                =>
                [
                    new(position.Line + 1, position.Column),
                    new(position.Line, position.Column + 1)
                ],
            TileType.Ground => [],
            TileType.Start
                =>
                [
                    new(position.Line + 1, position.Column),
                    new(position.Line - 1, position.Column),
                    new(position.Line, position.Column + 1),
                    new(position.Line, position.Column - 1)
                ],
            _ => throw new NotImplementedException(),
        };
}
