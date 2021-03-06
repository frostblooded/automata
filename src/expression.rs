use crate::nfa::NFA;
use crate::dfa::DFA;
use crate::minimizer::Minimizer;
use crate::determinizer::Determinizer;

pub struct Expression {
    dfa: DFA
}

impl Expression {
    pub fn new(string: &str) -> Self {
        let nfa = NFA::from_string(string);
        let dfa = Determinizer::new(nfa).determinize().take();
        let dfa = Minimizer::new(dfa).minimize().take();

        Expression {
            dfa
        }
    }

    pub fn matches(&self, text: &str) -> bool {
        self.dfa.matches(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transition::Transition;

    #[test]
    fn test_expression_from_plain_string() {
        let expression = Expression::new("abc");
        let dfa = expression.dfa;

        assert_eq!(dfa.alphabet, set!['a', 'b', 'c']);
        assert_eq!(dfa.states, set![0, 1, 2, 3, 4]);
        assert_eq!(dfa.initial_state, Some(1));
        assert_eq!(dfa.final_states, set![4]);
        assert_eq!(dfa.transitions, set![
            Transition::new(0, 'a', 0),
            Transition::new(0, 'b', 0),
            Transition::new(0, 'c', 0),
            Transition::new(1, 'a', 2),
            Transition::new(1, 'b', 0),
            Transition::new(1, 'c', 0),
            Transition::new(2, 'a', 0),
            Transition::new(2, 'b', 3),
            Transition::new(2, 'c', 0),
            Transition::new(3, 'a', 0),
            Transition::new(3, 'b', 0),
            Transition::new(3, 'c', 4),
            Transition::new(4, 'a', 0),
            Transition::new(4, 'b', 0),
            Transition::new(4, 'c', 0)
        ]);
        assert_eq!(dfa.counter.value, 5);
    }

    #[test]
    fn test_expression_from_string_with_optional_chars() {
        let expression = Expression::new("ab?c");
        let dfa = expression.dfa;

        assert_eq!(dfa.alphabet, set!['a', 'b', 'c']);
        assert_eq!(dfa.states, set![0, 1, 2, 3, 4]);
        assert_eq!(dfa.initial_state, Some(1));
        assert_eq!(dfa.final_states, set![4]);
        assert_eq!(dfa.transitions, set![
            Transition::new(0, 'a', 0),
            Transition::new(0, 'b', 0),
            Transition::new(0, 'c', 0),
            Transition::new(1, 'a', 3),
            Transition::new(1, 'b', 0),
            Transition::new(1, 'c', 0),
            Transition::new(2, 'a', 0),
            Transition::new(2, 'b', 0),
            Transition::new(2, 'c', 4),
            Transition::new(3, 'a', 0),
            Transition::new(3, 'b', 2),
            Transition::new(3, 'c', 4),
            Transition::new(4, 'a', 0),
            Transition::new(4, 'b', 0),
            Transition::new(4, 'c', 0)
        ]);
        assert_eq!(dfa.counter.value, 5);
    }

    #[test]
    fn test_expression_from_string_with_kleene_chars() {
        let expression = Expression::new("ab*c");
        let dfa = expression.dfa;

        assert_eq!(dfa.alphabet, set!['a', 'b', 'c']);
        assert_eq!(dfa.states, set![0, 1, 2, 3]);
        assert_eq!(dfa.initial_state, Some(1));
        assert_eq!(dfa.final_states, set![3]);
        assert_eq!(dfa.transitions, set![
            Transition::new(0, 'a', 0),
            Transition::new(0, 'b', 0),
            Transition::new(0, 'c', 0),
            Transition::new(1, 'a', 2),
            Transition::new(1, 'b', 0),
            Transition::new(1, 'c', 0),
            Transition::new(2, 'a', 0),
            Transition::new(2, 'b', 2),
            Transition::new(2, 'c', 3),
            Transition::new(3, 'a', 0),
            Transition::new(3, 'b', 0),
            Transition::new(3, 'c', 0)
        ]);
        assert_eq!(dfa.counter.value, 4);
    }

    #[test]
    fn test_expression_from_string_with_plus_chars() {
        let expression = Expression::new("ab+c");
        let dfa = expression.dfa;

        assert_eq!(dfa.alphabet, set!['a', 'b', 'c']);
        assert_eq!(dfa.states, set![0, 1, 2, 3, 4]);
        assert_eq!(dfa.initial_state, Some(1));
        assert_eq!(dfa.final_states, set![4]);
        assert_eq!(dfa.transitions, set![
            Transition::new(0, 'a', 0),
            Transition::new(0, 'b', 0),
            Transition::new(0, 'c', 0),
            Transition::new(1, 'a', 2),
            Transition::new(1, 'b', 0),
            Transition::new(1, 'c', 0),
            Transition::new(2, 'a', 0),
            Transition::new(2, 'b', 3),
            Transition::new(2, 'c', 0),
            Transition::new(3, 'a', 0),
            Transition::new(3, 'b', 3),
            Transition::new(3, 'c', 4),
            Transition::new(4, 'a', 0),
            Transition::new(4, 'b', 0),
            Transition::new(4, 'c', 0)
        ]);
        assert_eq!(dfa.counter.value, 5);
    }

    #[test]
    fn test_expression_from_string_with_or_chars() {
        let expression = Expression::new("ab|c");
        let dfa = expression.dfa;

        assert_eq!(dfa.alphabet, set!['a', 'b', 'c']);
        assert_eq!(dfa.states, set![0, 1, 2, 3]);
        assert_eq!(dfa.initial_state, Some(1));
        assert_eq!(dfa.final_states, set![3]);
        assert_eq!(dfa.transitions, set![
            Transition::new(0, 'a', 0),
            Transition::new(0, 'b', 0),
            Transition::new(0, 'c', 0),
            Transition::new(1, 'a', 2),
            Transition::new(1, 'b', 0),
            Transition::new(1, 'c', 3),
            Transition::new(2, 'a', 0),
            Transition::new(2, 'b', 3),
            Transition::new(2, 'c', 0),
            Transition::new(3, 'a', 0),
            Transition::new(3, 'b', 0),
            Transition::new(3, 'c', 0)
        ]);
        assert_eq!(dfa.counter.value, 4);
    }
}
