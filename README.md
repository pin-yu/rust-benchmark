# Rust Benchmark
This repo shows benchmark results for some Rust functions or features because I don't know which is better for the performance.

## Table of Content
- ["".to_owned vs "".to_string](#benchmark-1)

## Benchmark 1
"".to_owned vs "".to_string
- conclusion
    - the benchmark result shows that to_string has the same performance as to_owned, the absolute mean difference is less than 0.5 ns.
        - [report](http://htmlpreview.github.io/?https://github.com/pin-yu/rust-benchmark/blob/main/target/criterion/str_literal_conversion/report/index.html)
    - using to_string might be more intuitive in the latest Rust

- environment
    - rustc 1.65.0
    - Intel i9-9900K 3.6GHz 8 cores 16 threads
    - memory 16G
