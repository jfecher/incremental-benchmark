use salsa::Setter;

#[salsa::db]
#[derive(Default, Clone)]
pub struct UpdateInputDbImpl {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for UpdateInputDbImpl {}

#[salsa::interned]
struct Root {
    input: Input,
}

#[salsa::interned]
struct TenDeps {
    i: u32,
    input: Input,
}

#[salsa::interned]
struct HundredDeps {
    x: u32,
    i: u32,
    input: Input,
}

#[salsa::input(debug)]
struct Input {
    x: u32,
}

#[salsa::tracked]
fn root_impl<'db>(db: &'db dyn salsa::Database, root: Root<'db>) -> u32 {
    (0..10)
        .map(|i| ten_impl(db, TenDeps::new(db, i, root.input(db))))
        .sum()
}

#[salsa::tracked]
fn ten_impl<'db>(db: &'db dyn salsa::Database, ctx: TenDeps<'db>) -> u32 {
    (0..100)
        .map(|i| {
            let ctx_i = ctx.i(db);
            hundred_impl(db, HundredDeps::new(db, ctx_i, i, ctx.input(db)))
        })
        .sum()
}

#[salsa::tracked]
fn hundred_impl<'db>(db: &'db dyn salsa::Database, ctx: HundredDeps<'db>) -> u32 {
    ctx.input(db).x(db)
}

pub fn bench() -> u32 {
    let mut db = UpdateInputDbImpl::default();
    let input = Input::new(&db, 1);
    let result = root_impl(&db, Root::new(&db, input));

    for i in 0..1000 {
        // Update an unused input
        Input::new(&db, 2).set_x(&mut db).to(i);
        let result = root_impl(&db, Root::new(&db, input));
        assert_ne!(result, 0);
    }

    result
}
