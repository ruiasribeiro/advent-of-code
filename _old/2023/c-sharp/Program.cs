using System.Reflection;
using Aoc2023.Days;

// Get all the classes that implement the `ISolver` interface that are not the
// interface itself.
var solvers = Assembly
    .GetExecutingAssembly()
    .GetTypes()
    .Where(t => typeof(ISolver).IsAssignableFrom(t) && t.Name != "ISolver");

// For all those solver classes, we execute their `Solve()` method.
foreach (var solver in solvers)
{
    var method = solver.GetMethod("Solve", BindingFlags.Static | BindingFlags.Public);
    Console.WriteLine($"{solver.Name}: {method!.Invoke(null, null)}");
}
