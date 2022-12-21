# Rust Benchmark
This repo shows benchmark results for some Rust functions and features because I wonder the better usage for the performance.

## Table of Content
- [Comparison among "".to_owned(), "".to_string(), String::from("")](#benchmark-1)

## Benchmark 1
### Comparison among "".to_owned(), "".to_string(), String::from("")
#### Conclusion
- Both to_string() and String::from("") use to_owned() under the hood.
- The benchmark result shows that they have similar performance.
    - [report](http://htmlpreview.github.io/?https://github.com/pin-yu/rust-benchmark/blob/main/target/criterion/str_literal_conversion/report/index.html)
- Using to_owned might be more Rust like.

#### Environment
    - rustc 1.65.0
    - Intel i9-9900K 3.6GHz 8 cores 16 threads
    - memory 16G
