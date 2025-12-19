using System.Text;
using LogicalConclusion.Common.Models;

namespace LogicalConclusion.Common;

public class ProofTree
{
    public class Node
    {
        public string Id;
        public string Type;
        public string Content;
        public string? ParentId;
        public Dictionary<string, object>? Substitutions;
        public List<string> Children;
        public bool Proven;
        public bool Proccessed;
    }

    public List<Node> Tree = [];
    
    public string CurrentNodeId;
    public string CurrentRuleId;
    public string SubgoalId;

    public string Add(string nodeType, string content, string? parentId = null, Table substitutions = null)
    {
        var nodeId = $"node_{Tree.Count}";
        var node = new Node
        {
            Id = nodeId,
            Type = nodeType,
            Content = content,
            ParentId = parentId,
            Substitutions = substitutions?.Clone().Variables ?? null,
            Children = [],
            Proven = nodeType == "fact" || nodeType == "new_fact"
        };

        if (parentId != null)
        {
            foreach (var n in Tree)
            {
                if (n.Id == parentId)
                {
                    n.Children.Add(nodeId);
                    break;
                }
            }
        }

        Tree.Add(node);
        return nodeId;
    }
    
    public string ToDot()
    {
        var sb = new StringBuilder();
        sb.AppendLine("digraph ProofTree {");
        sb.AppendLine("rankdir=TB;");
        sb.AppendLine("node [fontname=\"Consolas\"];");

        foreach (var node in Tree)
        {
            var shape = node.Type switch
            {
                "goal" => "oval",
                "rule" => "box",
                "subgoal" => "ellipse",
                "fact" => "diamond",
                "new_fact" => "diamond",
                _ => "ellipse"
            };

            var color = node.Proven ? "green" :
                node.Proccessed ? "red" :
                "black";
            if (node.Type == "new_fact")
                color = "blue";

            var label = $"{node.Type}\\n{node.Content}";
            sb.AppendLine(
                $"{node.Id} [label=\"{label}\", shape={shape}, color={color}];"
            );
        }

        foreach (var node in Tree)
        {
            foreach (var child in node.Children)
            {
                sb.AppendLine($"{node.Id} -> {child};");
            }
        }

        sb.AppendLine("}");
        return sb.ToString();
    }
}