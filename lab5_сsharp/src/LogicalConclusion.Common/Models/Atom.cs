namespace LogicalConclusion.Common.Models;

public class Atom
{
    public string Name { get; }
    public List<Term> Terminals { get; }

    public Atom(string name, IEnumerable<Term> terminals)
    {
        Name = name;
        Terminals = terminals.ToList();
    }

    public Atom Substitutions(Table table)
    {
        var newTerms = new List<Term>();

        foreach (var term in Terminals)
        {
            if (term is Variable v && table.Variables.TryGetValue(v.Name, out var value))
            {
                if (value is string s)
                {
                    if (table.Variables.TryGetValue(s, out var v2))
                        newTerms.Add(v2 is string ? new Variable(s) : (Term)v2);
                    else
                        newTerms.Add(new Variable(s));
                }
                else
                {
                    newTerms.Add((Term)value);
                }
            }
            else
            {
                newTerms.Add(term);
            }
        }

        return new Atom(Name, newTerms);
    }

    public override string ToString()
    {
        return $"{Name}({string.Join(", ", Terminals)})";
    }
}