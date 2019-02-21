use crate::nfa::NFA;
use crate::dfa::DFA;
use crate::transition::Transition;
use crate::counter::Counter;

use std::collections::{BTreeSet, BTreeMap};

pub(crate) struct Determinizer {
    nfa: NFA,
    dfa: DFA
}    

impl Determinizer {
    pub(crate) fn new(new_nfa: NFA) -> Self {
        Determinizer {
            nfa: new_nfa,
            dfa: DFA::new()
        }
    }

    pub(crate) fn determinize(mut self) -> Self {
        let mut res_final_states = BTreeSet::<u32>::new();
        let mut res_transitions = BTreeSet::<Transition<char>>::new();

        let initial_epsilon_closure: BTreeSet<u32> = self.epsilon_closure(&self.nfa.initial_states);

        let mut found_this_step: BTreeSet<BTreeSet<u32>> = set![initial_epsilon_closure.clone()];
        let mut found_last_step: BTreeSet<BTreeSet<u32>>;

        // While making the automaton deterministic, we are finding
        // sets of states, which are themselves the new states.
        // In the process of doing so, we need to have the state sets and
        // their respective ids stored somewhere.
        let mut found_set_states: BTreeMap<BTreeSet<u32>, u32> = BTreeMap::new();
        let mut set_states_counter = Counter::new();
        found_set_states.insert(initial_epsilon_closure.clone(), set_states_counter.tick());

        while !found_this_step.is_empty() {
            found_last_step = found_this_step.clone();
            found_this_step.clear();

            for state in &found_last_step {
                for letter in &self.nfa.alphabet {
                    let reachable_with_letter = self.reachable_from_set(state, Some(*letter));
                    let reachable_enclosed = self.epsilon_closure(&reachable_with_letter);

                    if !found_set_states.contains_key(&reachable_enclosed) {
                        found_set_states.insert(reachable_enclosed.clone(), set_states_counter.tick());
                        found_this_step.insert(reachable_enclosed.clone());
                    }

                    let found_state_id = found_set_states[&reachable_enclosed];
                    let state_id = found_set_states[state];
                    res_transitions.insert(Transition::new(state_id, *letter, found_state_id));
                }

                if !self.nfa.final_states.is_disjoint(state) {
                    let state_id = found_set_states[state];
                    res_final_states.insert(state_id);
                }
            }
        }

        let mut dfa = DFA::new();

        dfa.alphabet = self.nfa.alphabet.clone();
        dfa.states = found_set_states.values().cloned().collect();

        // The initial state is always the first state because the algorithm
        // starts working from it
        dfa.initial_state = Some(0);

        dfa.final_states = res_final_states;
        dfa.transitions = res_transitions;
        dfa.counter = set_states_counter;

        self.dfa = dfa;
        self
    }

    fn reachable_from_set(&self, start_states: &BTreeSet<u32>, wanted_label: Option<char>) -> BTreeSet<u32> {
        let mut res = BTreeSet::new();

        for state in start_states {
            res = res.union(&self.nfa.reachable(*state, wanted_label)).cloned().collect();
        }

        res
    }

    fn epsilon_closure(&self, starting_states: &BTreeSet<u32>) -> BTreeSet<u32> {
        let mut res = starting_states.clone();
        let mut found_this_step = starting_states.clone();
        let mut found_last_step: BTreeSet<u32>;

        while !found_this_step.is_empty() {
            found_last_step = found_this_step;
            found_this_step = BTreeSet::new();

            for state in found_last_step {
                let epsilon_reachable = self.nfa.reachable(state, None);

                for reached_state in &epsilon_reachable {
                    if !res.contains(reached_state) {
                        res.insert(*reached_state);
                        found_this_step.insert(*reached_state);
                    }
                }
            }
        }

        res
    }

    pub(crate) fn take(self) -> DFA {
        self.dfa
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epsilon_closure() {
        let mut nfa = NFA::new();
        nfa.states.insert(0);
        nfa.states.insert(1);
        nfa.states.insert(2);
        nfa.counter.value = 3;
        nfa.transitions.insert(Transition::new(0, Some('a'), 1));
        nfa.transitions.insert(Transition::new(1, None, 2));
        nfa.transitions.insert(Transition::new(2, Some('b'), 0));

        let determinizer = Determinizer::new(nfa);

        assert_eq!(determinizer.epsilon_closure(&set![0]), set![0]);
        assert_eq!(determinizer.epsilon_closure(&set![0, 1]), set![0, 1, 2]);
        assert_eq!(determinizer.epsilon_closure(&set![1]), set![1, 2]);
    }

    #[test]
    fn determinize() {
        let mut nfa = NFA::new();

        nfa.alphabet = set!['a', 'b'];
        nfa.states = set![0, 1, 2];
        nfa.counter.value = 3;

        nfa.initial_states = set![2];
        nfa.final_states = set![0];

        nfa.transitions = set![
            Transition::new(0, Some('a'), 1),
            Transition::new(0, Some('b'), 2),
            Transition::new(0, None, 1),
            Transition::new(1, Some('b'), 1),
            Transition::new(1, None, 0),
            Transition::new(2, Some('a'), 2),
            Transition::new(2, Some('b'), 1)
        ];

        let dfa = Determinizer::new(nfa).determinize().take();

        assert_eq!(dfa.alphabet, set!['a', 'b']);
        assert_eq!(dfa.states, set![0, 1, 2]);
        assert_eq!(dfa.counter.value, 3);
        assert_eq!(dfa.initial_state, Some(0));
        assert_eq!(dfa.final_states, set![1, 2]);
        assert_eq!(dfa.transitions, set![
            Transition::new(0, 'a', 0),
            Transition::new(0, 'b', 1),
            Transition::new(1, 'a', 1),
            Transition::new(1, 'b', 2),
            Transition::new(2, 'a', 2),
            Transition::new(2, 'b', 2)
        ]);
    }
}
