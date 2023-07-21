#[macro_use]
extern crate rocket;

use sqlx::{query, Execute, PgPool, Postgres, QueryBuilder};

mod fpl_api;
use fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use fpl_api::pull_data::pull_overview;
use fpl_api::types::{Overview, Player, PlayerFromDB};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

struct AppState {
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
async fn overview(state: &State<AppState>) -> Json<Overview> {
    let resp = pull_overview().await.unwrap();
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO players(player_id, first_name, second_name, now_cost, points_per_game, selected_by_percent, element_type, photo, team, total_points, minutes, starts)",
    );
    query_builder.push_values(&resp.elements, |mut b, player| {
        b.push_bind(player.player_id)
            .push_bind(&player.first_name)
            .push_bind(&player.second_name)
            .push_bind(player.now_cost)
            .push_bind(player.points_per_game.parse::<f32>().unwrap())
            .push_bind(player.selected_by_percent.parse::<f32>().unwrap())
            .push_bind(player.element_type)
            .push_bind(&player.photo)
            .push_bind(player.team)
            .push_bind(player.total_points)
            .push_bind(player.minutes)
            .push_bind(player.starts);
    });
    let query = query_builder.build();
    query.execute(&state.pool).await.unwrap();
    Json(resp)
}

#[get("/players")]
async fn players(state: &State<AppState>) -> Json<Vec<PlayerFromDB>> {
    let players = sqlx::query_as::<_, PlayerFromDB>("SELECT * from players");
    let result = players.fetch_all(&state.pool).await.unwrap();
    Json(result)
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let state = AppState { pool };
    let rocket = rocket::build()
        .manage(state)
        .mount("/", routes![index, fpl_endpoints, overview, players]);
    Ok(rocket.into())
}
