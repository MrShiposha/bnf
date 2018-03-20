extern crate bnf;

use bnf::Grammar;
use bnf::Term;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    origin: usize,
    lhs: Term,
    terms: Vec<Term>,
    dot: usize,
}

fn earley_predictor(term: &Term, k: usize, grammar: &Grammar) -> HashSet<State> {
    let mut productions: HashSet<State> = HashSet::new();

    for prod in grammar.productions_iter() {
        if prod.lhs == *term {
            for expr in prod.rhs_iter() {
                productions.insert(State {
                    origin: k,
                    lhs: prod.lhs.clone(),
                    terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                    dot: 0,
                });
            }
        }
    }

    productions
}

fn earley_scanner(
    term: &Term,
    k: usize,
    words: &String,
    grammar: &Grammar,
    production: &State,
) -> HashSet<State> {
    let mut pattern: String = String::new();
    let mut matches: HashSet<State> = HashSet::new();
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
                            }
                        }
                    }
                }
            }
        }
    }

    matches
}

fn earley_completer(productions: &HashSet<State>, finished: &State) -> HashSet<State> {
    let mut updates: HashSet<State> = HashSet::new();
    for production in productions {
        if let Some(term) = earley_next_element(production) {
            if finished.lhs == *term {
                let mut update = production.clone();
                update.dot += 1;
                updates.insert(update);
            }
        }
    }

    updates
}

fn earlt_init(grammar: &Grammar) -> Option<HashSet<State>> {
    if let Some(prod) = grammar.productions_iter().nth(0) {
        let mut productions: HashSet<State> = HashSet::new();
        for expr in prod.rhs_iter() {
            productions.insert(State {
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

fn earley_next_element(production: &State) -> Option<&Term> {
    production.terms.iter().nth(production.dot)
}

fn hashset(data: &[State]) -> HashSet<State> {
    HashSet::from_iter(data.iter().cloned())
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
    let input = String::from("2+3*4");

    let mut states: Vec<HashSet<State>> = vec![HashSet::new(); input.len() + 1];
    let mut productions: Vec<State> = vec![];

    if let Some(intial) = earlt_init(&grammar) {
        states[0] = intial;
    } else {
        println!("Something in init went wrong!");
    }

    for k in 0..input.len() + 1 {
        if let Some(state) = states.iter_mut().nth(k) {
            productions = state.iter().cloned().collect::<Vec<_>>();
            state.drain();
        }

        while let Some(mut production) = productions.pop() {
            if let Some(state) = states.iter_mut().nth(k) {
                if state.contains(&production) {
                    continue;
                }

                state.insert(production.clone());
            }

            if let Some(term) = earley_next_element(&production) {
                match *term {
                    Term::Nonterminal(_) => {
                        let predicted = earley_predictor(&term, k, &grammar);
                        productions = hashset(&productions).union(&predicted).cloned().collect();
                    }
                    Term::Terminal(_) => {
                        if let Some(state) = states.iter_mut().nth(k + 1) {
                            let scanned = earley_scanner(&term, k, &input, &grammar, &production);
                            *state = scanned.union(&state).cloned().collect();
                        }
                    }
                }
            } else {
                if let Some(state) = states.iter_mut().nth(production.origin) {
                    let completed = earley_completer(&state, &production);
                    productions = hashset(&productions).union(&completed).cloned().collect();
                }
            }
        }
    }

    for (i, state) in states.iter().enumerate() {
        println!("\n---S({})\n", i);
        for (_, production) in state.iter().enumerate() {
            let finished: String;
            if let None = earley_next_element(production) {
                finished = String::from("(complete)");
            } else {
                finished = String::from("");
            }
            println!(
                "{} | {} -> {:?} - dot:{} {}",
                production.origin, production.lhs, production.terms, production.dot, finished
            );
        }
    }
}
