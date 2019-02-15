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
    fn match_text_with_plus_chars() {
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

    #[test]
    fn match_text_with_or_chars() {
        let expression = Expression::new("b|ac");

        assert!(expression.matches("b"));
        assert!(expression.matches("ac"));

        assert!(!expression.matches("a"));
        assert!(!expression.matches("c"));
        assert!(!expression.matches("ab"));
        assert!(!expression.matches("bac"));
        assert!(!expression.matches("ba"));
        assert!(!expression.matches("abc"));
    }

    #[test]
    fn match_text_with_realistic_example_1() {
        let expression = Expression::new("Ivan|Petq");

        assert!(expression.matches("Ivan"));
        assert!(expression.matches("Petq"));
        assert!(!expression.matches("Petar"));
        assert!(!expression.matches("Niki"));
    }

    #[test]
    fn match_text_with_realistic_example_2() {
        let expression = Expression::new("a+bc*|ca*");

        assert!(expression.matches("ab"));
        assert!(expression.matches("abc"));
        assert!(expression.matches("aaabcc"));
        assert!(expression.matches("abc"));
        assert!(expression.matches("abcccc"));
        assert!(expression.matches("c"));
        assert!(expression.matches("ca"));
        assert!(expression.matches("caaa"));
        assert!(!expression.matches("b"));
        assert!(!expression.matches("bc"));
    }
}