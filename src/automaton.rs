use crate::transition::Transition;
use crate::counter::Counter;
use crate::alphabet;

use std::collections::HashSet;

// Build sets easily for easy testing and comparing
macro_rules! set {
    [$($x:expr),+] => {
        [$($x,)+].iter().map(|&x| x.clone()).collect()
    }
}

#[derive(Debug)]
pub struct Automaton {
    alphabet: HashSet<char>,
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
            alphabet: alphabet::get_english(),
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

    pub fn union(&mut self, other: &Automaton) {
        self.shift_states(other.counter.value);
        self.counter.value += other.counter.value;

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

    pub fn concat(&mut self, other: &Automaton) {
        self.shift_states(other.counter.value);
        self.counter.value += other.counter.value;

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

    pub fn kleene(&mut self) {
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

    // Returns the states that are reachable by a state
    // through a specific transition
    fn reachable_with(&self, start_state: u32, wanted_label: Option<char>) -> HashSet<u32> {
        self.transitions.iter()
                        .filter(|s| s.from == start_state && s.label == wanted_label)
                        .map(|s| s.to)
                        .collect()
    }

    pub fn epsilon_closure(&self, starting_states: &HashSet<u32>) -> HashSet<u32> {
        let mut res = starting_states.clone();
        let mut found_this_step = starting_states.clone();
        let mut found_last_step: HashSet<u32>;

        while found_this_step.len() > 0 {
            found_last_step = found_this_step.clone();
            found_this_step.clear();

            for s in found_last_step {
                let epsilon_reachable = self.reachable_with(s, None);

                for reached_state in &epsilon_reachable {
                    res.insert(*reached_state);
                    found_this_step.insert(*reached_state);
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
}