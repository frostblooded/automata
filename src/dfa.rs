use crate::transition::Transition;
use crate::counter::Counter;

use std::collections::BTreeSet;

#[derive(Debug)]
pub(crate) struct DFA {
    pub(crate) alphabet: BTreeSet<char>,
    pub(crate) states: BTreeSet<u32>,
    pub(crate) transitions: BTreeSet<Transition<char>>,
    pub(crate) final_states: BTreeSet<u32>,
    pub(crate) initial_state: Option<u32>,

    // Counter to track what the next state's id will be
    pub(crate) counter: Counter
}

impl DFA {
    pub(crate) fn new() -> Self {
        DFA {
            alphabet: BTreeSet::new(),
            states: BTreeSet::new(),
            transitions: BTreeSet::new(),
            final_states: BTreeSet::new(),
            initial_state: None,
            counter: Counter::new()
        }
    }

    // Returns the states that are reachable by a state
    // through a specific transition
    pub(crate) fn reachable(&self, start_state: u32, wanted_label: char) -> Option<u32> {
        for transition in &self.transitions {
            if transition.from == start_state && transition.label == wanted_label {
                return Some(transition.to);
            }
        }

        None
    }

    pub(crate) fn matches(&self, text: &str) -> bool {
        let mut current_state;

        match self.initial_state {
            Some(state) => current_state = state,
            None        => return false
        };

        for ch in text.chars() {
            match self.reachable(current_state, ch) {
                Some(state) => current_state = state,
                None        => return false
            }
        }

        self.final_states.contains(&current_state)
    }
}
