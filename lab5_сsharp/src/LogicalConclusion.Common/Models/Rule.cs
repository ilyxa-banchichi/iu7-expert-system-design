namespace LogicalConclusion.Common.Models;

public class Rule
{
    public List<Atom> InputAtoms { get; }
    public Atom OutputVertex { get; }
    public bool IsProven { get; set; } = false;

    public Rule(IEnumerable<Atom> inputs, Atom output)
    {
        InputAtoms = new List<Atom>(inputs);
        OutputVertex = output;
    }

    public string ToStringWithSubstitutions(Table table)
    {
        var inputs = InputAtoms.Select(atom => atom.Substitutions(table)).ToList();
        return $"{string.Join(" & ", inputs)} -> {OutputVertex.Substitutions(table)}";
    }

    public override string ToString()
    {
        return $"{string.Join(" & ", InputAtoms)} -> {OutputVertex}";
    }
}