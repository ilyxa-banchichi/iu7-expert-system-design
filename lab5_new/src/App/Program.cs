using LogicalConclusion.Common;
using LogicalConclusion.Common.Models;

namespace App;

internal static class Program
{
    private static void Main()
    {
        var cN = new Constant("N");
        var cM1 = new Constant("M1");
        var cW = new Constant("W");
        var cA1 = new Constant("A1");

        var vx = new Variable("x");
        var vy = new Variable("y");
        var vz = new Variable("z");
        var vx1 = new Variable("x1");
        var vx2 = new Variable("x2");
        var vx3 = new Variable("x3");

        var node1 = new Atom("A", [vx]);
        var node2 = new Atom("W", [vy]);
        var node3 = new Atom("S", [vx, vy, vz]);
        var node4 = new Atom("H", [vz]);
        var node5 = new Atom("C", [vx]);

        var node6 = new Atom("M", [vx1]);
        var node7 = new Atom("O", [cN, vx1]);
        var node8 = new Atom("S", [cW, vx1, cN]);

        var node9 = new Atom("M", [vx2]);
        var node10 = new Atom("W", [vx2]);

        var node11 = new Atom("E", [vx3, cA1]);
        var node12 = new Atom("H", [vx3]);

        var rules = new Dictionary<int, Rule>
        {
            [1] = new Rule([node1, node2, node3, node4], node5),
            [2] = new Rule([node6, node7], node8),
            [3] = new Rule([node9], node10),
            [4] = new Rule([node11], node12)
        };

        var target = new Atom("C", [cW]);

        var given = new List<Atom>
        {
            new Atom("O", [cN, cM1]),
            new Atom("M", [cM1]),
            new Atom("A", [cW]),
            new Atom("E", [cN, cA1])
        };

        Console.WriteLine("=== БАЗА ПРАВИЛ ===");
        foreach (var (num, rule) in rules)
            Console.WriteLine($"Правило {num}: {rule}");

        Console.WriteLine("ЦЕЛЬ ДОКАЗАТЕЛЬСТВА:");
        Console.WriteLine($"\t{target}");
        Console.WriteLine("ИСХОДНЫЕ ФАКТЫ:");
        foreach (var atom in given)
            Console.WriteLine($"{atom}");

        Console.WriteLine("================");

        var searcher = new HyperGraphSearcher(rules);
        var result = searcher.SearchFromTarget(given, target);
        
        if (result.HasValue)
        {
            Console.WriteLine("\nРЕШЕНИЕ НАЙДЕНО");
            foreach (var a in result.Value.newAtoms)
                Console.WriteLine(a);

            Console.WriteLine("Правила: " + string.Join(", ", result.Value.rules));
        }
        else
        {
            Console.WriteLine("\nРЕШЕНИЕ НЕ НАЙДЕНО");
        }

        Console.WriteLine("\nТаблица подстановок:");
        Console.WriteLine(searcher.Table);
    }
}