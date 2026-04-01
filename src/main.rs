use std::time::Instant;

mod accumulate;
mod fib;
mod update_unused_input;
mod update_used_input;

const FIB_INPUT: u32 = 425;
const ITERS: u32 = 1000;

fn main() {
    bench(ITERS, "fib-inc", fib::inc::bench);
    bench(ITERS, "fib-salsa", fib::salsa::bench);
    bench(ITERS, "fib-salsa-raw-u32", fib::salsa_u32::bench_fib);
    bench(1, "update-used-input-inc", update_used_input::inc::bench);
    bench(
        1,
        "update-used-input-salsa",
        update_used_input::salsa::bench,
    );
    bench(
        1,
        "update-unused-input-inc",
        update_unused_input::inc::bench,
    );
    bench(
        1,
        "update-unused-input-salsa",
        update_unused_input::salsa::bench,
    );
    bench(1, "accumulate-inc", accumulate::inc::bench_accumulated);
    bench(
        1,
        "accumulate-inc-uncached",
        accumulate::inc::bench_accumulated_uncached,
    );
    bench(1, "accumulate-salsa", accumulate::salsa::bench);
    bench(
        1,
        "accumulate-repeat-inc",
        accumulate::inc::bench_accumulated_repeat,
    );
    bench(
        1,
        "accumulate-repeat-inc-uncached",
        accumulate::inc::bench_accumulated_repeat_uncached,
    );
    bench(
        1,
        "accumulate-repeat-salsa",
        accumulate::salsa::bench_repeat,
    );
}

fn bench(runs: u32, name: &str, f: impl Fn() -> u32) {
    let now = Instant::now();

    let mut total: u32 = 0;
    for _ in 0..runs {
        total = total.wrapping_add(std::hint::black_box(f()));
    }
    let elapsed = now.elapsed();

    println!("{:<25}: {:.3?}", name, elapsed);
}
