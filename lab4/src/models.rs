use std::fmt;

#[derive(Clone, Debug)]
pub struct Term {
    pub name: String,
    pub kind: TermKind,
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TermKind {
    Const,
    Var,
}

impl Term {
    pub fn new(name: &str) -> Self {
        let first = name.chars().next().unwrap();
        let kind = if first.is_uppercase() {
            TermKind::Const
        } else {
            TermKind::Var
        };

        let value = if kind == TermKind::Const {
            Some(name.to_string())
        } else {
            None
        };

        Term {
            name: name.to_string(),
            kind,
            value,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:?}", self.name, self.kind)
    }
}

#[derive(Clone, Debug)]
pub struct Atom {
    pub name: String,
    pub sign: bool,
    pub args: Vec<Term>,
}

impl Atom {
    pub fn new(name: &str, args: Vec<Term>, sign: bool) -> Self {
        Atom {
            name: name.to_string(),
            args,
            sign,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = if self.sign { "" } else { "~" };
        let args = self
            .args
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}{}({})", prefix, self.name, args)
    }
}

#[derive(Clone, Debug)]
pub struct Disjunct {
    pub args: Vec<Atom>,
}

impl Disjunct {
    pub fn new(args: Vec<Atom>) -> Self {
        Disjunct { args }
    }

    pub fn parse(exp: &str) -> Self {
        let parts = exp.split('v');
        let mut atoms = Vec::new();

        for part in parts {
            let mut p = part.trim().to_string();
            let sign = !p.starts_with('~');

            if !sign {
                p = p[1..].trim().to_string();
            }

            let open = p.find('(').unwrap();
            let close = p.rfind(')').unwrap();

            let name = p[..open].trim();
            let args_str = &p[open + 1..close];

            let mut terms = Vec::new();
            for t in args_str.split(',') {
                terms.push(Term::new(t.trim()));
            }

            atoms.push(Atom::new(name, terms, sign));
        }

        Disjunct::new(atoms)
    }
}

impl fmt::Display for Disjunct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .args
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" | ");
        write!(f, "{}", s)
    }
}
