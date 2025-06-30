#[salsa::db]
#[derive(Default, Clone)]
pub struct FibDbImpl {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for FibDbImpl {}

#[salsa::interned]
struct FibX<'db> {
    x: u32
}

#[salsa::interned]
struct Empty<'db> {}

// Need to go through some hoops to represent a recursive function.
// Calling FibX::new outside of a tracked function panics so we need
// to have a separate driver accepting this as input which just creates
// a FibX internally and forwards it to `fib`.
#[salsa::input]
struct FibInput {
    x: u32
}

/// Test if it is faster to accept an empty input which doesn't change along with
/// `x: u32` separately to avoid creating `FibX` for recursion parameters within `fib`.
#[salsa::tracked]
fn fib<'db>(db: &'db dyn salsa::Database, empty: Empty<'db>, x: u32) -> FibX<'db> {
    if x < 2 {
        FibX::new(db, x)
    } else {
        let x1 = fib(db, empty, x - 2).x(db);
        let x2 = fib(db, empty, x - 1).x(db);
        FibX::new(db, x1.wrapping_add(x2))
    }
}

#[salsa::tracked]
fn fib_driver<'db>(db: &'db dyn salsa::Database, input: FibInput) -> FibX<'db> {
    fib(db, Empty::new(db), input.x(db))
}

pub fn bench_fib() -> u32 {
    let db = FibDbImpl::default();
    let input = FibInput::new(&db, crate::FIB_INPUT);
    fib_driver(&db, input).x(&db)
}
