namespace LogicalConclusion.Common.Models;

public class Constant : Term
{
    public string Value { get; }

    public Constant(string value)
    {
        Value = value;
    }

    public override bool IsVariable => false;

    public override string ToString()
    {
        return $"{Value}:const";
    }
}