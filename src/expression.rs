use crate::automaton::Automaton;
use crate::minimizer::Minimizer;
use crate::transition::Transition;

// Build sets easily for easy testing and comparing
macro_rules! set {
    [$($x:expr),+] => {
        [$($x,)+].iter().map(|x| x.clone()).collect()
    }
}

pub struct Expression {
    automaton: Automaton
}

impl Expression {
    pub fn new(string: &str) -> Self {
        let mut automaton = Automaton::from_string(string);

        automaton.determinize();

        let mut minimizer = Minimizer::new(automaton);
        minimizer.minimize();
        automaton = minimizer.take();

        Expression {
            automaton: automaton
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_from_plain_string() {
        let expression = Expression::new("abc");
        let automaton = expression.automaton;

        assert_eq!(automaton.alphabet, set!['a', 'b', 'c']);
        assert_eq!(automaton.states, set![0, 1, 2, 3, 4]);
        assert_eq!(automaton.initial_states, set![1]);
        assert_eq!(automaton.final_states, set![4]);
        assert_eq!(automaton.transitions, set![
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
        assert_eq!(automaton.counter.value, 5);
    }
}
