use crate::automaton::Automaton;
use crate::transition::Transition;
use crate::counter::Counter;

use std::collections::{BTreeSet, BTreeMap};

// Build sets easily for easy testing and comparing
macro_rules! set {
    [$($x:expr),+] => {
        [$($x,)+].iter().map(|x| x.clone()).collect()
    }
}

pub struct Minimizer {
    automaton: Automaton
}    

impl Minimizer {
    pub fn new(new_automaton: Automaton) -> Self {
        Minimizer {
            automaton: new_automaton
        }
    }

    fn find_group_with_state(groups: &BTreeMap<u32, BTreeMap<u32, BTreeMap<char, u32>>>, state: u32) -> Option<u32>{
        for (group_id, group) in groups {
            if group.get(&state).is_some() {
                return Some(*group_id);
            }
        }

        None
    }

    // This function assumes that the automaton is deterministic.
    pub fn minimize(&mut self) {
        // There are the diffent groups that states are being split into during the steps of the
        // minimization process. The current_groups hash has the following structure:
        //
        // {
        //      0: {
        //          0: {
        //              'a': 0,
        //              'b': 1
        //          }
        //      },
        //      1: {
        //          2: {
        //              'a': 0,
        //              'b': 0
        //          }
        //      },
        // }
        // This means that there are two groups. Group 0 and group 1 (the outermost hash elements).
        // The first group has only the state 0 and it has a transition with the letter 'a' into a
        // state of group 0 and transition with the letter 'b' into a state of group 1.
        // Group 1 is analogous.
        //
        // Another example: {0: {1: {'a': 0, 'b': 1}, 3: {'b': 1, 'a': 0}}, 1: {0: {'a': 0, 'b': 1}, 2: {'a': 0, 'b': 1}}}
        let mut current_groups = BTreeMap::<u32, BTreeMap<u32, BTreeMap<char, u32>>>::new();
        let mut counter = Counter::new();

        // At the start we create two groups. One of them has all final states and the other
        // one has all other states.
        let mut other_group = BTreeMap::<u32, BTreeMap<char, u32>>::new();
        let mut final_group = BTreeMap::<u32, BTreeMap<char, u32>>::new();

        for s in self.automaton.states.difference(&self.automaton.final_states) {
            other_group.insert(*s, BTreeMap::new());
        }

        for f in &self.automaton.final_states {
            final_group.insert(*f, BTreeMap::new());
        }

        current_groups.insert(counter.tick(), other_group);
        current_groups.insert(counter.tick(), final_group);

        let mut prev_groups = BTreeMap::new();

        // Now we spit into the next groups
        // We look for states withing the groups that have the same transitions and group them into new groups
        while prev_groups != current_groups {
            prev_groups = current_groups;
            let mut prev_groups_with_transitions = prev_groups.clone();
            current_groups = BTreeMap::new();
            counter.reset();

            let mut state_group_ids = BTreeMap::<u32, u32>::new();

            for (_group_id, group) in &prev_groups_with_transitions {
                for (state, _state_transitions) in group {
                    let group_with_state_id: u32 = Minimizer::find_group_with_state(&prev_groups, *state).unwrap();
                    state_group_ids.insert(*state, group_with_state_id);
                }
            }

            for (_group_id, group) in &mut prev_groups_with_transitions {
                for (state, state_transitions) in group {
                    for letter in &self.automaton.alphabet {
                        // This will be a single state if the automaton is deterministic
                        let reachable_state: u32 = *self.automaton.reachable(*state, Some(*letter)).iter().nth(0).unwrap();
                        let group_with_state_id = state_group_ids.get(&reachable_state).unwrap();
                        state_transitions.insert(*letter, *group_with_state_id);
                    }
                }
            }

            for (_group_id, group) in &prev_groups_with_transitions {
                // Find states with same transitions
                let mut states_with_same_key = BTreeMap::<BTreeMap<char, u32>, BTreeSet<u32>>::new();

                for (state, state_transitions) in group {
                    match states_with_same_key.get_mut(state_transitions) {
                        Some(val) => val.insert(*state),
                        None      => states_with_same_key.insert(state_transitions.clone(), set![*state]).is_some()
                    };
                }

                for (_state_transition, states) in &states_with_same_key {
                    let mut new_group = BTreeMap::new();

                    for state in states {
                        new_group.insert(*state, BTreeMap::new());
                    }

                    current_groups.insert(counter.tick(), new_group);
                }
            }
        }

        let mut state_group_ids = BTreeMap::<u32, u32>::new();

        for (_group_id, group) in &current_groups {
            for (state, _state_transitions) in group {
                let group_with_state_id: u32 = Minimizer::find_group_with_state(&prev_groups, *state).unwrap();
                state_group_ids.insert(*state, group_with_state_id);
            }
        }

        for (_group_id, group) in &mut current_groups {
            for (state, state_transitions) in group {
                for letter in &self.automaton.alphabet {
                    // This will be a single state if the automaton is deterministic
                    let reachable_state: u32 = *self.automaton.reachable(*state, Some(*letter)).iter().nth(0).unwrap();
                    let group_with_state_id = state_group_ids.get(&reachable_state).unwrap();
                    state_transitions.insert(*letter, *group_with_state_id);
                }
            }
        }

        let mut res_states = BTreeSet::<u32>::new();
        let mut res_transitions = BTreeSet::<Transition>::new();
        let mut res_final_states = BTreeSet::<u32>::new();
        let mut res_initial_states = BTreeSet::<u32>::new();

        for (group_id, group) in &current_groups {
            // TODO: Make this clearer
            let group_transitions = group.iter().nth(0).unwrap().1.clone();
            res_states.insert(*group_id);

            if group.iter().any(|(state, _)| self.automaton.final_states.contains(state)) {
                res_final_states.insert(*group_id);
            }

            if group.iter().any(|(state, _)| self.automaton.initial_states.contains(state)) {
                res_initial_states.insert(*group_id);
            }

            for (transition_letter, transition_to) in group_transitions {
                res_transitions.insert(Transition::new(*group_id, Some(transition_letter), transition_to));
            }
        }

        self.automaton.states = res_states;
        self.automaton.initial_states = res_initial_states;
        self.automaton.final_states = res_final_states;
        self.automaton.transitions = res_transitions;
        self.automaton.counter = counter;
    }

    pub fn take(self) -> Automaton {
        self.automaton
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimize_1() {
        // return;
        let mut automaton = Automaton::new();

        automaton.alphabet = set!['a', 'b'];
        automaton.states = set![0, 1, 2, 3];
        automaton.counter.value = 4;

        automaton.initial_states = set![0];

        automaton.final_states = set![1, 3];

        automaton.transitions = set![
            Transition::new(0, Some('a'), 1),
            Transition::new(0, Some('b'), 2),
            Transition::new(1, Some('a'), 1),
            Transition::new(1, Some('b'), 2),
            Transition::new(2, Some('a'), 3),
            Transition::new(2, Some('b'), 0),
            Transition::new(3, Some('a'), 3),
            Transition::new(3, Some('b'), 0)
        ];

        let mut minimizer = Minimizer::new(automaton);
        minimizer.minimize();
        automaton = minimizer.take();

        assert_eq!(automaton.states, set![0, 1]);
        assert_eq!(automaton.counter.value, 2);
        assert_eq!(automaton.initial_states, set![0]);
        assert_eq!(automaton.final_states, set![1]);
        assert_eq!(automaton.transitions, set![
            Transition::new(0, Some('a'), 1),
            Transition::new(0, Some('b'), 0),
            Transition::new(1, Some('a'), 1),
            Transition::new(1, Some('b'), 0)
        ]);
    }

    #[test]
    fn minimize_2() {
        let mut automaton = Automaton::new();

        automaton.alphabet = set!['a', 'b'];
        automaton.states = set![0, 1, 2, 3];
        automaton.counter.value = 4;

        automaton.initial_states = set![0];

        automaton.final_states = set![2];

        automaton.transitions = set![
            Transition::new(0, Some('a'), 1),
            Transition::new(0, Some('b'), 2),
            Transition::new(1, Some('a'), 2),
            Transition::new(1, Some('b'), 3),
            Transition::new(2, Some('a'), 1),
            Transition::new(2, Some('b'), 3),
            Transition::new(3, Some('a'), 2),
            Transition::new(3, Some('b'), 1)
        ];

        let mut minimizer = Minimizer::new(automaton);
        minimizer.minimize();
        automaton = minimizer.take();

        assert_eq!(automaton.states, set![0, 1, 2]);
        assert_eq!(automaton.counter.value, 3);
        assert_eq!(automaton.initial_states, set![0]);
        assert_eq!(automaton.final_states, set![2]);
        assert_eq!(automaton.transitions, set![
            Transition::new(0, Some('a'), 1),
            Transition::new(0, Some('b'), 2),
            Transition::new(1, Some('a'), 2),
            Transition::new(1, Some('b'), 1),
            Transition::new(2, Some('a'), 1),
            Transition::new(2, Some('b'), 1)
        ]);
    }
}
