use crate::transition::Transition;
use crate::counter::Counter;

use std::iter::Peekable;
use std::str::Chars;

use std::collections::BTreeSet;

#[derive(Debug)]
pub(crate) struct NFA {
    pub(crate) alphabet: BTreeSet<char>,
    pub(crate) states: BTreeSet<u32>,
    pub(crate) transitions: BTreeSet<Transition<Option<char>>>,
    pub(crate) final_states: BTreeSet<u32>,
    pub(crate) initial_states: BTreeSet<u32>,

    // Counter to track what the next state's id will be
    pub(crate) counter: Counter
}

impl NFA {
    pub(crate) fn new() -> Self {
        NFA {
            alphabet: BTreeSet::new(),
            states: BTreeSet::new(),
            transitions: BTreeSet::new(),
            final_states: BTreeSet::new(),
            initial_states: BTreeSet::new(),
            counter: Counter::new()
        }
    }

    pub(crate) fn from_char(letter: char) -> Self {
        let mut nfa = NFA::new();
        let state1 = nfa.counter.tick();
        let state2 = nfa.counter.tick();

        nfa.alphabet.insert(letter);
        nfa.states.insert(state1);
        nfa.states.insert(state2);
        nfa.transitions.insert(Transition::new(state1, Some(letter), state2));

        nfa.initial_states.insert(state1);
        nfa.final_states.insert(state2);

        nfa
    }

    pub(crate) fn from_optional_char(letter: char) -> Self {
        let mut nfa = NFA::new();
        let state1 = nfa.counter.tick();
        let state2 = nfa.counter.tick();

        nfa.alphabet.insert(letter);
        nfa.states.insert(state1);
        nfa.states.insert(state2);
        nfa.transitions.insert(Transition::new(state1, Some(letter), state2));
        nfa.transitions.insert(Transition::new(state1, None, state2));

        nfa.initial_states.insert(state1);
        nfa.final_states.insert(state2);

        nfa
    }

    pub(crate) fn from_plus_char(letter: char) -> Self {
        let mut nfa = NFA::new();
        let state1 = nfa.counter.tick();
        let state2 = nfa.counter.tick();

        nfa.alphabet.insert(letter);
        nfa.states.insert(state1);
        nfa.states.insert(state2);
        nfa.transitions.insert(Transition::new(state1, Some(letter), state2));
        nfa.transitions.insert(Transition::new(state2, Some(letter), state2));

        nfa.initial_states.insert(state1);
        nfa.final_states.insert(state2);

        nfa
    }

    // You pass the current char that is being processed to this function and the
    // chars iterator that is being iterated over. The function peeks into the next
    // chars and decides how to handle the current char.
    // For example if the next items in the iterator are normal letters, it returns
    // an automaton that matches only the current letter. If, however, the next char is
    // '?', it returns an automata that mathes both the current letter and it's absence,
    // so it optionally matches the current letter.
    fn from_chars_iterator(current_char: char, iterator: &mut Peekable<Chars>) -> NFA {
        match iterator.peek() {
            Some(&next) => {
                match next {
                    '?' => {
                        iterator.next();
                        NFA::from_optional_char(current_char)
                    },
                    '*' => {
                        iterator.next();

                        let mut automata = NFA::from_char(current_char);
                        automata.kleene();
                        automata
                    },
                    '+' => {
                        iterator.next();
                        NFA::from_plus_char(current_char)
                    },
                    _   => NFA::from_char(current_char)
                }
            },
            None => NFA::from_char(current_char)
        }
    }

    pub(crate) fn from_string(string: &str) -> Self {
        let mut nfa = NFA::new();

        // For each slice of the expression that is separated by the OR character.
        // (for example "ab|ca" would handle "ab" and "ca" separately and then union
        // the two automata)
        for or_slice in string.split('|') {
            let mut chars = or_slice.chars().peekable();
            let mut current_nfa;

            // We need to handle the first character separately, because otherwise the current NFA would
            // always be empty and concatenating anything to it would result in an automaton that
            // doesn't match anything.
            current_nfa = match chars.next() {
                Some(ch) => NFA::from_chars_iterator(ch, &mut chars),
                None => NFA::new()
            };

            while let Some(ch) = chars.next() {
                current_nfa.concat(&NFA::from_chars_iterator(ch, &mut chars));
            }

            nfa.union(&current_nfa);
        }

        nfa
    }

    pub(crate) fn union(&mut self, other: &NFA) {
        self.shift_states(other.counter.value);
        self.counter.value += other.counter.value;

        self.alphabet = self.alphabet.union(&other.alphabet).cloned().collect();
        self.states = self.states.union(&other.states).cloned().collect();
        self.initial_states = self.initial_states.union(&other.initial_states).cloned().collect();
        self.final_states = self.final_states.union(&other.final_states).cloned().collect();
        self.transitions = self.transitions.union(&other.transitions).cloned().collect();
    }

    pub(crate) fn concat(&mut self, other: &NFA) {
        self.shift_states(other.counter.value);
        self.counter.value += other.counter.value;

        self.alphabet = self.alphabet.union(&other.alphabet).cloned().collect();
        self.states = self.states.union(&other.states).cloned().collect();

        self.transitions = self.transitions.union(&other.transitions).cloned().collect();

        for f in &self.final_states {
            for i in &other.initial_states {
                self.transitions.insert(Transition::new(*f, None, *i));
            }
        }

        self.final_states = other.final_states.clone();
    }

    pub(crate) fn kleene(&mut self) {
        let new_initial_state = self.counter.tick();
        let new_final_state = self.counter.tick();

        self.states.insert(new_initial_state);
        self.states.insert(new_final_state);

        for f in &self.final_states {
            self.transitions.insert(Transition::new(*f, None, new_final_state));

            for i in &self.initial_states {
                self.transitions.insert(Transition::new(*f, None, *i));
            }
        }

        for i in &self.initial_states {
            self.transitions.insert(Transition::new(new_initial_state, None, *i));
        }

        self.transitions.insert(Transition::new(new_initial_state, None, new_final_state));
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
    pub(crate) fn reachable(&self, start_state: u32, wanted_label: Option<char>) -> BTreeSet<u32> {
        self.transitions.iter()
                        .filter(|s| s.from == start_state && s.label == wanted_label)
                        .map(|s| s.to)
                        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_char() {
        let nfa = NFA::from_char('a');

        assert_eq!(nfa.alphabet, set!['a']);
        assert_eq!(nfa.states, set![0, 1]);
        assert_eq!(nfa.initial_states, set![0]);
        assert_eq!(nfa.final_states, set![1]);
        assert_eq!(nfa.transitions, set![Transition::new(0, Some('a'), 1)]);
        assert_eq!(nfa.counter.value, 2);
    }

    #[test]
    fn create_from_optional_char() {
        let nfa = NFA::from_optional_char('a');

        assert_eq!(nfa.alphabet, set!['a']);
        assert_eq!(nfa.states, set![0, 1]);
        assert_eq!(nfa.initial_states, set![0]);
        assert_eq!(nfa.final_states, set![1]);
        assert_eq!(nfa.transitions, set![
            Transition::new(0, Some('a'), 1),
            Transition::new(0, None, 1)
        ]);
        assert_eq!(nfa.counter.value, 2);
    }

    #[test]
    fn create_from_plus_char() {
        let nfa = NFA::from_plus_char('a');

        assert_eq!(nfa.alphabet, set!['a']);
        assert_eq!(nfa.states, set![0, 1]);
        assert_eq!(nfa.initial_states, set![0]);
        assert_eq!(nfa.final_states, set![1]);
        assert_eq!(nfa.transitions, set![
            Transition::new(0, Some('a'), 1),
            Transition::new(1, Some('a'), 1)
        ]);
        assert_eq!(nfa.counter.value, 2);
    }

    #[test]
    fn create_from_plain_string() {
        let nfa = NFA::from_string("abc");

        assert_eq!(nfa.alphabet, set!['a', 'b', 'c']);
        assert_eq!(nfa.states, set![0, 1, 2, 3, 4, 5]);
        assert_eq!(nfa.initial_states, set![4]);
        assert_eq!(nfa.final_states, set![1]);
        assert_eq!(nfa.transitions, set![
            Transition::new(4, Some('a'), 5),
            Transition::new(5, None, 2),
            Transition::new(2, Some('b'), 3),
            Transition::new(3, None, 0),
            Transition::new(0, Some('c'), 1)
        ]);
        assert_eq!(nfa.counter.value, 6);
    }

    #[test]
    fn create_from_string_with_optional_chars() {
        let nfa = NFA::from_string("ab?");

        assert_eq!(nfa.alphabet, set!['a', 'b']);
        assert_eq!(nfa.states, set![0, 1, 2, 3]);
        assert_eq!(nfa.initial_states, set![2]);
        assert_eq!(nfa.final_states, set![1]);
        assert_eq!(nfa.transitions, set![
            Transition::new(2, Some('a'), 3),
            Transition::new(3, None, 0),
            Transition::new(0, Some('b'), 1),
            Transition::new(0, None, 1)
        ]);
        assert_eq!(nfa.counter.value, 4);
    }

    #[test]
    fn create_from_string_with_kleene_chars() {
        let nfa = NFA::from_string("ca*");

        assert_eq!(nfa.alphabet, set!['c', 'a']);
        assert_eq!(nfa.states, set![0, 1, 2, 3, 4, 5]);
        assert_eq!(nfa.initial_states, set![4]);
        assert_eq!(nfa.final_states, set![3]);
        assert_eq!(nfa.transitions, set![
            Transition::new(0, Some('a'), 1),
            Transition::new(1, None, 0),
            Transition::new(1, None, 3),
            Transition::new(2, None, 0),
            Transition::new(2, None, 3),
            Transition::new(4, Some('c'), 5),
            Transition::new(5, None, 2)
        ]);
        assert_eq!(nfa.counter.value, 6);
    }

    #[test]
    fn create_from_string_with_plus_chars() {
        let nfa = NFA::from_string("a+b");

        assert_eq!(nfa.alphabet, set!['a', 'b']);
        assert_eq!(nfa.states, set![0, 1, 2, 3]);
        assert_eq!(nfa.initial_states, set![2]);
        assert_eq!(nfa.final_states, set![1]);
        assert_eq!(nfa.transitions, set![
            Transition::new(0, Some('b'), 1),
            Transition::new(2, Some('a'), 3),
            Transition::new(3, None, 0),
            Transition::new(3, Some('a'), 3)
        ]);
        assert_eq!(nfa.counter.value, 4);
    }

    #[test]
    fn create_from_string_with_or_chars() {
        let nfa = NFA::from_string("a|b");

        assert_eq!(nfa.alphabet, set!['a', 'b']);
        assert_eq!(nfa.states, set![0, 1, 2, 3]);
        assert_eq!(nfa.initial_states, set![0, 2]);
        assert_eq!(nfa.final_states, set![1, 3]);
        assert_eq!(nfa.transitions, set![
            Transition::new(0, Some('b'), 1),
            Transition::new(2, Some('a'), 3)
        ]);
        assert_eq!(nfa.counter.value, 4);
    }

    #[test]
    fn union_automata() {
        let mut nfa1 = NFA::from_char('a');
        let nfa2 = NFA::from_char('b');

        nfa1.union(&nfa2);

        assert_eq!(nfa1.alphabet, set!['a', 'b']);
        assert_eq!(nfa1.states, set![0, 1, 2, 3]);
        assert_eq!(nfa1.initial_states, set![0, 2]);
        assert_eq!(nfa1.final_states, set![1, 3]);
        assert_eq!(nfa1.transitions, set![
            Transition::new(2, Some('a'), 3),
            Transition::new(0, Some('b'), 1)
        ]);
        assert_eq!(nfa1.counter.value, 4);
    }

    #[test]
    fn concat_automata() {
        let mut nfa1 = NFA::from_char('a');
        let nfa2 = NFA::from_char('b');

        nfa1.concat(&nfa2);

        assert_eq!(nfa1.alphabet, set!['a', 'b']);
        assert_eq!(nfa1.states, set![0, 1, 2, 3]);
        assert_eq!(nfa1.initial_states, set![2]);
        assert_eq!(nfa1.final_states, set![1]);
        assert_eq!(nfa1.transitions, set![
            Transition::new(2, Some('a'), 3),
            Transition::new(3, None, 0),
            Transition::new(0, Some('b'), 1)
        ]);
        assert_eq!(nfa1.counter.value, 4);
    }

    #[test]
    fn kleene_automata() {
        let mut nfa = NFA::from_char('a');

        nfa.kleene();

        assert_eq!(nfa.states, set![0, 1, 2, 3]);
        assert_eq!(nfa.initial_states, set![2]);
        assert_eq!(nfa.final_states, set![3]);
        assert_eq!(nfa.transitions, set![
            Transition::new(0, Some('a'), 1),
            Transition::new(1, None, 0),
            Transition::new(1, None, 3),
            Transition::new(2, None, 0),
            Transition::new(2, None, 3)
        ]);
        assert_eq!(nfa.counter.value, 4);
    }

    #[test]
    fn shift_states() {
        let mut nfa = NFA::from_char('a');
        nfa.shift_states(2);

        assert_eq!(nfa.states, set![2, 3]);
        assert_eq!(nfa.initial_states, set![2]);
        assert_eq!(nfa.final_states, set![3]);
        assert_eq!(nfa.transitions, set![Transition::new(2, Some('a'), 3)]);
    }
}