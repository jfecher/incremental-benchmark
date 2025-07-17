Small benchmark attempting to compare different libraries for incremental computations.

## Benchmarks

- Fib: A recursive fibonacci function where each call is cached.

## Timings

Current timings on my M1 Mac:

```
$ cargo run
fib-inc                  : 3.510s
fib-salsa                : 5.812s
fib-salsa-raw-u32        : 5.961s
update-used-input-inc    : 8.409s
update-used-input-salsa  : 7.652s
update-unused-input-inc  : 12.680ms
update-unused-input-salsa: 1.857s

$ cargo run --release
fib-inc                  : 163.726ms
fib-salsa                : 270.835ms
fib-salsa-raw-u32        : 283.114ms
update-used-input-inc    : 353.958ms
update-used-input-salsa  : 518.638ms
update-unused-input-inc  : 694.375µs
update-unused-input-salsa: 88.334ms
```
Currently inc-complete is faster than salsa in each benchmark except for `update-used-input-inc` when compiled in debug mode
