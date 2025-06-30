Small benchmark attempting to compare different libraries for incremental computations.

## Benchmarks

- Fib: A recursive fibonacci function where each call is cached.

## Timings

Current timings on my machine:

```
$ cargo run
fib-inc             : 1165289928 in 2.558s
fib-salsa           : 1165289928 in 3.309s
fib-salsa-raw-u32   : 1165289928 in 3.457s

$ cargo run --release
fib-inc             : 1165289928 in 195.846ms
fib-salsa           : 1165289928 in 216.730ms
fib-salsa-raw-u32   : 1165289928 in 226.609ms
```
