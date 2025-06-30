use std::time::Instant;

mod fib_inc;
mod fib_salsa;
mod fib_salsa_u32;

const FIB_INPUT: u32 = 425;
const ITERS: u32 = 1000;

fn main() {
    bench(ITERS, "fib-inc", fib_inc::bench_fib);
    bench(ITERS, "fib-salsa", fib_salsa::bench_fib);
    bench(ITERS, "fib-salsa-raw-u32", fib_salsa_u32::bench_fib);
}

fn bench(runs: u32, name: &str, f: impl Fn() -> u32) {
    let now = Instant::now();

    let mut total: u32 = 0;
    for _ in 0 .. runs {
        total = total.wrapping_add(f());
    }
    let elapsed = now.elapsed();

    println!("{:<20}: {total} in {:.3?}", name, elapsed);
}
