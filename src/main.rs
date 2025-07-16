use std::time::Instant;

mod fib;
mod update_input;

const FIB_INPUT: u32 = 425;
const ITERS: u32 = 1000;

fn main() {
    bench(ITERS, "fib-inc", fib::fib_inc::bench_fib);
    bench(ITERS, "fib-salsa", fib::fib_salsa::bench_fib);
    bench(ITERS, "fib-salsa-raw-u32", fib::fib_salsa_u32::bench_fib);
    bench(1, "update-input-inc", update_input::inc::bench);
    bench(1, "update-input-salsa", update_input::salsa::bench);
}

fn bench(runs: u32, name: &str, f: impl Fn() -> u32) {
    let now = Instant::now();

    let mut total: u32 = 0;
    for _ in 0..runs {
        total = total.wrapping_add(f());
    }
    let elapsed = now.elapsed();

    println!("{:<20}: {total} in {:.3?}", name, elapsed);
}
