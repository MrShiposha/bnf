extern crate bnf;

use bnf::Grammar;
// use bnf::Production;
use bnf::Expression;
use bnf::Term;

use std::collections::HashSet;

// let input =
// "
// <Rule1> ::= <Rule2> | <Rule2> <Rule1>
// <Rule2> ::= \"ABC\" | \"AB\" | \"BC\" | \"AC\" <Rule3> | <Rule4>
// <Rule3> ::= \"AB\" | \"BC\" | \"AG\" | \"T\" | <Rule4>
// <Rule4> ::= \"BC\" | \"AC\"
// <Rule5> ::= \"QR\" | \"ST\"
// ";

fn earley_predictor(grammar: &Grammar, term: &Term) -> HashSet<(Term, Expression)> {
    let mut candidates: HashSet<(Term, Expression)> = HashSet::new();
    for prod in grammar.productions_iter() {
        if prod.lhs == *term {
            for expr in prod.rhs_iter() {
                candidates.insert((prod.lhs.clone(), expr.clone()));
            }
        }
    }

    let mut container: HashSet<(Term, Expression)> = HashSet::new();
    for expr in &candidates {
        for t in expr.1.terms_iter() {
            if let Term::Nonterminal(_) = *t {
                if t != term {
                    container = earley_predictor(&grammar, t).union(&container).cloned().collect();
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
    <Rule1> ::= <Rule2> | <Rule2> <Rule1>
    <Rule2> ::= \"ABC\" | \"AB\" | \"BC\" | \"AC\" <Rule3> | <Rule4>
    <Rule3> ::= \"AB\" | \"BC\" | \"AG\" | \"T\" | <Rule4>
    <Rule4> ::= \"BC\" | \"AC\"
    <Rule5> ::= \"QR\" | \"ST\"
    ";

    let grammar = Grammar::from_str(input).unwrap();

    // scanner
    let input = String::from("ABCACT");
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

    // println!("matches: {:?}", matches);

    let term = Term::Nonterminal(String::from("Rule1"));
    let candidates: HashSet<(Term, Expression)> = earley_predictor(&grammar, &term);

    println!("candidates: {:#?}", candidates);
}
