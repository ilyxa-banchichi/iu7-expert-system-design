using LogicalConclusion.Common.Models;

namespace LogicalConclusion.Common;

public class HyperGraphSearcher
{
    public Table Table { get; private set; } = new();
    
    private readonly List<int> _provenRules = [];
    private readonly List<Atom> _usedAtoms = [];
    private readonly Dictionary<int, Rule> _rules;

    private List<Atom> _provenAtoms = [];
    private List<Atom> _openedAtoms = [];

    private bool _found = false;

    public HyperGraphSearcher(Dictionary<int, Rule> rules)
    {
        _rules = rules;
    }

    public (bool found, List<Atom> newAtoms, List<int> rules)? SearchFromTarget(
        List<Atom> inputAtoms, Atom target)
    {
        Table = new Table();
        _provenAtoms = new List<Atom>(inputAtoms);
        _openedAtoms = [target];
        _usedAtoms.Clear();
        _found = false;

        Console.WriteLine($"Начинаем поиск доказательства для {target}");

        Atom current = _openedAtoms[0];

        while (!_found && _openedAtoms.Any() && current != null)
        {
            Console.WriteLine($"\nТекущая подцель: {current}");

            // ПРОВЕРКА 1: Уже доказан? (используем таблицу подстановок)
            var currentSub = current.CopyWithSubstitutions(Table);
            foreach (var proven in _provenAtoms)
            {
                var temp = new Table();
                temp.Reset(Table);

                if (Unifier.Unification(temp, currentSub, proven))
                {
                    Console.WriteLine($"Подцель {current} уже доказана как {proven}");
                    // Применяем подстановки, если нашли совпадение
                    Table = temp;
                    // Удаляем из открытых
                    _openedAtoms.Remove(current);
                    current = _openedAtoms.FirstOrDefault();
                }
            }

            // Проходимся по всем правилам
            bool applied = false;
            foreach (var (num, rule) in _rules)
            {
                var temp = new Table();
                temp.Reset(Table);

                // Унифицируем текущую подцель с выходным атомом правила
                if (!Unifier.Unification(temp, rule.OutputVertex, current))
                    continue;

                Console.WriteLine($"Нашли правило {num}: {rule}");
                applied = true;
                
                // НЕМЕДЛЕННО применяем найденные подстановки
                Table = temp;
                
                // Обновляем текущую подцель с подстановками
                current = current.CopyWithSubstitutions(Table);
                Console.WriteLine($"После подстановок: {current}");

                bool allProven = true;
                
                // Проверяем все входные атомы правила
                foreach (var atom in rule.InputAtoms)
                {
                    // Сразу применяем текущие подстановки к атому правила
                    var atomSub = atom.CopyWithSubstitutions(Table);
                    Console.WriteLine($"Проверяем атом (после подстановок): {atomSub}");

                    // Пытаемся доказать атом
                    bool proven = false;

                    // ПРОВЕРКА 2: Уже доказан этот атом?
                    foreach (var fact in _provenAtoms)
                    {
                        var temp2 = new Table();
                        temp2.Reset(Table);
                        if (Unifier.Unification(temp2, atomSub, fact))
                        {
                            Console.WriteLine($"Нашли факт: {fact}");
                            
                            // НЕМЕДЛЕННО применяем новые подстановки
                            Table = temp2;
                            
                            // Обновляем node_with_subs с новыми подстановками
                            atomSub = atom.CopyWithSubstitutions(Table);
                            
                            proven = true;
                            break;
                        }
                    }

                    // Если не нашли в фактах, проверяем в used_atoms чтобы избежать циклов
                    if (!proven)
                    {
                        // ПРОВЕРКА 3: Уже проверяли этот атом? (предотвращение циклов)
                        if (_usedAtoms.Any(a => a.ToString() == atom.ToString()))
                        {
                            Console.WriteLine($"Атом {atom} уже был использован, пропускаем");
                            continue;
                        }

                        // Добавляем в used_atoms
                        _usedAtoms.Add(atom);

                        Console.WriteLine($"Атом {atomSub} не найден в фактах, добавляем как подцель");
                        allProven = false;
                        
                        // Важно: добавляем node_with_subs (с подстановками) а не оригинал
                        _openedAtoms.Insert(0, atomSub);
                        
                        // Меняем текущую подцель
                        current = atomSub;
                        break;
                    }
                }

                // Если все атомы правила доказаны
                if (allProven)
                {
                    Console.WriteLine($"Все атомы правила {num} доказаны!");
                        
                    // Получаем доказанный атом с текущими подстановками
                    var provenAtom = current.CopyWithSubstitutions(Table);

                    Console.WriteLine($"Выводим новый факт: {provenAtom}");
                    
                    _provenAtoms.Add(provenAtom);
                    _provenRules.Add(num);
                    _openedAtoms.Remove(current);

                    // Проверяем, достигли ли цели (с учетом подстановок)
                    foreach (var p in _provenAtoms)
                    {
                        var tempCheck = new Table();
                        tempCheck.Reset(Table);
                        if (Unifier.Unification(tempCheck, target, p))
                        {
                            Console.WriteLine($"ЦЕЛЬ {target} ДОСТИГНУТА!");
                            _found = true;
                            break;
                        }
                    }
                    
                    // Берем следующую подцель если есть
                    if (_openedAtoms.Count > 0 && !_found)
                        current = _openedAtoms[0];
                }

                break;
            }

            if (!applied)
            {
                _usedAtoms.Add(current);
                _openedAtoms.Remove(current);
                current = _openedAtoms.FirstOrDefault();
            }
        }

        if (!_found) return null;

        var newAtoms = _provenAtoms
            .Where(a => inputAtoms.All(i => i.ToString() != a.ToString()))
            .ToList();

        return (true, newAtoms, _provenRules);
    }
}