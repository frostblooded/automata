use crate::dfa::DFA;
use crate::transition::Transition;
use crate::counter::Counter;

use std::collections::{BTreeSet, BTreeMap};

pub(crate) struct Minimizer {
    dfa: DFA
}    

impl Minimizer {
    pub(crate) fn new(new_dfa: DFA) -> Self {
        Minimizer {
            dfa: new_dfa
        }
    }

    fn find_group_with_state(groups: &BTreeMap<u32, BTreeMap<u32, BTreeMap<char, u32>>>, state: u32) -> Option<u32> {
        for (group_id, group) in groups {
            if group.get(&state).is_some() {
                return Some(*group_id);
            }
        }

        None
    }

    fn fill_group_transitions(&self, mut groups: BTreeMap<u32, BTreeMap<u32, BTreeMap<char, u32>>>) -> BTreeMap<u32, BTreeMap<u32, BTreeMap<char, u32>>> {
        let mut state_group_ids = BTreeMap::<u32, u32>::new();

        // Build a map that tells us which state is in which group
        for group in groups.values() {
            for state in group.keys() {
                let group_with_state_id: u32 = Minimizer::find_group_with_state(&groups, *state).expect("Invalid groups");
                state_group_ids.insert(*state, group_with_state_id);
            }
        }

        for group in groups.values_mut() {
            for (state, state_transitions) in group {
                for letter in &self.dfa.alphabet {
                    let reachable_state: u32 = self.dfa.reachable(*state, *letter).expect("Automaton is not total");
                    let group_with_state_id = state_group_ids[&reachable_state];
                    state_transitions.insert(*letter, group_with_state_id);
                }
            }
        }

        groups
    }

    fn build_dfa_from_groups(&mut self, groups: BTreeMap<u32, BTreeMap<u32, BTreeMap<char, u32>>>) {
        let mut res_states = BTreeSet::<u32>::new();
        let mut res_transitions = BTreeSet::<Transition<char>>::new();
        let mut res_final_states = BTreeSet::<u32>::new();
        self.dfa.counter.reset();

        // First pass through all groups and find the initial one.
        // This is done in a separate looping over the groups, because we want to
        // stop the iteration once we find the initial group.
        for (group_id, group) in &groups {
            // If the group has initial states, it will be initial in the new automaton
            if group.iter().any(|(&state, _)| self.dfa.initial_state.map_or(false, |x| x == state)) {
                self.dfa.initial_state = Some(*group_id);
                break;
            }
        }

        for (group_id, group) in &groups {
            // We get the group's first state and it's transitions, because those are
            // the transitions of the whole group.
            let group_transitions = group.iter().nth(0).expect("Empty group").1;
            res_states.insert(*group_id);

            // If the group has final states, it will be final in the new automaton
            if group.iter().any(|(state, _)| self.dfa.final_states.contains(state)) {
                res_final_states.insert(*group_id);
            }

            for (transition_letter, transition_to) in group_transitions {
                res_transitions.insert(Transition::new(*group_id, *transition_letter, *transition_to));
            }

            self.dfa.counter.value += 1;
        }

        self.dfa.states = res_states;
        self.dfa.final_states = res_final_states;
        self.dfa.transitions = res_transitions;
    }

    /*
    Group the states in a group that have the same transitions. For example

    {
        0: {'a': 0, 'b': 1},
        1: {'a': 1, 'b': 2},
        2: {'a': 0, 'b': 1}
    }

    would return

    {
        {'a': 0, 'b': 1}: {0, 2},
        {'a': 1, 'b': 2}: {1}
    }

    because states 0 and 2 have the same transitions with 'a' and 'b' to 0 and 1 respectively, while
    state 1's transitions are different.
    */
    fn find_states_with_same_transitions(group: &BTreeMap<u32, BTreeMap<char, u32>>) -> BTreeMap<BTreeMap<char, u32>, BTreeSet<u32>> {
        let mut res = BTreeMap::<BTreeMap<char, u32>, BTreeSet<u32>>::new();

        for (state, state_transitions) in group {
            match res.get_mut(state_transitions) {
                Some(val) => val.insert(*state),
                None      => res.insert(state_transitions.clone(), set![*state]).is_some()
            };
        }

        res
    }

    pub(crate) fn minimize(mut self) -> Self {
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

        // At the start we create two groups. One of them has all the final states and
        // the other has all other states.
        let mut other_group = BTreeMap::<u32, BTreeMap<char, u32>>::new();
        let mut final_group = BTreeMap::<u32, BTreeMap<char, u32>>::new();

        for s in self.dfa.states.difference(&self.dfa.final_states) {
            other_group.insert(*s, BTreeMap::new());
        }

        for f in &self.dfa.final_states {
            final_group.insert(*f, BTreeMap::new());
        }

        current_groups.insert(counter.tick(), other_group);
        current_groups.insert(counter.tick(), final_group);

        let mut prev_groups = BTreeMap::new();

        // Now we spit into the next groups. We look for states withing the groups
        // that have the same transitions and group them into new groups.
        while prev_groups != current_groups {
            prev_groups = current_groups;
            let mut prev_groups_with_transitions = prev_groups.clone();
            current_groups = BTreeMap::new();
            counter.reset();

            prev_groups_with_transitions = self.fill_group_transitions(prev_groups_with_transitions);

            for group in prev_groups_with_transitions.values() {
                let states_with_same_transitions = Minimizer::find_states_with_same_transitions(&group);

                for states in states_with_same_transitions.values() {
                    let mut new_group = BTreeMap::new();

                    for state in states {
                        new_group.insert(*state, BTreeMap::new());
                    }

                    current_groups.insert(counter.tick(), new_group);
                }
            }
        }

        current_groups = self.fill_group_transitions(current_groups);
        self.build_dfa_from_groups(current_groups);

        self
    }

    pub(crate) fn take(self) -> DFA {
        self.dfa
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimize_1() {
        let mut dfa = DFA::new();

        dfa.alphabet = set!['a', 'b'];
        dfa.states = set![0, 1, 2, 3];
        dfa.counter.value = 4;

        dfa.initial_state = Some(0);

        dfa.final_states = set![1, 3];

        dfa.transitions = set![
            Transition::new(0, 'a', 1),
            Transition::new(0, 'b', 2),
            Transition::new(1, 'a', 1),
            Transition::new(1, 'b', 2),
            Transition::new(2, 'a', 3),
            Transition::new(2, 'b', 0),
            Transition::new(3, 'a', 3),
            Transition::new(3, 'b', 0)
        ];

        dfa = Minimizer::new(dfa).minimize().take();

        assert_eq!(dfa.states, set![0, 1]);
        assert_eq!(dfa.counter.value, 2);
        assert_eq!(dfa.initial_state, Some(0));
        assert_eq!(dfa.final_states, set![1]);
        assert_eq!(dfa.transitions, set![
            Transition::new(0, 'a', 1),
            Transition::new(0, 'b', 0),
            Transition::new(1, 'a', 1),
            Transition::new(1, 'b', 0)
        ]);
    }

    #[test]
    fn minimize_2() {
        let mut dfa = DFA::new();

        dfa.alphabet = set!['a', 'b'];
        dfa.states = set![0, 1, 2, 3];
        dfa.counter.value = 4;

        dfa.initial_state = Some(0);

        dfa.final_states = set![2];

        dfa.transitions = set![
            Transition::new(0, 'a', 1),
            Transition::new(0, 'b', 2),
            Transition::new(1, 'a', 2),
            Transition::new(1, 'b', 3),
            Transition::new(2, 'a', 1),
            Transition::new(2, 'b', 3),
            Transition::new(3, 'a', 2),
            Transition::new(3, 'b', 1)
        ];

        dfa = Minimizer::new(dfa).minimize().take();

        assert_eq!(dfa.states, set![0, 1, 2]);
        assert_eq!(dfa.counter.value, 3);
        assert_eq!(dfa.initial_state, Some(0));
        assert_eq!(dfa.final_states, set![2]);
        assert_eq!(dfa.transitions, set![
            Transition::new(0, 'a', 1),
            Transition::new(0, 'b', 2),
            Transition::new(1, 'a', 2),
            Transition::new(1, 'b', 1),
            Transition::new(2, 'a', 1),
            Transition::new(2, 'b', 1)
        ]);
    }
}
