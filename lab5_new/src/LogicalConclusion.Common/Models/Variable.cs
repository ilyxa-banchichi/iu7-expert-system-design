namespace LogicalConclusion.Common.Models;

public class Variable : Term
{
    public string Name { get; }

    public Variable(string name)
    {
        Name = name;
    }

    public override bool IsVariable => true;

    public override string ToString()
    {
        return $"{Name}:var";
    }
}