use grammar::Grammar;
use production::Production;
use expression::Expression;
use term::Term;

#[derive(PartialEq, Debug, Clone)]
pub enum States {
    GrammarNode(Grammar),
    ProductionNode(Production),
    ExpressionNode(Expression),
    TermNode(Term),
    
}

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub dot: usize,
    pub origin: usize,    
}

#[derive(PartialEq, Debug, Clone)]
pub struct State {
    pub kind: States,
    pub position: Position,    
}

#[derive(PartialEq, Debug, Clone)]
pub struct StateSet {
    pub state: Vec<State>,
}

impl StateSet {
    pub fn new() -> StateSet {
        StateSet {
            state: vec![],
        }
    }
}