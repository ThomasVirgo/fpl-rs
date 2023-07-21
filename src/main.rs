#[macro_use]
extern crate rocket;

use sqlx::{PgPool, Postgres, QueryBuilder};

mod fpl_api;
use fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use fpl_api::pull_data::pull_overview;
use fpl_api::types::{Overview, PlayerFromDB};
use rocket::serde::json::Json;
use rocket::State;

struct AppState {
    pool: PgPool,
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to FPL wrapped..."
}

#[get("/fpl_endpoints")]
fn fpl_endpoints() -> String {
    get_fpl_url(FPLEndpoint::GameweekInfo { event_id: 5 })
}

#[get("/overview")]
async fn overview(state: &State<AppState>) -> String {
    let latest_created_at: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT created_at FROM players ORDER BY created_at DESC LIMIT 1")
            .fetch_one(&state.pool)
            .await
            .unwrap();
    let one_day_ago = chrono::Utc::now() - chrono::Duration::days(1);
    let resp = pull_overview().await.unwrap();
    if latest_created_at < one_day_ago {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO players(player_id, first_name, second_name, now_cost, points_per_game, selected_by_percent, element_type, photo, team, total_points, minutes, starts);",
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
        return String::from("added players data");
    }
    return String::from("did not add players data");
}

#[get("/players")]
async fn players(state: &State<AppState>) -> Json<Vec<PlayerFromDB>> {
    let players = sqlx::query_as::<_, PlayerFromDB>("SELECT * from players;");
    let result = players.fetch_all(&state.pool).await.unwrap();
    Json(result)
}

#[get("/players/<player_id>")]
async fn player_timeseries(state: &State<AppState>, player_id: i32) -> Json<Vec<PlayerFromDB>> {
    println!("{}", player_id);
    let players = sqlx::query_as::<_, PlayerFromDB>("SELECT * from players WHERE player_id = $1")
        .bind(&player_id);
    let result = players.fetch_all(&state.pool).await.unwrap();
    Json(result)
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let state = AppState { pool };
    let rocket = rocket::build().manage(state).mount(
        "/",
        routes![index, fpl_endpoints, overview, players, player_timeseries],
    );
    Ok(rocket.into())
}
