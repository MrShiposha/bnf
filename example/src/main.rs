extern crate bnf;

use bnf::Grammar;
use bnf::Term;
use std::collections::HashSet;

fn earley_predictor(grammar: &Grammar, term: &Term) -> HashSet<(Term, Vec<Term>)> {
    let mut candidates: HashSet<(Term, Vec<Term>)> = HashSet::new();
    for prod in grammar.productions_iter() {
        if prod.lhs == *term {
            for expr in prod.rhs_iter() {
                candidates.insert((prod.lhs.clone(), expr.terms_iter()
                                                         .cloned()
                                                         .collect()));
            }
        }
    }

    let mut container: HashSet<(Term, Vec<Term>)> = HashSet::new();
    for candidate in &candidates {
        for c in &candidate.1 {
            if let Term::Nonterminal(_) = *c {
                if c != term {
                    container = earley_predictor(&grammar, &c).union(&container)
                                                              .cloned()
                                                              .collect();
                }
            }
        }
    }

    candidates.union(&container).cloned().collect()
}

fn earley_scanner(grammar: &Grammar, term: &Term) -> HashSet<Term> {
    let mut matches: HashSet<Term> =  HashSet::new();
    for prod in grammar.productions_iter() {
        for expr in prod.rhs_iter() {
            for t in expr.terms_iter() {
                if let Term::Terminal(_) = *t {
                    if t == term {
                        matches.insert(prod.lhs.clone());
                    }
                }
            }
        }
    }

    matches
}

fn main() {
    let input =
    "
    <P> ::= <S>
    <S> ::= <S> \"+\" <M> | <M>
    <M> ::= <M> \"*\" <T> | <T>
    <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
    ";

    let grammar = Grammar::from_str(input).unwrap();

    // scanner
    let input = String::from("1234");
    let mut pattern: String = String::new();
    let mut matches: HashSet<Term> =  HashSet::new();
    for i in 0..input.len() {
        for(_, c) in input[i..].chars().enumerate() {
            pattern.push(c);
            let new_term = Term::Terminal(pattern.clone());
            matches = earley_scanner(&grammar, &new_term).union(&matches).cloned().collect();
        }
        pattern.clear();
    }

    // println!("matches: {:?}\n\n", matches);

    let term = Term::Nonterminal(String::from("P"));
    let candidates: HashSet<(Term, Vec<Term>)> = earley_predictor(&grammar, &term);

    for candidate in candidates {
        println!("{:?}", candidate);
    }

    // println!("candidates: {:#?}", candidates);
}
