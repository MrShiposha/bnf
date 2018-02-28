extern crate bnf;

use bnf::Grammar;
use bnf::Term;
use std::collections::HashSet;

// #[derive(Clone, Debug, Eq, PartialEq)]
// struct State {
//     origin: usize,
//     productions: HashSet<EarleyProduction>,
// }

// impl State {
//     pub fn new() -> State {
//         State {
//             origin: 0,
//             productions: HashSet::new(),
//         }
//     }
// }

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct EarleyProduction {
    origin: usize,
    lhs: Term,
    terms: Vec<Term>,
    dot: usize,
}

fn earley_predictor(term: &Term, k: usize, grammar: &Grammar) -> HashSet<EarleyProduction> {
    let mut productions_first: HashSet<EarleyProduction> = HashSet::new();
    let mut productions_last: HashSet<EarleyProduction> = HashSet::new();

    for prod in grammar.productions_iter() {
        if prod.lhs == *term {
            for expr in prod.rhs_iter() {
                productions_first.insert(EarleyProduction {
                    origin: k,
                    lhs: prod.lhs.clone(),
                    terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                    dot: 0,
                });
            }
        }
    }

    for production in &productions_first {
        for t in &production.terms {
            if let Term::Nonterminal(_) = *t {
                if t != term {
                    productions_last = earley_predictor(&t, k, &grammar)
                        .union(&productions_last)
                        .cloned()
                        .collect();
                }
            }
        }
    }

    productions_first
        .union(&productions_last)
        .cloned()
        .collect()
}

fn earley_scanner(
    term: &Term,
    k: usize,
    words: &String,
    grammar: &Grammar,
    production: &EarleyProduction,
) -> HashSet<EarleyProduction> {
    let mut pattern: String = String::new();
    let mut matches: HashSet<EarleyProduction> = HashSet::new();
    for (_, c) in words[k..].chars().enumerate() {
        pattern.push(c);
        for prod in grammar.productions_iter() {
            for expr in prod.rhs_iter() {
                for t in expr.terms_iter() {
                    if let Term::Terminal(ref s) = *t {
                        if t == term {
                            if pattern == *s {
                                let mut p = production.clone();
                                p.dot += 1;
                                matches.insert(p);
                            } else {
                                pattern = String::new();
                            }
                        }
                    }
                }
            }
        }
    }

    matches
}

fn earley_completer(productions: &Vec<EarleyProduction>, finished: &EarleyProduction) -> HashSet<EarleyProduction> {
    let mut updates: HashSet<EarleyProduction> = HashSet::new();
    for production in productions {
        if let Some(term) = earley_next_element(&production) {
            if finished.lhs == *term {
                let mut update = production.clone();
                update.dot += 1;
                updates.insert(update);
            }
        }
    }

    updates
}

fn earlt_init(grammar: &Grammar) -> Option<HashSet<EarleyProduction>> {
    if let Some(prod) = grammar.productions_iter().nth(0) {
        let mut productions: HashSet<EarleyProduction> = HashSet::new();
        for expr in prod.rhs_iter() {
            productions.insert(EarleyProduction {
                origin: 0,
                lhs: prod.lhs.clone(),
                terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                dot: 0,
            });
        }

        return Some(productions);
    }

    return None;
}

fn earley_next_element(production: &EarleyProduction) -> Option<&Term> {
    production.terms.iter().nth(production.dot)
}

fn main() {
    let input = "
    <P> ::= <S>
    <S> ::= <S> \"+\" <M> | <M>
    <M> ::= <M> \"*\" <T> | <T>
    <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
    ";

    let grammar = Grammar::from_str(input).unwrap();

    // scanner
    let input = String::from("2 + 3 * 4");

    let mut states: Vec<HashSet<EarleyProduction>> = vec![HashSet::new(); input.len()];

    if let Some(intial) = earlt_init(&grammar) {
        states[0] = intial;
    } else {
        println!("Something in init went wrong!");
    }

    for k in 0..input.len() {
        let mut productions: Vec<EarleyProduction> = states[k].iter().cloned().collect::<Vec<_>>();
        states[k].drain();

        while let Some(mut production) = productions.pop() {
            if states[k].contains(&production) {
                continue;
            }

            states[k].insert(production.clone());

            if let Some(term) = earley_next_element(&production) {
                match *term {
                    Term::Nonterminal(_) => {
                        productions.extend(
                            earley_predictor(term, k, &grammar)
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>(),
                        );
                    }
                    Term::Terminal(_) => {
                        states[k+1].extend(earley_scanner(&term, k, &input, &grammar, &production));
                    }
                }
            } else {
                productions.extend(
                    earley_completer(&states[production.origin].iter().cloned().collect::<Vec<_>>(), &production)
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                );
            }
        }
    }

    for state in states {
        for production in state {
            if let None = earley_next_element(&production) {
                println!("{:?}\n\n", production);
            }
        }
    }
}
