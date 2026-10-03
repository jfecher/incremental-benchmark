use inc_complete::{
    DbHandle,
    accumulate::{Accumulated, Accumulator},
    define_input, define_intermediate, impl_storage,
    storage::{HashMapStorage, SingletonStorage},
};

#[derive(Default)]
struct Storage {
    root: SingletonStorage<Root>,
    ten_deps: HashMapStorage<TenDeps>,
    hundred_deps: HashMapStorage<HundredDeps>,
    shared_deps: HashMapStorage<SharedDeps>,
    input: SingletonStorage<Input>,
    acc_storage: HashMapStorage<Accumulated<u32>>,
    acc: Accumulator<u32>,
}

impl_storage!(Storage,
    root: Root,
    ten_deps: TenDeps,
    hundred_deps: HundredDeps,
    shared_deps: SharedDeps,
    input: Input,
    acc_storage: Accumulated<u32>,
    @accumulators {
        acc: u32,
    }
);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Root;
define_intermediate!(0, Root -> u32, Storage, root_impl);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TenDeps(u32);
define_intermediate!(1, TenDeps -> u32, Storage, ten_impl);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct HundredDeps(u32, u32);
define_intermediate!(2, HundredDeps -> u32, Storage, hundred_impl);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SharedDeps(u32);
define_intermediate!(3, SharedDeps -> u32, Storage, shared_impl);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Input;
define_input!(4, Input -> u32, Storage);

fn root_impl(_: &Root, db: &DbHandle<Storage>) -> u32 {
    (0..10).map(|i| TenDeps(i).get(db)).sum()
}

fn ten_impl(ctx: &TenDeps, db: &DbHandle<Storage>) -> u32 {
    let unique: u32 = (0..50).map(|j| HundredDeps(ctx.0, j).get(db)).sum();
    let shared: u32 = (0..50).map(|j| SharedDeps(j).get(db)).sum();
    unique + shared
}

// Accumulates values in range 1..951 (i*100+j+1, for i=0..10, j=0..50)
fn hundred_impl(ctx: &HundredDeps, db: &DbHandle<Storage>) -> u32 {
    let val = ctx.0 * 100 + ctx.1 + Input.get(db);
    db.accumulate(val);
    val
}

// Accumulates values in range 10_001..10_051 (distinct from HundredDeps range)
fn shared_impl(ctx: &SharedDeps, db: &DbHandle<Storage>) -> u32 {
    let val = 10_000 + ctx.0 + Input.get(db);
    db.accumulate(val);
    val
}

pub fn bench_accumulated() -> u32 {
    (0..1000)
        .map(|_| {
            let mut db = inc_complete::Db::<Storage>::new();
            Input.set(&mut db, 1);
            Root.get(&db);
            db.get_accumulated::<u32, _>(Root).len() as u32
        })
        .sum()
}

pub fn bench_accumulated_uncached() -> u32 {
    (0..1000)
        .map(|_| {
            let mut db = inc_complete::Db::<Storage>::new();
            Input.set(&mut db, 1);
            Root.get(&db);

            db.get_accumulated_uncached::<u32, _>(Root).len() as u32
        })
        .sum()
}

// Reuse the same db to see if each accumulate call is cached
pub fn bench_accumulated_repeat() -> u32 {
    let mut db = inc_complete::Db::<Storage>::new();
    Input.set(&mut db, 1);
    Root.get(&db);

    (0..1000)
        .map(|_| db.get_accumulated::<u32, _>(Root).len() as u32)
        .sum()
}

pub fn bench_accumulated_repeat_uncached() -> u32 {
    let mut db = inc_complete::Db::<Storage>::new();
    Input.set(&mut db, 1);
    Root.get(&db);

    (0..1000)
        .map(|_| db.get_accumulated_uncached::<u32, _>(Root).len() as u32)
        .sum()
}
