use inc_complete::{DbHandle, define_intermediate, impl_storage, storage::HashMapStorage};

#[derive(Default)]
struct Storage {
    fibs: HashMapStorage<Fib>,
}

impl_storage!(Storage, fibs: Fib);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Fib(u32);

define_intermediate!(0, Fib -> u32, Storage, fib_impl);

fn fib_impl(fib: &Fib, db: &DbHandle<Storage>) -> u32 {
    if fib.0 < 2 {
        fib.0
    } else {
        Fib(fib.0 - 2).get(db).wrapping_add(Fib(fib.0 - 1).get(db))
    }
}

pub fn bench_fib() -> u32 {
    let db = inc_complete::Db::<Storage>::new();
    Fib(crate::FIB_INPUT).get(&db)
}
