mod models;

use crate::models::{Atom, Disjunct, Term, TermKind};
use std::collections::HashMap;

struct Unification;

impl Unification {
    fn unify_atoms(a: &Atom, b: &Atom) -> Option<HashMap<String, Term>> {
        if a.name != b.name || a.args.len() != b.args.len() {
            return None;
        }

        let mut subst = HashMap::new();

        for (ta, tb) in a.args.iter().zip(b.args.iter()) {
            match (&ta.kind, &tb.kind) {
                (TermKind::Const, TermKind::Const) => {
                    if ta.value != tb.value {
                        return None;
                    }
                }
                (TermKind::Var, TermKind::Const) => {
                    subst.insert(ta.name.clone(), tb.clone());
                }
                (TermKind::Const, TermKind::Var) => {
                    subst.insert(tb.name.clone(), ta.clone());
                }
                (TermKind::Var, TermKind::Var) => {
                    if ta.name != tb.name {
                        subst.insert(ta.name.clone(), tb.clone());
                    }
                }
            }
        }

        Some(subst)
    }

    fn unify_disjunct(
        left: &Disjunct,
        right: &Disjunct,
    ) -> Option<(Disjunct, HashMap<String, Term>)> {
        let mut result: Vec<Atom> = left.args.clone();
        result.extend(right.args.clone());

        let mut global_subst = HashMap::new();
        let mut changed = true;
        let mut unufy_has_done = false;

        while changed {
            changed = false;

            'outer: for i in 0..result.len() {
                for j in i + 1..result.len() {
                    let a = &result[i];
                    let b = &result[j];

                    if a.name != b.name {
                        continue;
                    }

                    if a.sign != b.sign {
                        if let Some(sub) = Self::unify_atoms(a, b) {
                            unufy_has_done = true;
                            println!("\tПравый {}", right);
                            println!("\t\tКонтрарная пара {} {}", a, b);

                            let mut new_res = Vec::new();
                            for k in 0..result.len() {
                                if k != i && k != j {
                                    new_res.push(result[k].clone());
                                }
                            }
                            result = new_res;

                            for (k, v) in sub.iter() {
                                global_subst.insert(k.clone(), v.clone());
                            }

                            print!("\t\tРезольвента ",);
                            for atom in result.iter_mut() {
                                for t in atom.args.iter_mut() {
                                    if let Some(s) = global_subst.get(&t.name) {
                                        t.name = s.name.clone();
                                        t.kind = s.kind.clone();
                                        t.value = s.value.clone();
                                    }
                                }

                                print!("{}", atom);
                            }
                            println!("",);

                            changed = true;
                            break 'outer;
                        }
                    } else {
                        if let Some(sub) = Self::unify_atoms(a, b) {
                            if sub.is_empty() {
                                result.remove(j);
                                changed = true;
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        if !unufy_has_done {
            return None;
        }

        Some((Disjunct::new(result), global_subst))
    }
}

fn resolve(mut current: Disjunct, knowledge: &[Disjunct], limit: usize) -> Disjunct {
    let mut counter = limit;

    loop {
        let mut changed = false;
        println!("\n№ {}, Левый: {}", limit - counter + 1, current);

        for rule in knowledge {
            if counter == 0 {
                println!("Попытки закончились");
                return current;
            }

            if let Some((next, subs)) = Unification::unify_disjunct(&current, rule) {
                println!("\n{} ~~ {} => {:?}", current, rule, subs);
                current = next;
                changed = true;
                counter -= 1;
                break;
            }
        }

        if !changed {
            return current;
        }
    }
}

fn knowledge() -> Vec<Disjunct> {
    vec![
        Disjunct::parse("~P1(y1) v ~P2(x1, y1) v ~L(z1, x1)"), // 3
        Disjunct::parse("~P1(y3) v L(DT, y3)"),                // 4
        Disjunct::parse("P2(DT, BT) v P2(CT, BT)"),            // 5
        Disjunct::parse("P3(BT)"),                             // 6
        Disjunct::parse("~P3(x2) v P1(x2)"),                   // 7
        Disjunct::parse("~P4(x3) v P3(x3)"),                   // 8
        Disjunct::parse("~P4(x3) v P2(x2, x3)"),               // 9
        Disjunct::parse("P4(RT)"),                             // 10
        Disjunct::parse("~P2(CT, BT)"),                        // 11
    ]
}

fn main() {
    let kb = knowledge();

    println!("Knowledge:");
    for d in &kb {
        println!("{}", d);
    }

    let goal = Disjunct::parse("P3(BT)");

    println!("\nResolving: {}\n", goal);

    let result = resolve(goal, &kb, 1000);

    println!("\nRESULT = {}", result);
}
