use term::Term;
use production::Production;

#[derive(PartialEq, Debug, Clone)]
pub struct StateSet {
    pub production: Production,
    pub dot: Term,
    pub origin: usize,
}

impl StateSet {
    pub fn new() -> StateSet {
        StateSet {
            production: Production::new(),
            dot: Term::Nonterminal(String::new()),
            origin: 0,
        }
    }
}