#[macro_use]
extern crate rocket;

use sqlx::PgPool;

mod fpl_api;
use fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use fpl_api::pull_data::pull_overview;
use fpl_api::types::Overview;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

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

#[get("/overview")]
async fn overview() -> Json<Overview> {
    let resp = pull_overview().await.unwrap();
    Json(resp)
    // match resp {
    //     Result::Ok(overview) => Ok(Json(overview)),
    //     Result::Err(_) => Err(status::Custom(
    //         Status::InternalServerError,
    //         "Could not load overview data".into(),
    //     )),
    // }
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let state = MyState { pool };
    let rocket = rocket::build()
        .manage(state)
        .mount("/", routes![index, fpl_endpoints, overview]);
    Ok(rocket.into())
}
