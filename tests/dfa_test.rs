#[cfg(test)]
mod tests {
    use automata::expression::Expression;

    #[test]
    fn match_plain_text() {
        assert!(Expression::new("abc").matches("abc"));
        assert!(!Expression::new("abc").matches("a"));
        assert!(!Expression::new("abc").matches("aa"));
        assert!(!Expression::new("abc").matches("b"));
        assert!(!Expression::new("abc").matches("c"));
        assert!(!Expression::new("abc").matches("ab"));
        assert!(!Expression::new("abc").matches("abca"));
    }
}