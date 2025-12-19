using System.Text;

namespace LogicalConclusion.Common.Models;

public class Table
{
    public Dictionary<string, object> Variables { get; private set; } = new();

    public void Reset(Table other)
    {
        Variables = new Dictionary<string, object>(other.Variables);
    }

    public Table Clone()
    {
        var table = new Table();
        table.Reset(this);
        return table;
    }

    public override string ToString()
    {
        var sb = new StringBuilder();
        foreach (var kv in Variables)
            sb.AppendLine($"{kv.Key} = {kv.Value}");
        return sb.ToString();
    }
}