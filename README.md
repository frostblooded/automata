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
