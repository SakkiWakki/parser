use cfg::structs::{Grammar, NonTerm, Symbol, Term};

#[test]
fn builds_a_minimal_grammar() {
    let start = NonTerm::new(0);
    let literal = Term::new(0);
    let rules = vec![(start, vec![Symbol::Term(literal)])];

    let grammar = Grammar::new(vec![start], vec![literal], rules, start);

    assert_eq!(grammar.start(), start);
    assert!(grammar.nonterms().contains(&start));
    assert_eq!(grammar.rules().len(), 1);
}
