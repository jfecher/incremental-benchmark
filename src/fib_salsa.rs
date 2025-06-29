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

// Need to go through some hoops to represent a recursive function.
// Calling FibX::new outside of a tracked function panics so we need
// to have a separate driver accepting this as input which just creates
// a FibX internally and forwards it to `fib`.
#[salsa::input]
struct FibInput {
    x: u32
}

#[salsa::tracked]
fn fib<'db>(db: &'db dyn salsa::Database, input: FibX<'db>) -> FibX<'db> {
    let x = input.x(db);

    if x < 2 {
        FibX::new(db, x)
    } else {
        let xm2 = FibX::new(db, x - 2);
        let xm1 = FibX::new(db, x - 1);
        let x1 = fib(db, xm2).x(db);
        let x2 = fib(db, xm1).x(db);
        FibX::new(db, x1.wrapping_add(x2))
    }
}

#[salsa::tracked]
fn fib_driver<'db>(db: &'db dyn salsa::Database, input: FibInput) -> FibX<'db> {
    let x = FibX::new(db, input.x(db));
    fib(db, x)
}

pub fn bench_fib() -> u32 {
    let db = FibDbImpl::default();
    let x = 425;

    let input = FibInput::new(&db, x);
    fib_driver(&db, input).x(&db)
}
