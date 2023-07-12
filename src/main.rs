#[macro_use]
extern crate rocket;

use sqlx::PgPool;

mod fpl_api;
use fpl_api::endpoints::{get_fpl_url, FPLEndpoint};

struct MyState {
    pool: PgPool,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/fpl_endpoints")]
fn fpl_endpoints() -> String {
    get_fpl_url(FPLEndpoint::GameweekInfo { event_id: 5 })
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let state = MyState { pool };
    let rocket = rocket::build()
        .manage(state)
        .mount("/", routes![index, fpl_endpoints]);
    Ok(rocket.into())
}
