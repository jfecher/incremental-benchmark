Small benchmark attempting to compare different libraries for incremental computations.

## Benchmarks

- Fib: A recursive fibonacci function where each call is cached.

## Timings

Current timings on my machine:

```
$ cargo run
fib-inc                  : 2.508s
fib-salsa                : 3.354s
fib-salsa-raw-u32        : 3.504s
update-used-input-inc    : 5.964s
update-used-input-salsa  : 4.447s
update-unused-input-inc  : 8.815ms
update-unused-input-salsa: 1.059s

$ cargo run --release
fib-inc                  : 201.810ms
fib-salsa                : 219.763ms
fib-salsa-raw-u32        : 227.539ms
update-used-input-inc    : 401.201ms
update-used-input-salsa  : 486.190ms
update-unused-input-inc  : 702.451µs
update-unused-input-salsa: 81.960ms
```
Currently inc-complete is faster than salsa in each benchmark except for `update-used-input-inc` when compiled in debug mode
