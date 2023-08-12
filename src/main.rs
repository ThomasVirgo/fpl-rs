#[macro_use]
extern crate rocket;

use sqlx::{PgPool, Postgres, QueryBuilder};

mod fpl_api;
use fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use fpl_api::pull_data::{pull_league_standings, pull_overview};
use fpl_api::types::{LeagueStandings, ManagerDB, PlayerFromDB};
use rocket::serde::json::Json;
use rocket::State;

const OVERALL_LEAGUE_ID: u32 = 314;

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

#[get("/managers")]
async fn managers(state: &State<AppState>) -> Json<Vec<ManagerDB>> {
    let managers = sqlx::query_as::<_, ManagerDB>("SELECT * from managers;");
    let result = managers.fetch_all(&state.pool).await.unwrap();
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

#[get("/managers/<player_name>")]
async fn get_managers(state: &State<AppState>, player_name: String) -> Json<Vec<ManagerDB>> {
    println!("{}", player_name);
    let players = sqlx::query_as::<_, ManagerDB>(
        "SELECT *
    FROM managers
    WHERE player_name ILIKE $1;",
    )
    .bind(format!("%{}%", &player_name));
    let result = players.fetch_all(&state.pool).await.unwrap();
    Json(result)
}

#[get("/add_managers")]
async fn add_managers(state: &State<AppState>) -> Json<LeagueStandings> {
    let latest_page: Result<i32, sqlx::Error> = sqlx::query_scalar(
        "SELECT page
        FROM page_logs
        ORDER BY created_at DESC
        LIMIT 1;",
    )
    .fetch_one(&state.pool)
    .await;
    let page = match latest_page {
        Ok(latest) => latest + 1,
        Err(_) => 1,
    };
    let resp = pull_league_standings(OVERALL_LEAGUE_ID, page)
        .await
        .unwrap();
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO managers(manager_id, player_name, entry_name)");
    query_builder.push_values(&resp.standings.results, |mut b, manager| {
        b.push_bind(manager.entry)
            .push_bind(&manager.player_name)
            .push_bind(&manager.entry_name);
    });
    let query = query_builder.build();
    let query_result = query.execute(&state.pool).await;
    match query_result {
        Ok(_) => {}
        Err(_) => {}
    }

    sqlx::query("INSERT INTO page_logs (page) VALUES ($1)")
        .bind(page)
        .execute(&state.pool)
        .await
        .unwrap();
    Json(resp)
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let state = AppState { pool };
    let rocket = rocket::build().manage(state).mount(
        "/",
        routes![
            index,
            fpl_endpoints,
            overview,
            players,
            player_timeseries,
            add_managers,
            get_managers,
            managers,
        ],
    );
    Ok(rocket.into())
}
