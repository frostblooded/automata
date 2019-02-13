use crate::transition::Transition;
use crate::counter::Counter;


use std::collections::{BTreeSet, BTreeMap};

// Build sets easily for easy testing and comparing
macro_rules! set {
    [$($x:expr),+] => {
        [$($x,)+].iter().map(|x| x.clone()).collect()
    }
}

#[derive(Debug)]
pub struct Automaton {
    alphabet: BTreeSet<char>,
    states: BTreeSet<u32>,
    transitions: BTreeSet<Transition>,
    final_states: BTreeSet<u32>,
    initial_states: BTreeSet<u32>,

    // Counter to track what the next state's id will be
    counter: Counter
}

impl Automaton {
    pub fn new() -> Self {
        Automaton {
            alphabet: BTreeSet::new(),
            states: BTreeSet::new(),
            transitions: BTreeSet::new(),
            final_states: BTreeSet::new(),
            initial_states: BTreeSet::new(),
            counter: Counter::new()
        }
    }

    pub fn from_letter(letter: char) -> Self {
        let mut automaton = Automaton::new();
        let state1 = automaton.counter.tick();
        let state2 = automaton.counter.tick();

        automaton.alphabet.insert(letter);
        automaton.states.insert(state1);
        automaton.states.insert(state2);
        automaton.transitions.insert(Transition::new(state1, Some(letter), state2));

        automaton.initial_states.insert(state1);
        automaton.final_states.insert(state2);

        automaton
    }

    fn union(&mut self, other: &Automaton) {
        self.shift_states(other.counter.value);
        self.counter.value += other.counter.value;

        self.alphabet = self.alphabet.union(&other.alphabet)
                                     .map(|&a| a)
                                     .collect();
        self.states = self.states.union(&other.states)
                                 .map(|&s| s)
                                 .collect();
        self.initial_states = self.initial_states.union(&other.initial_states)
                                                 .map(|&s| s)
                                                 .collect();
        self.final_states = self.final_states.union(&other.final_states)
                                             .map(|&s| s)
                                             .collect();
        self.transitions = self.transitions.union(&other.transitions)
                                           .map(|&s| s)
                                           .collect();
    }

    fn concat(&mut self, other: &Automaton) {
        self.shift_states(other.counter.value);
        self.counter.value += other.counter.value;

        self.alphabet = self.alphabet.union(&other.alphabet)
                                     .map(|&a| a)
                                     .collect();
        self.states = self.states.union(&other.states)
                                 .map(|&s| s)
                                 .collect();

        self.transitions = self.transitions.union(&other.transitions)
                                           .map(|&s| s)
                                           .collect();

        for f in &self.final_states {
            for i in &other.initial_states {
                self.transitions.insert(Transition::new(*f, None, *i));
            }
        }

        self.final_states = other.final_states.clone();
    }

    fn kleene(&mut self) {
        let new_initial_state = self.counter.tick();
        let new_final_state = self.counter.tick();

        self.states.insert(new_initial_state);
        self.states.insert(new_final_state);

        for f in &self.final_states {
            self.transitions.insert(Transition::new(*f, None, new_final_state));
        }

        for i in &self.initial_states {
            self.transitions.insert(Transition::new(new_initial_state, None, *i));
        }

        self.transitions.insert(Transition::new(new_final_state, None, new_initial_state));
        self.initial_states = set![new_initial_state];
        self.final_states = set![new_final_state];
    }

    fn shift_states(&mut self, amount: u32) {
        self.states = self.states.iter().map(|s| s + amount).collect();
        self.initial_states = self.initial_states.iter().map(|s| s + amount).collect();
        self.final_states = self.final_states.iter().map(|s| s + amount).collect();

        self.transitions = self.transitions.iter().map(|t|
            Transition::new(t.from + amount, t.label, t.to + amount)
        ).collect();
    }

    fn make_deterministic(&mut self) {
        let mut res_states: BTreeSet<BTreeSet<u32>>;
        let mut res_initial_states: BTreeSet<BTreeSet<u32>>;
        let mut res_final_states = BTreeSet::<u32>::new();
        let mut res_transitions = BTreeSet::<Transition>::new();

        let initial_epsilon_closure: BTreeSet<u32> = self.epsilon_closure(&self.initial_states).into();
        res_initial_states = set![initial_epsilon_closure];
        res_states = res_initial_states.clone();

        let mut found_this_step = res_initial_states.clone();
        let mut found_last_step: BTreeSet<BTreeSet<u32>>;

        // While making the automaton deterministic, we are finding
        // sets of states, which are themselves the new states.
        // In the process of doing so, we need to have the state sets and
        // their respective ids stored somewhere.
        let mut found_set_states: BTreeMap<BTreeSet<u32>, u32> = BTreeMap::new();
        let mut set_states_counter = Counter::new();

        for s in &res_initial_states {
            found_set_states.insert(s.clone(), set_states_counter.tick());
        }

        while found_this_step.len() > 0 {
            found_last_step = found_this_step.clone();
            found_this_step.clear();

            for s in &found_last_step {
                for a in &self.alphabet{
                    let reachable_with_letter = self.reachable_from_set(s, Some(*a));
                    let reachable_enclosed = self.epsilon_closure(&reachable_with_letter).into();

                    if !res_states.contains(&reachable_enclosed) {
                        found_set_states.insert(reachable_enclosed.clone(), set_states_counter.tick());
                        res_states.insert(reachable_enclosed.clone());
                        
                        if !found_this_step.contains(&reachable_enclosed) {
                            found_this_step.insert(reachable_enclosed.clone());
                        }
                    }

                    let found_state_id = found_set_states.get(&reachable_enclosed).unwrap();
                    let state_id = found_set_states.get(s).unwrap();

                    res_transitions.insert(Transition::new(*state_id, Some(*a), *found_state_id));
                }

                if !self.final_states.is_disjoint(s) {
                    let state_id = found_set_states.get(s).unwrap();
                    res_final_states.insert(*state_id);
                }
            }
        }

        self.states = found_set_states.values().map(|&s| s).collect();
        self.final_states = res_final_states;
        self.transitions = res_transitions;
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
    fn minimize(&mut self) {
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

        for s in self.states.difference(&self.final_states) {
            other_group.insert(*s, BTreeMap::new());
        }

        for f in &self.final_states {
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

            for (group_id, group) in &prev_groups_with_transitions {
                for (state, state_transitions) in group {
                    let group_with_state_id: u32 = Automaton::find_group_with_state(&prev_groups, *state).unwrap();
                    state_group_ids.insert(*state, group_with_state_id);
                }
            }

            for (group_id, group) in &mut prev_groups_with_transitions {
                for (state, state_transitions) in group {
                    for letter in &self.alphabet {
                        // This will be a single state if the automaton is deterministic
                        let reachable_state: u32 = *self.reachable(*state, Some(*letter)).iter().nth(0).unwrap();
                        let group_with_state_id = state_group_ids.get(&reachable_state).unwrap();
                        state_transitions.insert(*letter, *group_with_state_id);
                    }
                }
            }

            for (group_id, group) in &prev_groups_with_transitions {
                // Find states with same transitions
                let mut states_with_same_key = BTreeMap::<BTreeMap<char, u32>, BTreeSet<u32>>::new();

                for (state, state_transitions) in group {
                    match states_with_same_key.get_mut(state_transitions) {
                        Some(val) => val.insert(*state),
                        None      => states_with_same_key.insert(state_transitions.clone(), set![*state]).is_some()
                    };
                }

                for (state_transition, states) in &states_with_same_key {
                    let mut new_group = BTreeMap::new();

                    for state in states {
                        new_group.insert(*state, BTreeMap::new());
                    }

                    current_groups.insert(counter.tick(), new_group);
                }
            }
        }

        let mut state_group_ids = BTreeMap::<u32, u32>::new();

        for (group_id, group) in &current_groups {
            for (state, state_transitions) in group {
                let group_with_state_id: u32 = Automaton::find_group_with_state(&prev_groups, *state).unwrap();
                state_group_ids.insert(*state, group_with_state_id);
            }
        }

        for (group_id, group) in &mut current_groups {
            for (state, state_transitions) in group {
                for letter in &self.alphabet {
                    // This will be a single state if the automaton is deterministic
                    let reachable_state: u32 = *self.reachable(*state, Some(*letter)).iter().nth(0).unwrap();
                    let group_with_state_id = state_group_ids.get(&reachable_state).unwrap();
                    state_transitions.insert(*letter, *group_with_state_id);
                }
            }
        }

        let mut res_states = BTreeSet::<u32>::new();
        let mut res_transitions = BTreeSet::<Transition>::new();
        let mut res_final_states = BTreeSet::<u32>::new();
        let mut res_initial_states = BTreeSet::<u32>::new();
        let mut res_counter = Counter::new();

        for (group_id, group) in &current_groups {
            // TODO: Make this clearer
            let group_transitions = group.iter().nth(0).unwrap().1.clone();
            res_states.insert(*group_id);

            if group.iter().any(|(state, _)| self.final_states.contains(state)) {
                res_final_states.insert(*group_id);
            }

            if group.iter().any(|(state, _)| self.initial_states.contains(state)) {
                res_initial_states.insert(*group_id);
            }

            for (transition_letter, transition_to) in group_transitions {
                res_transitions.insert(Transition::new(*group_id, Some(transition_letter), transition_to));
            }
        }

        self.states = res_states;
        self.initial_states = res_initial_states;
        self.final_states = res_final_states;
        self.transitions = res_transitions;
        self.counter = res_counter;
    }

    // Returns the states that are reachable by a state
    // through a specific transition
    fn reachable(&self, start_state: u32, wanted_label: Option<char>) -> BTreeSet<u32> {
        self.transitions.iter()
                        .filter(|s| s.from == start_state && s.label == wanted_label)
                        .map(|s| s.to)
                        .collect()
    }

    fn reachable_from_set(&self, start_states: &BTreeSet<u32>, wanted_label: Option<char>) -> BTreeSet<u32> {
        let mut res = BTreeSet::<u32>::new();

        for s in start_states {
            res = res.union(&self.reachable(*s, wanted_label)).map(|&s| s).collect();
        }

        res
    }

    fn epsilon_closure(&self, starting_states: &BTreeSet<u32>) -> BTreeSet<u32> {
        let mut res = starting_states.clone();
        let mut found_this_step = starting_states.clone();
        let mut found_last_step: BTreeSet<u32>;

        while found_this_step.len() > 0 {
            found_last_step = found_this_step.clone();
            found_this_step.clear();

            for s in found_last_step {
                let epsilon_reachable = self.reachable(s, None);

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_letter() {
        let automaton = Automaton::from_letter('a');

        assert_eq!(automaton.alphabet, set!['a']);
        assert_eq!(automaton.states, set![0, 1]);
        assert_eq!(automaton.initial_states, set![0]);
        assert_eq!(automaton.final_states, set![1]);
        assert_eq!(automaton.transitions, set![Transition::new(0, Some('a'), 1)]);
        assert_eq!(automaton.counter.value, 2);
    }

    #[test]
    fn union_automata() {
        let mut automaton1 = Automaton::from_letter('a');
        let automaton2 = Automaton::from_letter('b');

        automaton1.union(&automaton2);

        assert_eq!(automaton1.alphabet, set!['a', 'b']);
        assert_eq!(automaton1.states, set![0, 1, 2, 3]);
        assert_eq!(automaton1.initial_states, set![0, 2]);
        assert_eq!(automaton1.final_states, set![1, 3]);
        assert_eq!(automaton1.transitions, set![
            Transition::new(2, Some('a'), 3),
            Transition::new(0, Some('b'), 1)
        ]);
        assert_eq!(automaton1.counter.value, 4);
    }

    #[test]
    fn concat_automata() {
        let mut automaton1 = Automaton::from_letter('a');
        let automaton2 = Automaton::from_letter('b');

        automaton1.concat(&automaton2);

        assert_eq!(automaton1.alphabet, set!['a', 'b']);
        assert_eq!(automaton1.states, set![0, 1, 2, 3]);
        assert_eq!(automaton1.initial_states, set![2]);
        assert_eq!(automaton1.final_states, set![1]);
        assert_eq!(automaton1.transitions, set![
            Transition::new(2, Some('a'), 3),
            Transition::new(3, None, 0),
            Transition::new(0, Some('b'), 1)
        ]);
        assert_eq!(automaton1.counter.value, 4);
    }

    #[test]
    fn kleene_automata() {
        let mut automaton = Automaton::from_letter('a');

        automaton.kleene();

        assert_eq!(automaton.states, set![0, 1, 2, 3]);
        assert_eq!(automaton.initial_states, set![2]);
        assert_eq!(automaton.final_states, set![3]);
        assert_eq!(automaton.transitions, set![
            Transition::new(0, Some('a'), 1),
            Transition::new(1, None, 3),
            Transition::new(3, None, 2),
            Transition::new(2, None, 0)
        ]);
        assert_eq!(automaton.counter.value, 4);
    }

    #[test]
    fn shift_states() {
        let mut automaton = Automaton::from_letter('a');
        automaton.shift_states(2);

        assert_eq!(automaton.states, set![2, 3]);
        assert_eq!(automaton.initial_states, set![2]);
        assert_eq!(automaton.final_states, set![3]);
        assert_eq!(automaton.transitions, set![Transition::new(2, Some('a'), 3)]);
    }

    #[test]
    fn epsilon_closure() {
        let mut automaton = Automaton::new();
        automaton.states.insert(0);
        automaton.states.insert(1);
        automaton.states.insert(2);
        automaton.counter.value = 3;
        automaton.transitions.insert(Transition::new(0, Some('a'), 1));
        automaton.transitions.insert(Transition::new(1, None, 2));
        automaton.transitions.insert(Transition::new(2, Some('b'), 0));

        assert_eq!(automaton.epsilon_closure(&set![0]), set![0]);
        assert_eq!(automaton.epsilon_closure(&set![0, 1]), set![0, 1, 2]);
        assert_eq!(automaton.epsilon_closure(&set![1]), set![1, 2]);
    }

    #[test]
    fn make_deterministic() {
        let mut automaton = Automaton::new();

        automaton.alphabet = set!['a', 'b', 'c'];
        automaton.states = set![0, 1, 2];
        automaton.counter.value = 3;

        automaton.initial_states = set![0];

        automaton.final_states = set![2];

        automaton.transitions = set![
            Transition::new(0, Some('a'), 1),
            Transition::new(1, Some('b'), 0),
            Transition::new(1, None, 2),
            Transition::new(2, None, 1),
            Transition::new(2, Some('c'), 0)
        ];

        automaton.make_deterministic();

        assert_eq!(automaton.states, set![0, 1, 2]);
        assert_eq!(automaton.initial_states, set![0]);
        assert_eq!(automaton.final_states, set![1]);
        assert_eq!(automaton.transitions, set![
            Transition::new(0, Some('a'), 1),
            Transition::new(0, Some('b'), 2),
            Transition::new(0, Some('c'), 2),
            Transition::new(1, Some('a'), 2),
            Transition::new(1, Some('b'), 0),
            Transition::new(1, Some('c'), 0),
            Transition::new(2, Some('a'), 2),
            Transition::new(2, Some('b'), 2),
            Transition::new(2, Some('c'), 2)
        ]);
    }

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

        automaton.minimize();

        assert_eq!(automaton.states, set![0, 1]);
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

        automaton.minimize();

        assert_eq!(automaton.states, set![0, 1, 2]);
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