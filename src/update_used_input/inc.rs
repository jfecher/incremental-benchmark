use inc_complete::{
    DbHandle, define_input, define_intermediate, impl_storage,
    storage::{HashMapStorage, SingletonStorage},
};

#[derive(Default)]
struct Storage {
    root: SingletonStorage<Root>,
    ten_deps: HashMapStorage<TenDeps>,
    hundred_deps: HashMapStorage<HundredDeps>,
    inputs: HashMapStorage<Input>,
}

impl_storage!(Storage,
    root: Root,
    ten_deps: TenDeps,
    hundred_deps: HundredDeps,
    inputs: Input,
);

#[derive(Copy, Clone, PartialEq, Eq)]
struct Root;
define_intermediate!(0, Root -> u32, Storage, root_impl);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct TenDeps(u32);
define_intermediate!(1, TenDeps -> u32, Storage, ten_impl);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct HundredDeps(u32, u32);
define_intermediate!(2, HundredDeps -> u32, Storage, hundred_impl);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Input(u32);
define_input!(3, Input -> u32, Storage);

fn root_impl(_: &Root, db: &DbHandle<Storage>) -> u32 {
    (0..10).map(|i| TenDeps(i).get(db)).sum()
}

fn ten_impl(ctx: &TenDeps, db: &DbHandle<Storage>) -> u32 {
    (0..100).map(|i| HundredDeps(ctx.0, i).get(db)).sum()
}

fn hundred_impl(_: &HundredDeps, db: &DbHandle<Storage>) -> u32 {
    Input(0).get(db)
}

pub fn bench() -> u32 {
    let mut db = inc_complete::Db::<Storage>::new();

    (0..1000).map(|i| {
        // Input(1) is unused, only Input(0) is used
        Input(0).set(&mut db, i);
        Root.get(&db)
    }).sum()
}
