namespace LogicalConclusion.Common.Models;

public class Atom
{
    public string Name { get; }
    public List<Term> Terminals { get; }
    public string Id { get; } = Guid.NewGuid().ToString()[..8];
    public bool Proven { get; set; } = false;

    public Atom(string name, IEnumerable<Term> terminals)
    {
        Name = name;
        Terminals = terminals.ToList();
    }

    public Atom CopyWithSubstitutions(Table table)
    {
        var newTerms = new List<Term>();

        foreach (var term in Terminals)
        {
            if (term is Variable v && table.Variables.ContainsKey(v.Name))
            {
                var value = table.Variables[v.Name];

                if (value is string s)
                {
                    if (table.Variables.ContainsKey(s))
                    {
                        var v2 = table.Variables[s];
                        newTerms.Add(v2 is string ? new Variable(s) : (Term)v2);
                    }
                    else
                    {
                        newTerms.Add(new Variable(s));
                    }
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