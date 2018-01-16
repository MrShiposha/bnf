use grammar::Grammar;
use production::Production;
use expression::Expression;
use term::Term;

#[derive(PartialEq, Debug, Clone)]
pub enum State {
    grammar(Grammar),
    production(Production),
    expression(Expression),
    term(Term),
}

#[derive(PartialEq, Debug, Clone)]
pub struct StateSet {
    pub state: Vec<State>,
    pub dot: Term,
    pub origin: usize,
}

impl StateSet {
    pub fn new() -> StateSet {
        StateSet {
            state: vec![],
            dot: Term::Nonterminal(String::new()),
            origin: 0,
        }
    }
}