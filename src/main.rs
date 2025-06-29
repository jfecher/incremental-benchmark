use std::time::Instant;

mod fib_inc;
mod fib_salsa;

fn main() {
    bench(1000, fib_inc::bench_fib);
    bench(1000, fib_salsa::bench_fib);
}

fn bench(runs: u32, f: impl Fn() -> u32) {
    let now = Instant::now();

    let mut total: u32 = 0;
    for _ in 0 .. runs {
        total = total.wrapping_add(f());
    }
    let elapsed = now.elapsed();

    println!("result: {total} in {:.3?}", elapsed);
}
