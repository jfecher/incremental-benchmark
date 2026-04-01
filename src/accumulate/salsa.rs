use salsa::Accumulator;

#[salsa::db]
#[derive(Default, Clone)]
pub struct AccumulateDbImpl {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for AccumulateDbImpl {}

#[allow(dead_code)]
#[salsa::accumulator]
struct AccumulatedValue(u32);

#[salsa::input]
struct Input {
    x: u32,
}

#[salsa::interned]
struct TenDeps {
    i: u32,
}

#[salsa::interned]
struct HundredDeps {
    i: u32,
    j: u32,
}

#[salsa::interned]
struct SharedDeps {
    j: u32,
}

#[salsa::tracked]
fn root_impl<'db>(db: &'db dyn salsa::Database, input: Input) -> u32 {
    (0..10_u32)
        .map(|i| ten_impl(db, TenDeps::new(db, i), input))
        .sum()
}

#[salsa::tracked]
fn ten_impl<'db>(db: &'db dyn salsa::Database, ctx: TenDeps<'db>, input: Input) -> u32 {
    let unique: u32 = (0..50_u32)
        .map(|j| hundred_impl(db, HundredDeps::new(db, ctx.i(db), j), input))
        .sum();
    let shared: u32 = (0..50_u32)
        .map(|j| shared_impl(db, SharedDeps::new(db, j), input))
        .sum();
    unique + shared
}

// Accumulates values in range 1..951 (i*100+j+1, for i=0..10, j=0..50)
#[salsa::tracked]
fn hundred_impl<'db>(db: &'db dyn salsa::Database, ctx: HundredDeps<'db>, input: Input) -> u32 {
    let val = ctx.i(db) * 100 + ctx.j(db) + input.x(db);
    AccumulatedValue(val).accumulate(db);
    val
}

// Accumulates values in range 10_001..10_051 (distinct from HundredDeps range)
#[salsa::tracked]
fn shared_impl<'db>(db: &'db dyn salsa::Database, ctx: SharedDeps<'db>, input: Input) -> u32 {
    let val = 10_000 + ctx.j(db) + input.x(db);
    AccumulatedValue(val).accumulate(db);
    val
}

pub fn bench() -> u32 {
    (0..1000)
        .map(|_| {
            let db = AccumulateDbImpl::default();
            let input = Input::new(&db, 1);
            root_impl(&db, input);
            root_impl::accumulated::<AccumulatedValue>(&db, input).len() as u32
        })
        .sum()
}

pub fn bench_repeat() -> u32 {
    let db = AccumulateDbImpl::default();
    let input = Input::new(&db, 1);
    root_impl(&db, input);

    (0..1000)
        .map(|_| root_impl::accumulated::<AccumulatedValue>(&db, input).len() as u32)
        .sum()
}
