Small benchmark attempting to compare different libraries for incremental computations.

## Benchmarks

- Fib: A recursive fibonacci function where each call is cached.
- Update used input: Measures the time to re-run 1000 operations 1000 times after an input used by each is changed each time.
- Update unused input: Measures the time to run 1000 operations 1000 times after an unused input is changed each time. Since the input is not used in these operations, the operation should already be cached. This measure the time for each framework to recur on each operation to see that all inputs used are unchanged.

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
