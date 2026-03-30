#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Term(Term),
    NonTerm(NonTerm),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NonTerm(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Term(u32);

pub type Rule = (NonTerm, Vec<Symbol>);


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTree {
    sym: Symbol,
    rule: Option<Rule>,
    children: Vec<ParseTree>,
}
/// V is a finite set of nonterms
/// Sigma is a finite set of terms
/// R is a set of relations/rules
/// S is the starting nonterminal symbol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grammar {
    v: Vec<NonTerm>,
    sig: Vec<Term>,
    r: Vec<Rule>,
    s: NonTerm,
}

impl NonTerm {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

impl Term {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

impl ParseTree {
    pub fn new(sym: Symbol, rule: Option<Rule>, children: Vec<ParseTree>) -> Self {
        Self { sym, rule, children }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.sym
    }

    pub fn children(&self) -> &[ParseTree] {
        &self.children
    }
}

impl Grammar {
    pub fn new(v: Vec<NonTerm>, sig: Vec<Term>, r: Vec<Rule>, s: NonTerm) -> Self {
        Self { v, sig, r, s }
    }

    pub fn start(&self) -> NonTerm {
        self.s
    }

    pub fn nonterms(&self) -> &[NonTerm] {
        &self.v
    }

    pub fn terms(&self) -> &[Term] {
        &self.sig
    }

    pub fn rules(&self) -> &[Rule] {
        &self.r
    }
}
