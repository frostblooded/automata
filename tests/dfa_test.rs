#[cfg(test)]
mod tests {
    use automata::expression::Expression;

    #[test]
    fn match_plain_text() {
        let expression = Expression::new("abc");

        assert!(expression.matches("abc"));
        assert!(!expression.matches("a"));
        assert!(!expression.matches("aa"));
        assert!(!expression.matches("b"));
        assert!(!expression.matches("c"));
        assert!(!expression.matches("ab"));
        assert!(!expression.matches("abca"));
    }

    #[test]
    fn match_text_with_optional_chars() {
        let expression = Expression::new("ab?c");

        assert!(expression.matches("ac"));
        assert!(expression.matches("abc"));

        assert!(!expression.matches("a"));
        assert!(!expression.matches("b"));
        assert!(!expression.matches("c"));
        assert!(!expression.matches("ab"));
        assert!(!expression.matches("aab"));
        assert!(!expression.matches("aa"));
    }

    #[test]
    fn match_text_with_kleene_chars() {
        let expression = Expression::new("ab*c");

        assert!(expression.matches("ac"));
        assert!(expression.matches("abc"));
        assert!(expression.matches("abbc"));
        assert!(expression.matches("abbbc"));

        assert!(!expression.matches("a"));
        assert!(!expression.matches("b"));
        assert!(!expression.matches("c"));
        assert!(!expression.matches("ab"));
        assert!(!expression.matches("aab"));
        assert!(!expression.matches("aa"));
    }

    #[test]
    fn match_text_with_plux_chars() {
        let expression = Expression::new("ab+c");

        assert!(expression.matches("abc"));
        assert!(expression.matches("abbc"));
        assert!(expression.matches("abbbc"));

        assert!(!expression.matches("ac"));
        assert!(!expression.matches("a"));
        assert!(!expression.matches("b"));
        assert!(!expression.matches("c"));
        assert!(!expression.matches("ab"));
        assert!(!expression.matches("aab"));
        assert!(!expression.matches("aa"));
    }
}