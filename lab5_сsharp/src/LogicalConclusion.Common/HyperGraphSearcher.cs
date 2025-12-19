using LogicalConclusion.Common.Models;

namespace LogicalConclusion.Common;

public record SearchResult(bool Found, List<Atom> NewAtoms, List<int> Rules);

public class HyperGraphSearcher
{
    private readonly Dictionary<int, Rule> _rules;
    private readonly List<int> _provenRules = new();
    private readonly HashSet<string> _usedAtomsCache = new();
    
    private readonly ProofTree _proofTree = new();
    
    public Table Table { get; private set; } = new();
    private List<Atom> _provenAtoms = new();
    private List<Atom> _openedAtoms = new();
    private Atom _target;
    
    private bool _found = false;

    public HyperGraphSearcher(Dictionary<int, Rule> rules)
    {
        _rules = rules ?? throw new ArgumentNullException(nameof(rules));
    }

    public SearchResult? SearchFromTarget(List<Atom> inputAtoms, Atom target)
    {
        InitializeSearch(inputAtoms, target);
        var goalId = _proofTree.Add("goal", target.ToString());
        _proofTree.CurrentNodeId = goalId;

        while (ShouldContinueSearch())
        {
            var currentGoal = _openedAtoms[0];
            Console.WriteLine($"\nТекущая подцель: {currentGoal}");
            Console.Write($"Стэк: \n\t");
            foreach (var atom in _openedAtoms)
                Console.Write($"{atom} ");
            Console.WriteLine($"\n");
            
            var treeNode = _proofTree.Tree.FirstOrDefault(n =>
                (n.Type == "subgoal" || n.Type == "goal") &&
                n.Content == currentGoal.ToString());
            if (treeNode != null)
            {
                treeNode.Proccessed = true;
                _proofTree.CurrentNodeId = treeNode.Id;
            }
            
            if (TryProveFromExistingFacts(currentGoal))
                continue;

            if (!TryApplyRuleToGoal(currentGoal))
            {
                _usedAtomsCache.Add(currentGoal.ToString());
                _openedAtoms.Remove(currentGoal);
            }
        }

        File.WriteAllText("proof_tree.dot", _proofTree.ToDot());
        return _found ? CreateResult(inputAtoms, target) : null;
    }

    private void InitializeSearch(List<Atom> inputAtoms, Atom target)
    {
        Table = new Table();
        _provenAtoms = new List<Atom>(inputAtoms);
        _openedAtoms = new List<Atom> { target };
        _usedAtomsCache.Clear();
        _provenRules.Clear();
        _found = false;
        _target = target;
        
        Console.WriteLine($"Цель: {target}");
        Console.WriteLine("Исходные факты:");
        foreach (var atom in inputAtoms)
            Console.WriteLine($"\t{atom}");
        
        Console.WriteLine("Правила: ");
        foreach (var (num, rule) in _rules)
            Console.WriteLine($"\t{num}: {rule}");
        
        Console.WriteLine($"_____________________");
    }

    private bool ShouldContinueSearch()
    {
        return !_found && _openedAtoms.Count > 0;
    }

    private bool TryProveFromExistingFacts(Atom goal)
    {
        var goalWithSubstitutions = goal.Substitutions(Table);
        
        foreach (var provenAtom in _provenAtoms)
        {
            var tempTable = new Table();
            tempTable.Reset(Table);

            if (Unifier.Unification(tempTable, goalWithSubstitutions, provenAtom))
            {
                Console.WriteLine($"\tПодцель {goal} доказана");
                Table = tempTable;
                _openedAtoms.Remove(goal);
                return true;
            }
        }
        
        return false;
    }

    private bool TryApplyRuleToGoal(Atom goal)
    {
        foreach (var (ruleId, rule) in _rules)
        {
            if (!TryApplyRule(ruleId, rule, goal))
                continue;
                
            return true;
        }
        
        return false;
    }

    private bool TryApplyRule(int ruleId, Rule rule, Atom goal)
    {
        var tempTable = Table.Clone();
        if (!Unifier.Unification(tempTable, rule.OutputVertex, goal))
            return false;

        Console.WriteLine($"\tНашли правило {ruleId}: {rule}");
        Table = tempTable;
        
        var currentGoalWithSubs = goal.Substitutions(Table);
        Console.WriteLine($"\t\tПосле подстановок: {currentGoalWithSubs}");

        _proofTree.CurrentRuleId = _proofTree.Add("rule", rule.ToString(), _proofTree.CurrentNodeId, Table);

        var allInputsProven = ProcessRuleInputs(rule);
        
        if (allInputsProven)
            CompleteRuleApplication(ruleId, currentGoalWithSubs, goal);
        
        return true;
    }

    private bool ProcessRuleInputs(Rule rule)
    {
        bool result = true;
        foreach (var inputAtom in rule.InputAtoms)
        {
            var atomWithSubs = inputAtom.Substitutions(Table);
            _proofTree.SubgoalId = _proofTree.Add("subgoal", atomWithSubs.ToString(), _proofTree.CurrentRuleId);
            
            if (IsAtomAlreadyProven(atomWithSubs))
                continue;

            if (IsAtomAlreadyUsed(atomWithSubs))
                return false;

            AddAtomToOpenList(atomWithSubs);
            result = false;
            // return false;
        }
        
        return result;
    }

    private bool IsAtomAlreadyProven(Atom atom)
    {
        foreach (var provenAtom in _provenAtoms)
        {
            var tempTable = Table.Clone();
            if (Unifier.Unification(tempTable, atom, provenAtom))
            {
                _proofTree.Add("fact", provenAtom.ToString(), _proofTree.SubgoalId);
                Console.WriteLine($"\t\tНашли факт: {provenAtom}");
                Table = tempTable;
                return true;
            }
        }
        
        return false;
    }

    private bool IsAtomAlreadyUsed(Atom atom)
    {
        return _usedAtomsCache.Contains(atom.ToString());
    }

    private void AddAtomToOpenList(Atom atom)
    {
        Console.WriteLine($"\t\tАтом {atom} не найден в фактах, добавляем как подцель");
        _usedAtomsCache.Add(atom.ToString());
        _openedAtoms.Insert(0, atom);
    }

    private void CompleteRuleApplication(int ruleId, Atom provenAtom, Atom originalGoal)
    {
        Console.WriteLine($"\tПправило {ruleId} доказано, новый факт {provenAtom}");
        
        _provenAtoms.Add(provenAtom);
        _provenRules.Add(ruleId);
        _openedAtoms.Remove(originalGoal);

        CheckIfTargetReached();
    }

    private void CheckIfTargetReached()
    {
        foreach (var provenAtom in _provenAtoms)
        {
            var tempTable = Table.Clone();
            if (Unifier.Unification(tempTable, _target, provenAtom))
            {
                Console.WriteLine($"Цель {_target} достигнута!");
                _found = true;
                return;
            }
        }
    }

    private SearchResult CreateResult(List<Atom> inputAtoms, Atom target)
    {
        var newAtoms = _provenAtoms
            .Where(atom => inputAtoms.All(input => input.ToString() != atom.ToString()))
            .ToList();
            
        return new SearchResult(_found, newAtoms, _provenRules);
    }
}