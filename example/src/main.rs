extern crate bnf;

use bnf::Grammar;
use bnf::Production;
use bnf::Expression;
use bnf::Term;

// #[derive(PartialEq, Debug, Clone)]
// pub enum States {
//     GrammarNode(Grammar),
//     ProductionNode(Production),
//     ExpressionNode(Expression),
//     TermNode(Term),

// }

// #[derive(PartialEq, Debug, Clone)]
// pub struct Position {
//     pub dot: usize,
//     pub origin: usize,
// }

// #[derive(PartialEq, Debug, Clone)]
// pub struct State {
//     pub kind: States,
//     pub position: Position,
// }

// #[derive(PartialEq, Debug, Clone)]
// pub struct StateSet {
//     pub state: Vec<State>,
// }

// impl StateSet {
//     pub fn new() -> StateSet {
//         StateSet {
//             state: vec![],
//         }
//     }
// }

// fn earley_init(input_len: usize) -> Result<Vec<StateSet>, Error>{
//     let start_state: State;
//     match productions.first() {
//         Some(p) => {
//             start_state = State {
//                 kind: States::ProductionNode(p.clone()),
//                 position: Position { dot: 0, origin: 0 },
//             };
//         },
//         None => {
//             return Err(
//                 Error::ParseError(String::from(
//                     "No grammar to parse.")));
//         }
//     }

//     Ok(vec![StateSet{ state: vec![start_state] }])
// }

// fn earley_finished(
//     state: &State, input_len: usize) ->
//     bool
// {
//     return state.position.dot == input_len
// }

// fn earley_predictor(
//     _state: &State, _pos: usize) ->
//     Result<String, Error>
// {

//     for prod in productions.iter() {

//     }
//     return Ok(String::from("TODO: Predictor Success"))
// }

// fn earley_scanner(
//     _state: &State, _pos: usize, _words: &[u8]) ->
//     Result<String, Error>
// {
//     return Ok(String::from("TODO: Scanner Success"))
// }

// fn earley_completer(
//     _state: &State, _pos: usize) ->
//     Result<String, Error>
// {

//     return Ok(String::from("TODO: Completer Success"))
// }

// pub fn earley_parse(words: &[u8]) -> Result<String, Error> {
//     let input_len = words.len();
//     let state_set;
//     match earley_init(input_len) {
//         Ok(s) => state_set = s,
//         Err(e) => return Err(e),
//     }

//     for k in 0..input_len {
//         let set;
//         match state_set.iter().nth(k) {
//             Some(s) => {
//                 set = s;
//             },
//             None => {
//                 return
//                 Err(
//                     Error::ParseError(String::from(
//                         "Failed to get element '{}' from input")));
//             },
//         }

//         for state in &set.state {
//             if earley_finished(state, input_len) {
//                 return earley_completer(state, k);
//             }

//             match state.kind {
//                 States::TermNode(ref t) =>
//                 {
//                     match *t {
//                         Term::Terminal(_) => {
//                             let _ = earley_scanner(state, k, words);
//                         },
//                         Term::Nonterminal(_) => {
//                             let _ = earley_predictor(state, k);
//                         }
//                     }
//                 }
//                 _ => {
//                     let _ = earley_predictor(state, k);
//                 },
//             }
//         }

//     }

//     return Ok(String::from("Completed!"))
// }

fn find_rule(grammar: &Grammar, term: Term) -> Vec<Production> {
    let mut matches: Vec<Production> = vec![];
    for prod in grammar.productions_iter() {
        if prod.lhs == term {
            matches.push(prod.clone());
        }
    }

    matches
}

fn scan_terminals(grammar: &Grammar, terminal: Term, matches: &mut Vec<Term>) {
    for prod in grammar.productions_iter() {
        for expr in prod.rhs_iter() {
            for term in expr.terms_iter() {
                if let Term::Terminal(_) = *term {
                    if *term == terminal {
                        if let None = matches.iter().find(|&&ref x| x == &prod.lhs) {
                            matches.push(prod.lhs.clone());
                        }
                        println!("matched {} with rule {}", term, prod.lhs);
                    }
                }
            }
        }
    }
}

fn main() {
    let input =
        r#"
        <Rule1> ::= <Rule2> | <Rule2> <Rule1>
        <Rule2> ::= "ABC" | "AB" | "BC" | "AC" | "AG" | "T"
        <Rule3> ::= "AB" | "BC" | "AC" | "AG" | "T"
        <Rule4> ::= "BC" | "AC"
        "#;

    let grammar = Grammar::from_str(input).unwrap();

    let term = Term::Nonterminal(String::from("Rule2"));
    let mut matches: Vec<Term> = vec![];

    let input = String::from("ABCACT");

    let mut pattern: String = String::new();
    for i in 0..input.len() {
        for(_, c) in input[i..].chars().enumerate() {
            pattern.push(c);
            let new_term = Term::Terminal(pattern.clone());
            scan_terminals(&grammar, new_term, &mut matches);
        }
        pattern.clear();
    }

    // matches.sort();
    println!("matches: {:?}", matches);
}
