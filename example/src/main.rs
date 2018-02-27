extern crate bnf;

use bnf::Grammar;
use bnf::Term;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    origin: usize,
    productions: HashSet<EarleyProduction>,
}

impl State {
    pub fn new() -> State {
        State {
            origin: 0,
            productions: HashSet::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct EarleyProduction {
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
) -> HashSet<EarleyProduction> {
    let mut matches: HashSet<EarleyProduction> = HashSet::new();
    let mut pattern: String = String::new();
    for (_, c) in words[k..].chars().enumerate() {
        pattern.push(c);
        for prod in grammar.productions_iter() {
            for expr in prod.rhs_iter() {
                for t in expr.terms_iter() {
                    if let Term::Terminal(_) = *t {
                        if t == term {
                            matches.insert(EarleyProduction {
                                lhs: prod.lhs.clone(),
                                terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                                dot: 0,
                            });
                        }
                    }
                }
            }
        }
    }

    matches
}

fn earley_completer(productions: &Vec<EarleyProduction>) -> HashSet<EarleyProduction> {
    let mut updates: HashSet<EarleyProduction> = HashSet::new();
    for production in productions {
        if let Some(&Term::Nonterminal(_)) = earley_next_element(&production) {
            let mut update = production.clone();
            update.dot += 1;
            updates.insert(update);
        }
    }

    updates
}

fn earlt_init(grammar: &Grammar) -> Option<State> {
    if let Some(prod) = grammar.productions_iter().nth(0) {
        let mut productions: HashSet<EarleyProduction> = HashSet::new();
        for expr in prod.rhs_iter() {
            productions.insert(EarleyProduction {
                lhs: prod.lhs.clone(),
                terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                dot: 0,
            });
        }

        return Some(State {
            origin: 0,
            productions,
        });
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

    let mut states: Vec<State> = vec![];

    if let Some(s) = earlt_init(&grammar) {
        states.push(s);
    } else {
        println!("Something went wrong!");
        return;
    }

    for k in 0..input.len() {
        if let Some(state) = states.clone().iter().nth(k) {
            let mut productions: Vec<EarleyProduction> =
                state.productions.iter().cloned().collect::<Vec<_>>();
            let mut tracked: Vec<Vec<EarleyProduction>> = vec![vec![]; productions.len()];
            while let Some(mut production) = productions.pop() {
                if let Some(contents) = tracked.iter_mut().nth(k) {
                    if contents.contains(&production) {
                        continue;
                    }

                    contents.push(production.clone());

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
                                if let Some(track) = tracked.iter_mut().nth(k + 1) {
                                    let matches = earley_scanner(&term, k, &input, &grammar)
                                        .iter()
                                        .cloned()
                                        .collect::<Vec<_>>();
                                    track.extend(matches);
                                }
                            }
                        }
                    } else {
                        productions.extend(
                            earley_completer(contents)
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
        }
    }

    for state in states {
        for production in state.productions {
            println!("{:?}\n\n", production);
        }
    }
}
