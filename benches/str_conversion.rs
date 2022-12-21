use criterion::{Criterion, BenchmarkId};

fn str_to_owned(s: &str) -> String {
    s.to_owned()
}

fn str_to_string(s: &str) -> String {
    s.to_string()
}

fn string_from(s: &str) -> String {
    String::from(s)
}

pub fn bench_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("str_literal_conversion");

    for i in ["short string", "long string: U0eXV8aOYRRXHjmoLb9qYgxN3NpkjzhKmofVJVq1ZnhlC8DBCn5DMbnB90XkYMCA"].into_iter() {
        group.bench_with_input(BenchmarkId::new("to_owned", i), i, |b, i| b.iter(|| str_to_owned(i)));
        group.bench_with_input(BenchmarkId::new("to_string", i), i, |b, i| b.iter(|| str_to_string(i)));
        group.bench_with_input(BenchmarkId::new("string_from", i), i, |b, i| b.iter(|| string_from(i)));
    }

    group.finish();
}