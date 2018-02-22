extern crate bnf;

use bnf::Grammar;
use bnf::Term;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct PartsOfSpeech {
    terms: HashSet<Term>,
}

impl PartsOfSpeech {
    /// Construct a new `PartsOfSpeech`
    pub fn new() -> PartsOfSpeech {
        PartsOfSpeech { terms: HashSet::new() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    origin: Term,
    rules: HashSet<Vec<Term>>,
    dot: usize,
}

// impl State {
//     /// Construct a new `State`
//     pub fn new() -> State {
//         State { rules: HashSet::new(), dot: 0 }
//     }
// }

// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// struct State {
//     parts_of_speech: PartsOfSpeech,
//     grammar_rules: State,
// }

// fn earley_predictor(grammar: &Grammar, term: &Term) -> State {
//     let mut candidates = State::new();
//     for prod in grammar.productions_iter() {
//         if prod.lhs == *term {
//             for expr in prod.rhs_iter() {
//                 candidates.rules.insert((prod.lhs.clone(), expr.terms_iter()
//                                                                .cloned()
//                                                                .collect()));
//             }
//         }
//     }

//     let mut container: State = State::new();
//     for candidate in &candidates.rules {
//         for c in &candidate.1 {
//             if let Term::Nonterminal(_) = *c {
//                 if c != term {
//                     container.rules =
//                         earley_predictor(&grammar, &c)
//                                                     .rules
//                                                     .union(&container.rules)
//                                                     .cloned()
//                                                     .collect();
//                 }
//             }
//         }
//     }

//     State{ rules: candidates.rules.union(&container.rules).cloned().collect(), dot: 0 }
// }

fn earley_scanner(term: &Term, k: usize, words: String, grammar: &Grammar) -> PartsOfSpeech {
    let mut matches: PartsOfSpeech =  PartsOfSpeech::new();
    let mut pattern: String = String::new();
    for(_, c) in words[k..].chars().enumerate() {
        pattern.push(c);
        let new_term = Term::Terminal(pattern.clone());
        for prod in grammar.productions_iter() {
            for expr in prod.rhs_iter() {
                for t in expr.terms_iter() {
                    if let Term::Terminal(_) = *t {
                        if t == term {
                            matches.terms.insert(prod.lhs.clone());
                        }
                    }
                }
            }
        }
    }

    matches
}

fn earlt_init(grammar: &Grammar) -> Option<State> {
    if let Some(prod) = grammar.productions_iter().nth(0) {
        let origin = prod.lhs.clone();
        let mut rules: HashSet<Vec<Term>> = HashSet::new();
        for expr in prod.rhs_iter() {
            rules.insert(expr.terms_iter().cloned().collect::<Vec<_>>());
        }

        return Some(State { origin, rules, dot: 0 })
    }

    return None
}

fn earley_finished(state: &State, rules: &Vec<Term> ) -> bool {
    if state.dot == (rules.len()) {
        return true
    }

    return false
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
    let input = String::from("2 + 3 * 4");

    let candidates: State;
    let mut states: Vec<State> = vec![];

    if let Some(state) = earlt_init(&grammar) {
        states.push(state);
    } else {
        return
    }

    for i in 0..input.len() {
        for state in &mut states {
            for rule in &state.rules {
                if !earley_finished(&state, rule) {

                    // let _ = earley_scanner(&grammar, &new_term).union(&matches)
                    //                                             .cloned()
                    //                                             .collect();

                    // candidates = earley_predictor(&grammar, &term);
                } else {
                    //
                }
            }
        }
    }

}
