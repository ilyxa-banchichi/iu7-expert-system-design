namespace LogicalConclusion.Common.Models;

public class Rule
{
    public List<Atom> InputAtoms { get; }
    public Atom OutputVertex { get; }
    public string Id { get; } = Guid.NewGuid().ToString()[..8];

    public Rule(IEnumerable<Atom> inputs, Atom output)
    {
        InputAtoms = new List<Atom>(inputs);
        OutputVertex = output;
    }

    public override string ToString()
    {
        return $"{string.Join(" & ", InputAtoms)} -> {OutputVertex}";
    }
}