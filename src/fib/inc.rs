use inc_complete::{DbHandle, Storage, define_intermediate, intermediate, storage::HashMapStorage};

#[derive(Default, Storage)]
struct MyStorage {
    fibs: HashMapStorage<Fib>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Fib(u32);

#[intermediate(id = 0)]
fn fib_impl(fib: &Fib, db: &DbHandle<MyStorage>) -> u32 {
    if fib.0 < 2 {
        fib.0
    } else {
        Fib(fib.0 - 2).get(db).wrapping_add(Fib(fib.0 - 1).get(db))
    }
}

pub fn bench() -> u32 {
    let db = inc_complete::Db::<MyStorage>::new();
    Fib(crate::FIB_INPUT).get(&db)
}
