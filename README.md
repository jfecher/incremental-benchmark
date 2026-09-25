Small benchmark attempting to compare different libraries for incremental computations.

## Benchmarks

- Fib: A recursive fibonacci function where each call is cached.
- Update used input: Measures the time to re-run 1000 operations 1000 times after an input used by each is changed each time.
- Update unused input: Measures the time to run 1000 operations 1000 times after an unused input is changed each time. Since the input is not used in these operations, the operation should already be cached. This measure the time for each framework to recur on each operation to see that all inputs used are unchanged.

## Timings

Current timings on my M1 Mac:

```
$ cargo run
fib-inc                  : 1.853s
fib-salsa                : 4.961s
fib-salsa-raw-u32        : 4.964s
update-used-input-inc    : 2.819s
update-used-input-salsa  : 7.147s
update-unused-input-inc  : 615.687ms
update-unused-input-salsa: 1.715s
accumulate-inc           : 8.589s
accumulate-inc-uncached  : 4.478s
accumulate-salsa         : 9.172s
accumulate-repeat-inc    : 64.777ms
accumulate-repeat-inc-uncached: 1.155s
accumulate-repeat-salsa  : 1.445s

$ cargo run --release
fib-inc                  : 95.816ms
fib-salsa                : 227.663ms
fib-salsa-raw-u32        : 232.213ms
update-used-input-inc    : 126.208ms
update-used-input-salsa  : 482.459ms
update-unused-input-inc  : 54.324ms
update-unused-input-salsa: 96.252ms
accumulate-inc           : 544.639ms
accumulate-inc-uncached  : 255.040ms
accumulate-salsa         : 390.672ms
accumulate-repeat-inc    : 3.687ms
accumulate-repeat-inc-uncached: 54.753ms
accumulate-repeat-salsa  : 52.944ms
```

Some notes:
- `accumulate-inc`: inc-complete's `Db::get_accumulated` is cached by default in inc-complete (reusing the same mechanism for queries), and is generally slower for it. Unlike `Db::get_accumulated_uncached` and salsa though, it can be safely used within queries.
- `accumulate-repeat`: a version of the accumulated test but reusing the same database object. This shows how the caching behavior of `Db::get_accumulated` can sometimes be faster.
- Generally inc-complete is faster in large part to it using monomorphized traits where salsa uses existentials. This is a compile-time tradeoff as well.
