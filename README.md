# automata
[![Build Status](https://travis-ci.com/frostblooded/automata.svg?token=7tx66Lvcqspf6pYV3W7h&branch=master)](https://travis-ci.com/frostblooded/automata)

A Rust implementation of automata.

# Supported features
### Plain text matching
Expression "abc" matches:
- "abc"

### Optional character matching
Expression "ab?c" matches:
- "ac"
- "abc"

### Kleene character matching
Expression "a*c" matches:
- "c"
- "ac"
- "aac"
- "aaac"
- and so on...

### Plus character matching
Expression "ca+b" matches:
- "cab"
- "caab"
- "caaab"
- and so on...

### Or character matching
Expression "ab|ca" matches:
- "ab"
- "ca"

# Usage
To use the crate, you need to use the `automata::expression::Expression` struct.

### Examples:
```rust
let expression = Expression::new("b|ac");

assert!(expression.matches("b"));
assert!(expression.matches("ac"));

assert!(!expression.matches("a"));
assert!(!expression.matches("c"));
assert!(!expression.matches("ab"));
assert!(!expression.matches("bac"));
assert!(!expression.matches("ba"));
assert!(!expression.matches("abc"));
```

```rust
let expression = Expression::new("Ivan|Petq");

assert!(expression.matches("Ivan"));
assert!(expression.matches("Petq"));
assert!(!expression.matches("Petar"));
assert!(!expression.matches("Niki"));
```

```rust
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
```
