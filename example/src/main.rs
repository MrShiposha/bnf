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

fn earley_completer(state: &State) -> HashSet<EarleyProduction> {
    let mut updates: HashSet<EarleyProduction> = HashSet::new();
    for production in &state.productions {
        if !earley_finished(production) {
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

fn earley_finished(production: &EarleyProduction) -> bool {
    if production.dot == production.terms.len() {
        return true;
    }

    return false;
}

fn earley_next_element(production: &mut EarleyProduction) -> Option<&Term> {
    let ret = production.terms.iter().nth(production.dot);
    production.dot += 1;
    ret
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

    // let result: Vec<State> = vec![];

    for k in 0..input.len() {
        if let Some(state) = states.clone().iter().nth(k) {
            let mut productions: Vec<EarleyProduction> =
                state.productions.iter().cloned().collect::<Vec<_>>();

            let mut collection: State = State::new();

            while let Some(mut production) = productions.pop() {
                let mut clone = production.clone();
                let ret = earley_next_element(&mut production);
                if let Some(term) = ret {
                    match *term {
                        Term::Nonterminal(_) =>
                        {
                            collection.productions = earley_predictor(term, k, &grammar)
                                        .union(&collection.productions)
                                        .cloned()
                                        .collect::<HashSet<_>>();
                        }
                        Term::Terminal(_) => {
                            if earley_scanner(&term, k, &input, &grammar) {
                                if let Some(state) = states.iter_mut().nth(k + 1) {
                                    clone.dot += 1;
                                    state.productions.insert(clone);
                                }
                            }
                        }
                    }
                } else {
                    collection.productions = earley_completer(state)
                                .union(&collection.productions)
                                .cloned()
                                .collect::<HashSet<_>>();
                }
            }

            states.remove(k);
            states.insert(k, collection)
        }
    }

    for state in states {
        println!("{:#?}\n", state);
    }
}
