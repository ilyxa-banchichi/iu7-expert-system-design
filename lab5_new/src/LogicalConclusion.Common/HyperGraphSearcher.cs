using System;
using System.Collections.Generic;
using System.Linq;
using LogicalConclusion.Common.Models;

namespace LogicalConclusion.Common;

public class HyperGraphSearcher
{
    private Dictionary<int, Rule> Rules;
    public Table Table { get; private set; } = new();

    private List<Atom> ProvenAtoms = new();
    private List<int> ProvenRules = new();
    private List<Atom> OpenedAtoms = new();
    private List<Atom> UsedAtoms = new();

    private bool Found = false;

    public HyperGraphSearcher(Dictionary<int, Rule> rules)
    {
        Rules = rules;
    }

    public (bool found, List<Atom> newAtoms, List<int> rules)? SearchFromTarget(
        List<Atom> inputAtoms, Atom target)
    {
        Table = new Table();
        ProvenAtoms = new List<Atom>(inputAtoms);
        OpenedAtoms = new List<Atom> { target };
        UsedAtoms.Clear();
        Found = false;

        Console.WriteLine($"Начинаем поиск доказательства для {target}");

        Atom current = OpenedAtoms[0];

        while (!Found && OpenedAtoms.Any())
        {
            Console.WriteLine($"\nТекущая подцель: {current}");

            // ПРОВЕРКА 1: Уже доказан? (используем таблицу подстановок)
            var currentSub = current.CopyWithSubstitutions(Table);
            foreach (var proven in ProvenAtoms)
            {
                var temp = new Table();
                temp.Reset(Table);

                if (Unifier.Unification(temp, currentSub, proven))
                {
                    Console.WriteLine($"Подцель {current} уже доказана как {proven}");
                    // Применяем подстановки, если нашли совпадение
                    Table = temp;
                    OpenedAtoms.Remove(current);
                    // Удаляем из открытых
                    current = OpenedAtoms.FirstOrDefault();

                    // Переходим к следующей
                    // if (OpenedAtoms.Count > 0)
                    // {
                    //     current = OpenedAtoms[0];
                    //     // Ищем узел в дереве
                    //     for node in :
                    //     if (node['type'] == 'subgoal' and
                    //     str(node['content']) == str(current)):
                    //     current_node_id = node['id']
                    //     break
                    // }
                }
            }

            // Проходимся по всем правилам
            bool applied = false;
            foreach (var (num, rule) in Rules)
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
                    foreach (var fact in ProvenAtoms)
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
                        if (UsedAtoms.Any(a => a.ToString() == atom.ToString()))
                        {
                            Console.WriteLine($"Атом {atom} уже был использован, пропускаем");
                            continue;
                        }

                        // Добавляем в used_atoms
                        UsedAtoms.Add(atom);

                        Console.WriteLine($"Атом {atomSub} не найден в фактах, добавляем как подцель");
                        allProven = false;
                        
                        // Важно: добавляем node_with_subs (с подстановками) а не оригинал
                        OpenedAtoms.Insert(0, atomSub);
                        
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
                    
                    ProvenAtoms.Add(provenAtom);
                    ProvenRules.Add(num);
                    OpenedAtoms.Remove(current);

                    // Проверяем, достигли ли цели (с учетом подстановок)
                    foreach (var p in ProvenAtoms)
                    {
                        var tempCheck = new Table();
                        tempCheck.Reset(Table);
                        if (Unifier.Unification(tempCheck, target, p))
                        {
                            Console.WriteLine($"ЦЕЛЬ {target} ДОСТИГНУТА!");
                            Found = true;
                            break;
                        }
                    }
                    
                    // Берем следующую подцель если есть
                    if (OpenedAtoms.Count > 0 && !Found)
                        current = OpenedAtoms[0];
                }

                break;
            }

            if (!applied)
            {
                UsedAtoms.Add(current);
                OpenedAtoms.Remove(current);
                current = OpenedAtoms.FirstOrDefault();
            }
        }

        if (!Found) return null;

        var newAtoms = ProvenAtoms
            .Where(a => !inputAtoms.Any(i => i.ToString() == a.ToString()))
            .ToList();

        return (true, newAtoms, ProvenRules);
    }
}