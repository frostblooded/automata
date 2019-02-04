#[cfg(test)]
mod tests {
    use automata::dfa::*;

    #[test]
    fn create_empty() {
        DFA::<char>::new();
    }
}