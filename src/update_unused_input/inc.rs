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
    // A difference between inc-complete and salsa:
    // - Salsa inputs are just structs with fields where in inc-complete
    //   they are still functions. So in inc-complete we set the return
    //   value of `Input(0)` as `1` while in salsa we create an `Input`
    //   struct with a field 1 and need to pass this down explicitly to
    //   computations which require it.
    Input(0).set(&mut db, 1);
    let result = Root.get(&db);

    for i in 0..1000 {
        // Input(1) is unused, only Input(0) is used
        Input(1).set(&mut db, i);
        let result = Root.get(&db);
        assert_ne!(result, 0);
    }

    result
}
