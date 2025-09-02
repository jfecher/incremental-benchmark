use inc_complete::{
    define_input, define_intermediate, intermediate, storage::{HashMapStorage, SingletonStorage}, DbHandle, Input, Storage
};

#[derive(Default, Storage)]
struct MyStorage {
    root: SingletonStorage<Root>,
    ten_deps: HashMapStorage<TenDeps>,
    hundred_deps: HashMapStorage<HundredDeps>,
    inputs: HashMapStorage<Input>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Root;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct TenDeps(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct HundredDeps(u32, u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Input)]
#[inc_complete(id = 3, output = u32, storage = MyStorage)]
struct Input(u32);

#[intermediate(id = 0)]
fn root_impl(_: &Root, db: &DbHandle<MyStorage>) -> u32 {
    (0..10).map(|i| TenDeps(i).get(db)).sum()
}

#[intermediate(id = 1)]
fn ten_impl(ctx: &TenDeps, db: &DbHandle<MyStorage>) -> u32 {
    (0..100).map(|i| HundredDeps(ctx.0, i).get(db)).sum()
}

#[intermediate(id = 2)]
fn hundred_impl(_: &HundredDeps, db: &DbHandle<MyStorage>) -> u32 {
    Input(0).get(db)
}

pub fn bench() -> u32 {
    let mut db = inc_complete::Db::<MyStorage>::new();
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
