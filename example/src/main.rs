extern crate bnf;

use bnf::Grammar;
use bnf::Term;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    origin: usize,
    productions: HashSet<EarleyProduction>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct EarleyProduction {
    lhs: Term,
    terms: Vec<Term>,
    dot: usize,
}

// #[derive(Clone, Debug, Eq, PartialEq)]
// struct PartsOfSpeech {
//     terms: HashSet<Term>,
// }

// impl PartsOfSpeech {
//     /// Construct a new `PartsOfSpeech`
//     pub fn new() -> PartsOfSpeech {
//         PartsOfSpeech {
//             terms: HashSet::new(),
//         }
//     }
// }

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

fn earley_scanner(term: &Term, k: usize, words: &String, grammar: &Grammar) -> bool {
    // let mut matches: PartsOfSpeech = PartsOfSpeech::new();
    let mut pattern: String = String::new();
    for (_, c) in words[k..].chars().enumerate() {
        pattern.push(c);
        for prod in grammar.productions_iter() {
            for expr in prod.rhs_iter() {
                for t in expr.terms_iter() {
                    if let Term::Terminal(_) = *t {
                        if t == term {
                            return true;
                            // matches.terms.insert(prod.lhs.clone());
                        }
                    }
                }
            }
        }
    }

    return false;
    // matches
}

fn earley_completer(state: &State) -> Vec<EarleyProduction> {
    let mut updates: HashSet<EarleyProduction> = HashSet::new();
    for production in &state.productions {
        if !earley_finished(production) {
            let mut update = production.clone();
            update.dot += 1;
            updates.insert(update);
        }
    }

    updates.iter().cloned().collect()
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

fn earley_finished(production: &EarleyProduction) -> bool {
    if production.dot == production.terms.len() {
        return true;
    }

    return false;
}

fn earley_next_element(production: &EarleyProduction) -> Option<&Term> {
    return production.terms.iter().nth(production.dot);
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

            while let Some(production) = productions.pop() {
                if !earley_finished(&production) {
                    if let Some(term) = earley_next_element(&production) {
                        match *term {
                            Term::Nonterminal(_) => productions.extend(
                                earley_predictor(term, k, &grammar)
                                    .iter()
                                    .cloned()
                                    .collect::<Vec<_>>(),
                            ),
                            Term::Terminal(_) => {
                                if earley_scanner(&term, k, &input, &grammar) {
                                    if let Some(state) = states.iter_mut().nth(k + 1) {
                                        let mut new_production = production.clone();
                                        new_production.dot += 1;
                                        state.productions.insert(new_production);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // productions.extend(earley_completer(state));
                }

                let temp = productions.clone();
                for t in temp {
                    println!("{:?}", t);
                }
            }
        }
    }
}
