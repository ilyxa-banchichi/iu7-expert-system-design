using System.Linq;
using LogicalConclusion.Common.Models;

namespace LogicalConclusion.Common;

public static class Unifier
{
    public static bool Unification(Table table, Atom p1, Atom p2)
    {
        if (p1.Name != p2.Name) 
            return false;
        
        if (p1.Terminals.Count != p2.Terminals.Count) 
            return false;

        var original = new Table();
        original.Reset(table);

        for (int i = 0; i < p1.Terminals.Count; i++)
        {
            var t1 = p1.Terminals[i];
            var t2 = p2.Terminals[i];

            if (t1.IsVariable)
            {
                var v1 = (Variable)t1;
                if (t2.IsVariable)
                {
                    var v2 = (Variable)t2;
                    if (!table.Variables.ContainsKey(v1.Name) && !table.Variables.ContainsKey(v2.Name))
                    {
                        table.Variables[v1.Name] = v2.Name;
                        table.Variables[v2.Name] = v1.Name;
                    }
                    else if (!table.Variables.ContainsKey(v1.Name))
                    {
                        table.Variables[v1.Name] = table.Variables[v2.Name];
                    }
                    else if (!table.Variables.TryGetValue(v2.Name, out var variable))
                    {
                        table.Variables[v2.Name] = table.Variables[v1.Name];
                    }
                    else if (!Equals(table.Variables[v1.Name], variable))
                    {
                        table.Reset(original);
                        return false;
                    }
                }
                else
                {
                    table.Variables.TryAdd(v1.Name, t2);

                    if (table.Variables[v1.Name] is string k)
                    {
                        table.Variables[v1.Name] = t2;
                        table.Variables[k] = t2;
                    }
                }
            }
            else
            {
                if (t2.IsVariable)
                {
                    var v2 = (Variable)t2;
                    table.Variables.TryAdd(v2.Name, t1);

                    if (table.Variables[v2.Name] is string k)
                    {
                        table.Variables[v2.Name] = t1;
                        table.Variables[k] = t1;
                    }
                }
                else
                {
                    if (((Constant)t1).Value != ((Constant)t2).Value)
                    {
                        table.Reset(original);
                        return false;
                    }
                }
            }
        }

        return true;
    }
}