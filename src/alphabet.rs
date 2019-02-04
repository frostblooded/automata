use std::collections::HashSet;

pub fn get_english() -> HashSet<char> {
    (97..122).chain(65..90).map(char::from).collect()
}