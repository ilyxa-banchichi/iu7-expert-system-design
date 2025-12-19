using LogicalConclusion.Common.Models;

namespace LogicalConclusion.Common;

public class ForwardChaining
{
    public List<Atom> Facts { get; } = [];
    private readonly Dictionary<int, Rule> _rules;

    public ForwardChaining(Dictionary<int, Rule> rules)
    {
        _rules = rules;
    }

    public bool Prove(IEnumerable<Atom> initialFacts, Atom goal)
    {
        Facts.AddRange(initialFacts);
        
        bool newFactAdded = true;

        while (newFactAdded)
        {
            newFactAdded = false;

            foreach (var (num, rule) in _rules)
            {
                if (rule.IsProven)
                    continue;

                if (!TryProveRule(rule, out var derived))
                    continue;

                if (Facts.All(f => f.ToString() != derived.ToString()))
                {
                    Facts.Add(derived);
                    newFactAdded = true;
                }

                rule.IsProven = true;

                var temp = new Table();
                if (Unifier.Unification(temp, goal, derived))
                    return true;
            }
        }

        return false;
    }
    
    private bool TryProveRule(Rule rule, out Atom derived)
    {
        derived = null!;
        var table = new Table();

        foreach (var input in rule.InputAtoms)
        {
            var inputSub = input.Substitutions(table);

            if (!TryProveAtom(inputSub, table))
                return false;
        }

        derived = rule.OutputVertex.Substitutions(table);
        return true;
    }
    
    private bool TryProveAtom(Atom atom, Table table)
    {
        foreach (var fact in Facts)
        {
            var temp = table.Clone();

            if (!Unifier.Unification(temp, atom, fact))
                continue;
            
            table.Reset(temp);
            return true;
        }

        return false;
    }
}
