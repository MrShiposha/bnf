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

fn main() {
    let input =
        "<dna> ::= <base> | <base> <dna>
        <base> ::= \"A\" | \"C\" | \"G\" | \"T\"";
    let grammar = Grammar::from_str(input).unwrap();

    for prod in grammar.productions_iter() {
        println!("{}", prod.lhs);
        print!("\t");
        for expr in prod.rhs_iter() {
            for term in expr.terms_iter() {
                print!(" {} ", term);
            }
        }
        println!("\n");
    }

}
