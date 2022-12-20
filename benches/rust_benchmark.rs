mod str_conversion;
use str_conversion::bench_string;

use criterion::{criterion_group, criterion_main};


criterion_group!(benches, bench_string);
criterion_main!(benches);