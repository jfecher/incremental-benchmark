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
    let db = UpdateInputDbImpl::default();

    (0..1000).map(|i| {
        let input = Input::new(&db, i);
        root_impl(&db, Root::new(&db, input))
    }).sum()
}
