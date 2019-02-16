#[macro_use]
extern crate criterion;

use criterion::Criterion;

use automata::expression::Expression;

fn match_text_with_realistic_example() {
    let expression = Expression::new("a+bc*d+e*s*ac+e*");

    assert!(expression.matches("abdac"));
    assert!(expression.matches("abcdac"));
    assert!(expression.matches("abcccdeeac"));
    assert!(expression.matches("abcccdesac"));
    assert!(!expression.matches("bc"));
    assert!(!expression.matches("abces"));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("expression generation and matching", |b| b.iter(|| match_text_with_realistic_example()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
