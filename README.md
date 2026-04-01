Small benchmark attempting to compare different libraries for incremental computations.

## Benchmarks

- Fib: A recursive fibonacci function where each call is cached.
- Update used input: Measures the time to re-run 1000 operations 1000 times after an input used by each is changed each time.
- Update unused input: Measures the time to run 1000 operations 1000 times after an unused input is changed each time. Since the input is not used in these operations, the operation should already be cached. This measure the time for each framework to recur on each operation to see that all inputs used are unchanged.

## Timings

Current timings on my M1 Mac:

```
$ cargo run
fib-inc                  : 3.700s
fib-salsa                : 4.617s
fib-salsa-raw-u32        : 4.603s
update-used-input-inc    : 8.325s
update-used-input-salsa  : 6.797s
update-unused-input-inc  : 13.386ms
update-unused-input-salsa: 1.588s
accumulate-inc           : 17.500s
accumulate-inc-uncached  : 8.483s
accumulate-salsa         : 8.340s
accumulate-repeat-inc    : 75.808ms
accumulate-repeat-inc-uncached: 1.320s
accumulate-repeat-salsa  : 1.338s

$ cargo run --release
fib-inc                  : 167.378ms
fib-salsa                : 219.545ms
fib-salsa-raw-u32        : 223.521ms
update-used-input-inc    : 363.602ms
update-used-input-salsa  : 453.163ms
update-unused-input-inc  : 734.167µs
update-unused-input-salsa: 91.087ms
accumulate-inc           : 949.743ms
accumulate-inc-uncached  : 432.848ms
accumulate-salsa         : 366.509ms
accumulate-repeat-inc    : 4.229ms
accumulate-repeat-inc-uncached: 63.258ms
accumulate-repeat-salsa  : 47.670ms
```

Some notes:
- `update-used-input`: inc-complete's used inputs optimization slows down the general case here in debug mode
- `update-unused-input`: inc-complete is significantly faster than salsa in debug & release due to the above optimization. Untested: using `Durability`s other than the default in salsa may improve its time here.
- `accumulate-inc`: inc-complete's `Db::get_accumulated` is cached by default in inc-complete (reusing the same mechanism for queries), and is generally slower for it. Unlike `Db::get_accumulated_uncached` and salsa though, it can be safely used within queries.
- `accumulate-repeat`: a version of the accumulated test but reusing the same database object. This shows how the caching behavior of `Db::get_accumulated` can sometimes be faster.
