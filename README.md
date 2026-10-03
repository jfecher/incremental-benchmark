Small benchmark attempting to compare different libraries for incremental computations.

## Benchmarks

- Fib: A recursive fibonacci function where each call is cached.
- Update used input: Measures the time to re-run 1000 operations 1000 times after an input used by each is changed each time.
- Update unused input: Measures the time to run 1000 operations 1000 times after an unused input is changed each time. Since the input is not used in these operations, the operation should already be cached. This measure the time for each framework to recur on each operation to see that all inputs used are unchanged.

## Timings

Current timings on my M1 Mac:

```
$ cargo run
fib-inc                  : 1.843s
fib-salsa                : 3.692s
fib-salsa-raw-u32        : 3.690s
update-used-input-inc    : 2.756s
update-used-input-salsa  : 7.146s
update-unused-input-inc  : 624.286ms
update-unused-input-salsa: 2.145s
accumulate-inc           : 8.499s
accumulate-inc-uncached  : 3.946s
accumulate-salsa         : 8.814s
accumulate-repeat-inc    : 72.172ms
accumulate-repeat-inc-uncached: 601.584ms
accumulate-repeat-salsa  : 1.347s

$ cargo run --release
fib-inc                  : 94.547ms
fib-salsa                : 163.155ms
fib-salsa-raw-u32        : 173.800ms
update-used-input-inc    : 120.207ms
update-used-input-salsa  : 466.095ms
update-unused-input-inc  : 47.669ms
update-unused-input-salsa: 66.819ms
accumulate-inc           : 543.754ms
accumulate-inc-uncached  : 228.786ms
accumulate-salsa         : 379.721ms
accumulate-repeat-inc    : 3.699ms
accumulate-repeat-inc-uncached: 30.456ms
accumulate-repeat-salsa  : 53.092ms
```

Some notes:
- `accumulate-inc`: inc-complete's `Db::get_accumulated` is cached by default in inc-complete (reusing the same mechanism for queries), and is generally slower for it. Unlike `Db::get_accumulated_uncached` and salsa though, it can be safely used within queries.
- `accumulate-repeat`: a version of the accumulated test but reusing the same database object. This shows how the caching behavior of `Db::get_accumulated` can sometimes be faster.
- Generally inc-complete is faster in large part to it using monomorphized traits where salsa uses existentials. This is a compile-time tradeoff as well.
