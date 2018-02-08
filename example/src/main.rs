extern crate bnf;

use bnf::Grammar;
// use bnf::Production;
use bnf::Expression;
use bnf::Term;


use std::collections::HashSet;

// let input =
// r#"
// <Rule1> ::= <Rule2> | <Rule2> <Rule1>
// <Rule2> ::= "ABC" | "AB" | "BC" | "AC" | "AG" | "T" | <Rule3> | <Rule4>
// <Rule3> ::= "AB" | "BC" | "AC" | "AG" | "T" | <Rule4>
// <Rule4> ::= "BC" | "AC"
// <Rule5> ::= "QR" | "ST"
// "#;

fn earley_predictor(grammar: &Grammar, term: &Term) -> HashSet<Expression> {
    let mut candidates: HashSet<Expression> =  HashSet::new();
    for prod in grammar.productions_iter() {
        if prod.lhs == *term {
            for expr in prod.rhs_iter() {
                candidates.insert(expr.clone());
            }
        }
    }

    candidates
}

fn earley_scanner(grammar: &Grammar, term: &Term) -> HashSet<Term> {
    let mut matches: HashSet<Term> =  HashSet::new();
    for prod in grammar.productions_iter() {
        for expr in prod.rhs_iter() {
            expr_as_nonterm(expr.clone());
            for t in expr.terms_iter() {
                if let Term::Terminal(_) = *t {
                    if t == term {
                        matches.insert(prod.lhs.clone());
                        // println!("matched {} with rule {}", term, prod.lhs);
                    }
                }
            }
        }
    }

    matches
}

fn main() {
    let input =
    r#"
    <Rule1> ::= <Rule2> | <Rule2> <Rule1>
    <Rule2> ::= "ABC" | "AB" | "BC" | "AC" | "AG" | "T" | <Rule3> | <Rule4>
    <Rule3> ::= "AB" | "BC" | "AC" | "AG" | "T" | <Rule4>
    <Rule4> ::= "BC" | "AC"
    <Rule5> ::= "QR" | "ST"
    "#;

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

    // matches.sort();
    println!("matches: {:?}", matches);

    let term = Term::Nonterminal(String::from("Rule1"));
    let candidates: HashSet<Expression> = earley_predictor(&grammar, &term);

    println!("candidates: {:?}", candidates);
}
