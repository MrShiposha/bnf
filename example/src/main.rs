extern crate bnf;

use bnf::Grammar;
use bnf::Term;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    origin: Option<usize>,
    lhs: Option<Term>,
    terms: Vec<Term>,
    dot: Option<usize>,
}

fn earley_predictor(term: &Term, k: usize, grammar: &Grammar) -> HashSet<State> {
    let mut productions: HashSet<State> = HashSet::new();

    for prod in grammar.productions_iter() {
        if prod.lhs == *term {
            for expr in prod.rhs_iter() {
                productions.insert(State {
                    origin: Some(k),
                    lhs: Some(prod.lhs.clone()),
                    terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                    dot: Some(0),
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
                                let mut update = production.clone();
                                if let Some(dot) = update.dot {
                                    update.dot = Some(dot + 1);
                                }
                                matches.insert(update);
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
            if let &Some(ref lhs) = &finished.lhs {
                if lhs == term {
                    let mut update = production.clone();
                    if let Some(dot) = update.dot {
                        update.dot = Some(dot + 1);
                    }
                    updates.insert(update);
                }
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
                origin: Some(0),
                lhs: Some(prod.lhs.clone()),
                terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                dot: Some(0),
            });
        }

        return Some(productions);
    }

    return None;
}

fn earley_next_element(production: &State) -> Option<&Term> {
    if let Some(dot) = production.dot {
        return production.terms.iter().nth(dot);
    }

    None
}

fn hashset(data: &[State]) -> HashSet<State> {
    HashSet::from_iter(data.iter().cloned())
}

fn main() {
    // let g = "
    // <P> ::= <S>
    // <S> ::= <S> \"+\" <M> | <M>
    // <M> ::= <M> \"*\" <T> | <T>
    // <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
    // ";
    // // scanner
    // let input = String::from("2+3*4");

    let g = "
    <Sum> ::= <Sum> '+' <Product> | <Sum> '-' <Product> | <Product>
    <Product> ::= <Product> '*' <Factor> | <Product> '/' <Factor> | <Factor>
    <Factor> ::= '(' <Sum> ')' | <Number>
    <Number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ";

    let input = String::from("1+(2*3-4)");

    let grammar = Grammar::from_str(g).unwrap();

    let mut states: Vec<HashSet<State>> = vec![HashSet::new(); input.len() + 1];
    let mut productions: Vec<State> = vec![];

    let mut tokens: Vec<String> = vec![];

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
                    Term::Terminal(ref t) => {
                        if let Some(state) = states.iter_mut().nth(k + 1) {
                            let scanned = earley_scanner(&term, k, &input, &grammar, &production);

                            if scanned.len() > 0 {
                                tokens.push(t.clone());
                            }

                            *state = scanned.union(&state).cloned().collect();
                        }
                    }
                }
            } else {
                if let Some(origin) = production.origin {
                    if let Some(state) = states.iter_mut().nth(origin) {
                        let completed = earley_completer(&state, &production);
                        productions = hashset(&productions).union(&completed).cloned().collect();
                    }
                }
            }
        }
    }

    //****************

    // println!("input: {:?}", tokens);

    //****************

    //****************

    // let mut curr: Option<State> = None;
    // let mut parse: Vec<State> = vec![];

    // if let Some(state) = states.iter().nth(states.len() - 1) {
    //     for production in state {
    //         if let None = earley_next_element(&production) {
    //             if let Some(0) = production.origin {
    //                 curr = Some(production.clone());
    //                 // parse.push(production.clone());
    //                 break;
    //             }
    //         }
    //     }
    // }

    // let mut terms: Vec<Term> = vec![];
    // if let Some(c) = curr {
    //     terms = c.terms;
    // }

    // let mut c: Option<Term> = terms.pop();

    // for state in states.iter().rev() {
    //     for production in state {
    //         let cl = c.clone();
    //         if let Some(term) = cl {
    //             match term {
    //                 Term::Nonterminal(_) => {
    //                     if let None = earley_next_element(production) {
    // if let Some(ref prod) = production.lhs {
    //     if *prod == term {
    //         parse.push(production.clone());
    //         c = terms.pop();
    //     }
    // }
    //                     }
    //                 }
    //                 Term::Terminal(_) => {
    //                     parse.push(
    //                         State {
    //                             origin: None,
    //                             lhs: None,
    //                             terms: vec![term.clone()],
    //                             dot: None,
    //                         }
    //                     );

    //                     c = terms.pop();
    //                 }
    //             }
    //         }
    //     }
    // }

    //****************

    //****************

    let mut terms: Vec<Term> = vec![];
    let mut parent: Option<State> = None;

    let mut completed_states: Vec<Vec<State>> = vec![];

    for state in &states {
        let mut completes: Vec<State> = vec![];
        for prod in state {
            if let None = earley_next_element(&prod) {
                completes.push(prod.clone());
            }
        }

        completed_states.push(completes.clone());
    }

    if let Some(state) = completed_states.iter().nth(states.len() - 1) {
        for production in state {
            if let Some(0) = production.origin {
                terms = production.terms.clone();
                parent = Some(production.clone());
                break;
            }
        }
    } else {
        return;
    }

    recurse(&completed_states, terms, parent);

    fn recurse(states: &Vec<Vec<State>>, mut terms: Vec<Term>, parent: Option<State>) {
        let mut parse: Vec<State> = vec![];

        fn do_work(states: &Vec<Vec<State>>, rule: &Term, dot: usize) -> Option<State> {
            if let Some(state) = states.iter().nth(dot) {
                for production in state {
                    if let Some(ref prod) = production.lhs {
                        if prod == rule {
                            return Some(production.clone());
                        }
                    }
                }
            }

            None
        }

        let mut dot = states.len() - 1;
        while let Some(term) = terms.pop() {
            match term {
                Term::Nonterminal(_) => {
                    if let Some(mut s) = do_work(&states, &term, dot) {
                        if let Some(d) = s.dot {
                            dot = d;
                        }
                        parse.push(s.clone());
                    } else {
                        break;
                    }
                }
                Term::Terminal(_) => {
                    parse.push(State {
                        origin: None,
                        lhs: None,
                        terms: vec![term.clone()],
                        dot: None,
                    });
                }
            }
        }

        if let Some(prod) = parent {
            if let Some(lhs) = prod.lhs {
                println!("\n{} ::= {:?}", lhs, prod.terms);
            }
        }

        for p in parse.iter().rev() {
            if let &Some(ref lhs) = &p.lhs {
                if let &Some(ref origin) = &p.origin {
                    println!("  | {} ::= {:?} (origin: {})", *lhs, p.terms, origin);
                }
            } else {
                println!("  | {:?}", p.terms);
            }
        }

        println!("\n");

        // for p in parse {
        //     if let Some(_) = p.lhs {
        //         recurse(states, p.terms.clone(), Some(p));
        //     }
        // }
    }

    //****************

    //****************

    // for (i, state) in states.iter().enumerate() {
    //     println!("\n---S({})\n", i);
    //     for (_, production) in state.iter().enumerate() {
    //         let finished: String;
    //         if let None = earley_next_element(production) {
    //             finished = String::from("(complete)");
    //         } else {
    //             finished = String::from("");
    //         }
    //         println!(
    //             "{} | {} -> {:?} - dot:{} {}",
    //             production.origin, production.lhs, production.terms, production.dot, finished
    //         );
    //     }
    // }

    //****************
}
