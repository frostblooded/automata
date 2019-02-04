use crate::transition::Transition;
use crate::counter::Counter;

use std::collections::HashSet;

#[derive(Debug)]
pub struct Automaton {
    states: HashSet<u32>,
    transitions: HashSet<Transition>,
    final_states: HashSet<u32>,
    initial_states: HashSet<u32>,

    // Counter to track what the next state's id will be
    counter: Counter
}

impl Automaton {
    pub fn new() -> Self {
        Automaton {
            states: HashSet::new(),
            transitions: HashSet::new(),
            final_states: HashSet::new(),
            initial_states: HashSet::new(),
            counter: Counter::new()
        }
    }

    pub fn from_letter(letter: char) -> Self {
        let mut automaton = Automaton::new();
        let state1 = automaton.counter.tick();
        let state2 = automaton.counter.tick();

        automaton.states.insert(state1);
        automaton.states.insert(state2);
        automaton.transitions.insert(Transition::new(state1, Some(letter), state2));

        automaton.initial_states.insert(state1);
        automaton.final_states.insert(state2);

        automaton
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
        let automaton = Automaton::from_letter('a');

        assert_eq!(automaton.states, set![0, 1]);
        assert_eq!(automaton.initial_states, set![0]);
        assert_eq!(automaton.final_states, set![1]);
        assert_eq!(automaton.transitions, set![Transition::new(0, Some('a'), 1)]);
    }
}