use crate::transition::{Transition, State};

use std::collections::HashSet;
use std::hash::Hash;

pub struct DFA<S: Hash + PartialEq + Eq> {
    alphabet: HashSet<S>,
    states: HashSet<State>,
    transitions: HashSet<Transition<S>>,
    final_states: HashSet<State>,
    initial_states: HashSet<State>
}

impl<S: Hash + PartialEq + Eq> DFA<S> {
    pub fn new() -> Self {
        DFA {
            alphabet: HashSet::new(),
            states: HashSet::new(),
            transitions: HashSet::new(),
            final_states: HashSet::new(),
            initial_states: HashSet::new()
        }
    }
}