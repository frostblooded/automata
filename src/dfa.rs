use crate::transition::Transition;
use crate::counter::Counter;

use std::collections::HashSet;

#[derive(Debug)]
pub struct DFA {
    states: HashSet<u32>,
    transitions: HashSet<Transition>,
    final_states: HashSet<u32>,
    initial_states: HashSet<u32>,

    // Counter to track what the next state's id will be
    counter: Counter
}

impl DFA {
    pub fn new() -> Self {
        DFA {
            states: HashSet::new(),
            transitions: HashSet::new(),
            final_states: HashSet::new(),
            initial_states: HashSet::new(),
            counter: Counter::new()
        }
    }

    pub fn from_letter(letter: char) -> Self {
        let mut dfa = DFA::new();
        let state1 = dfa.counter.tick();
        let state2 = dfa.counter.tick();

        dfa.states.insert(state1);
        dfa.states.insert(state2);
        dfa.transitions.insert(Transition::new(state1, Some(letter), state2));

        dfa.initial_states.insert(state1);
        dfa.final_states.insert(state2);

        dfa
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Build sets easily for easy testing and comparing
    macro_rules! set {
        [$($x:expr),+] => {
            [$($x,)+].iter().map(|&x| x.clone()).collect()
        }
    }

    #[test]
    fn create_from_letter() {
        let dfa = DFA::from_letter('a');

        assert_eq!(dfa.states, set![0, 1]);
        assert_eq!(dfa.initial_states, set![0]);
        assert_eq!(dfa.final_states, set![1]);
        assert_eq!(dfa.transitions, set![Transition::new(0, Some('a'), 1)]);
    }
}