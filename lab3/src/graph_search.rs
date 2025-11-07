use crate::models::{Node, Rule};

pub struct GraphSearch {
    pub rules: Vec<Rule>,
    pub target: Node,
    pub flag_y: bool,
    pub flag_n: bool,

    pub open_nodes: Vec<Node>,
    pub closed_nodes: Vec<Node>,
    pub forbidden_nodes: Vec<Node>,

    pub open_rules: Vec<usize>,
    pub closed_rules: Vec<usize>,
    pub forbidden_rules: Vec<usize>,
}

impl GraphSearch {
    pub fn new(rules: Vec<Rule>, initial: Vec<Node>, target: Node)  -> Self {
        Self {
            rules,
            target: target.clone(),
            flag_y: true,
            flag_n: true,
            closed_nodes: initial,
            open_nodes: vec![target],
            forbidden_nodes: vec![],
            open_rules: vec![],
            closed_rules: vec![],
            forbidden_rules: vec![],
        }
    }

    pub fn get_closed_rules_id(&self) -> Vec<i32>{
        self.rules.iter().filter(|r| r.mark == 1).map(|r| r.id).collect::<Vec<_>>()
    }

    pub fn search(&mut self) {
        while self.flag_y && self.flag_n {
            let found_result = self.child_search();
            if !self.flag_y {
                println!("Решение найдено");
            } else if !found_result && self.open_nodes.len() == 1 && self.open_nodes.contains(&self.target) {
                println!("Решения нет");
                self.flag_n = false;
            } else if !found_result && self.open_nodes.len() != 0 {
                self.back_tracking();
            }
        }
    }

    /* возвращает
        1, если нашли правило
        0, если нет */
    pub fn child_search(&mut self) -> bool {
println!("Открытые вершины {}", self.open_nodes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
println!("Закрытые вершины {}", self.closed_nodes.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
        let mut rule_index = 0;
        for rule in &mut self.rules {
            /* если выходная вершина совпадает с подцелью, т.е. раскрывает эту вершину и метка правила == 0 */
            if rule.output == *self.open_nodes.last().unwrap() && rule.mark == 0 {
                rule.mark = 1;
                /* номер правила пишем в голову стека открытых правил */
                self.open_rules.push(rule_index);
                println!("\tОткрыли правило {}", rule.id);

                let mut counter = 0;
                // определяем какие вершины выбранного правила не входят в закрытые
                for node in &mut rule.inputs {
                    // если вершины в списке доказанных выставляем флаг
                    if self.closed_nodes.contains(node) {
                        node.flag = true;
                    // определяем какие вершины выбранного правила не входят в закрытые
                    } else if !node.flag {
                        counter += 1;
                        self.open_nodes.push(node.clone());
                    }
                }

                if counter == 0 {
                    self.markup();
                }
                return true;
            }

            rule_index += 1;
        }

        return false;
    }

        /* алгоритм разметки */
    pub fn markup(&mut self) {
println!("\tРазметка");
        while self.flag_y {
            /* проверить, выполняется ли покрытие входных вершин правила из головы стека закрытыми */
            let current_rule = *self.open_rules.last().unwrap();
            let current_nodes = &self.rules[current_rule].inputs;
println!("\t\t{}", self.rules[current_rule].id);
            let mut flag = true;
            for node in current_nodes {
                let found_result = self.closed_nodes.contains(&node);
                flag &= found_result;
                if !found_result {
println!("\t\tЕщё не доказана вершина {}", node);
                }
            }

            if flag {
                if self.rules[current_rule].output == self.target {
                    self.flag_y = false;
                }

                // ставим флаг, что правило доказана
                self.rules[current_rule].mark = 1;

                // удаляем из головы открытых вершин
                self.open_rules.pop();
                self.closed_rules.push(current_rule);
                self.closed_nodes.push(self.open_nodes.pop().unwrap());

            } else {
                break;
            }
        }
    }

    pub fn back_tracking(&mut self) {
        let current_rule = *self.open_rules.last().unwrap();
        let current_nodes = &self.rules[current_rule].inputs;
        let mut flag = true;
        for node in current_nodes {
            if let Some(pos) = self.open_nodes.iter().position(|n| n == node) {
                let node = self.open_nodes.remove(pos);

                // Если вершина не доказана и ещё не была запрещена
                if !node.flag && !flag {
                    self.forbidden_nodes.push(node);
                    flag = true;
                }
            }
        }

println!("\tЗапретили правило {}", self.rules[current_rule].id);

        // Помечаем правило как запрещённое
        self.rules[current_rule].mark = -1;

        // Перемещаем правило в список запрещённых
        self.forbidden_rules.push(current_rule);

        // Удаляем его из стека открытых правил
        self.open_rules.pop();
    }

}