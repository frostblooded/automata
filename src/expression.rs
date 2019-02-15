use crate::nfa::NFA;
use crate::minimizer::Minimizer;
use crate::determinizer::Determinizer;
use crate::transition::Transition;

pub struct Expression {
    nfa: NFA
}

impl Expression {
    pub fn new(string: &str) -> Self {
        let mut nfa = NFA::from_string(string);

        nfa = Determinizer::new(nfa).determinize().take();
        nfa = Minimizer::new(nfa).minimize().take();

        Expression {
            nfa: nfa
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_from_plain_string() {
        let expression = Expression::new("abc");
        let nfa = expression.nfa;

        assert_eq!(nfa.alphabet, set!['a', 'b', 'c']);
        assert_eq!(nfa.states, set![0, 1, 2, 3, 4]);
        assert_eq!(nfa.initial_states, set![1]);
        assert_eq!(nfa.final_states, set![4]);
        assert_eq!(nfa.transitions, set![
            Transition::new(0, Some('a'), 0),
            Transition::new(0, Some('b'), 0),
            Transition::new(0, Some('c'), 0),
            Transition::new(1, Some('a'), 2),
            Transition::new(1, Some('b'), 0),
            Transition::new(1, Some('c'), 0),
            Transition::new(2, Some('a'), 0),
            Transition::new(2, Some('b'), 3),
            Transition::new(2, Some('c'), 0),
            Transition::new(3, Some('a'), 0),
            Transition::new(3, Some('b'), 0),
            Transition::new(3, Some('c'), 4),
            Transition::new(4, Some('a'), 0),
            Transition::new(4, Some('b'), 0),
            Transition::new(4, Some('c'), 0)
        ]);
        assert_eq!(nfa.counter.value, 5);
    }
}
