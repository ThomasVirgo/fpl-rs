#[macro_use]
extern crate rocket;

use fpl_api::fpl_schemas::manager_team::ManagerTeam;
use fpl_api::fpl_schemas::manager_transfers::ManagerTransfers;
use fpl_api::fpl_schemas::player_points_scored::PlayerStatsForGameweek;
use rocket::futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::time::Instant;

mod fpl_api;
use fpl_api::data_loader::get_data_for_endpoint;
use fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use fpl_api::fpl_schemas::manager_history::ManagerHistory;
use fpl_api::fpl_schemas::manager_summary::ManagerSummary;
use fpl_api::logic::GameweekInfo;
use fpl_api::logic::{element_to_name_mapping, ids_difference};
use fpl_api::pull_data::{pull_league_standings, pull_manager, pull_overview};
use fpl_api::types::{LeagueStandings, ManagerDB, PlayerFromDB};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

const OVERALL_LEAGUE_ID: i32 = 314;

struct AppState {
    pool: PgPool,
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to FPL wrapped..."
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

#[get("/count_managers")]
async fn count_managers(state: &State<AppState>) -> Json<i64> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM managers")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    Json(count)
}

#[get("/players/<player_id>")]
async fn player_timeseries(state: &State<AppState>, player_id: i32) -> Json<Vec<PlayerFromDB>> {
    println!("{}", player_id);
    let players = sqlx::query_as::<_, PlayerFromDB>("SELECT * from players WHERE player_id = $1")
        .bind(&player_id);
    let result = players.fetch_all(&state.pool).await.unwrap();
    Json(result)
}

enum JsonResultorError {}

//example https://fpl.shuttleapp.rs/managers?get_by_name&name=Bob%20Smith
#[get("/managers?get_by_name&<name>")]
async fn get_manager_by_name(state: &State<AppState>, name: String) -> Json<Vec<ManagerDB>> {
    let managers = sqlx::query_as::<_, ManagerDB>(
        "SELECT *
    FROM managers
    WHERE LOWER(player_name) = $1;",
    )
    .bind(name.to_lowercase());
    let result = managers.fetch_all(&state.pool).await.unwrap();
    Json(result)
}

#[derive(Deserialize, Serialize, Debug)]
struct ManagerInfo {
    manager_history: ManagerHistory,
    manager_summary: ManagerSummary,
    manager_transfers: ManagerTransfers,
    manager_teams: Vec<ManagerTeam>,
    player_points_scores: Vec<PlayerStatsForGameweek>,
    time_taken: u128,
}

#[get("/managers/<manager_id>/captain")]
async fn manager_info_v2(state: &State<AppState>, manager_id: i32) {
    let overview = pull_overview().await.unwrap();
    let mut gameweek_infos: Vec<GameweekInfo> = Vec::new();
    for event_id in 1..=2 {
        let manager_team = get_data_for_endpoint::<ManagerTeam>(FPLEndpoint::ManagerTeam {
            manager_id,
            event_id,
        })
        .await
        .unwrap();

        let stats_for_gw =
            get_data_for_endpoint::<PlayerStatsForGameweek>(FPLEndpoint::GameweekInfo { event_id })
                .await
                .unwrap();

        gameweek_infos.push(GameweekInfo {
            gameweek: event_id,
            player_stats: stats_for_gw,
            manager_team: manager_team,
        })
    }
}

#[get("/managers/<manager_id>")]
async fn manager_info(state: &State<AppState>, manager_id: i32) -> Json<ManagerInfo> {
    let now = Instant::now();
    let manager_summary =
        get_data_for_endpoint::<ManagerSummary>(FPLEndpoint::ManagerSummary { manager_id })
            .await
            .unwrap();
    let manager_history =
        get_data_for_endpoint::<ManagerHistory>(FPLEndpoint::ManagerHistory { manager_id })
            .await
            .unwrap();
    let manager_transfers =
        get_data_for_endpoint::<ManagerTransfers>(FPLEndpoint::ManagerTransfers { manager_id })
            .await
            .unwrap();
    let mut manager_teams = Vec::new();
    let mut player_points_scores = Vec::new();
    for event_id in 1..=2 {
        let manager_team = get_data_for_endpoint::<ManagerTeam>(FPLEndpoint::ManagerTeam {
            manager_id,
            event_id,
        })
        .await
        .unwrap();
        manager_teams.push(manager_team);

        let stats_for_gw =
            get_data_for_endpoint::<PlayerStatsForGameweek>(FPLEndpoint::GameweekInfo { event_id })
                .await
                .unwrap();
        player_points_scores.push(stats_for_gw);
    }
    let time_taken = now.elapsed();
    Json(ManagerInfo {
        manager_history,
        manager_summary,
        manager_transfers,
        manager_teams,
        player_points_scores,
        time_taken: time_taken.as_millis(),
    })
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let state = AppState { pool };
    let rocket = rocket::build()
        .manage(state)
        .mount(
            "/",
            routes![
                index,
                overview,
                player_timeseries,
                get_manager_by_name,
                count_managers,
                manager_info,
            ],
        )
        .attach(CORS);
    Ok(rocket.into())
}

// OLD CODE FOR REFERENCE

// #[get("/players")]
// async fn players(state: &State<AppState>) -> Json<Vec<PlayerFromDB>> {
//     let players = sqlx::query_as::<_, PlayerFromDB>("SELECT * from players;");
//     let result = players.fetch_all(&state.pool).await.unwrap();
//     Json(result)
// }

// #[get("/managers")]
// async fn managers(state: &State<AppState>) -> Json<Vec<ManagerDB>> {
//     let managers = sqlx::query_as::<_, ManagerDB>("SELECT * from managers;");
//     let result = managers.fetch_all(&state.pool).await.unwrap();
//     Json(result)
// }

// #[get("/add_managers")]
// async fn add_managers(state: &State<AppState>) -> Json<Vec<i32>> {
//     let start_idx_result: Result<i32, sqlx::Error> = sqlx::query_scalar(
//         "SELECT start_idx
//         FROM add_manager_logs
//         ORDER BY created_at DESC
//         LIMIT 1;",
//     )
//     .fetch_one(&state.pool)
//     .await;
//     let start_idx = match start_idx_result {
//         Ok(latest) => latest + 100,
//         Err(_) => 1,
//     };
//     let all_manager_ids: Vec<i32> = (start_idx..start_idx + 100).collect();
//     let managers =
//         sqlx::query_as::<_, ManagerDB>("SELECT * FROM managers WHERE manager_id = ANY($1)")
//             .bind(&all_manager_ids)
//             .fetch_all(&state.pool)
//             .await
//             .unwrap();
//     let manager_ids: Vec<i32> = managers.iter().map(|x| x.manager_id).collect();
//     let ids_to_add = ids_difference(all_manager_ids, manager_ids);
//     for manager_id in ids_to_add.clone() {
//         let manager_summary = pull_manager(manager_id).await.unwrap();
//         sqlx::query(
//             "INSERT INTO managers (manager_id, player_name, entry_name) VALUES ($1, $2, $3)",
//         )
//         .bind(manager_summary.id)
//         .bind(format!(
//             "{} {}",
//             manager_summary.player_first_name, manager_summary.player_last_name
//         ))
//         .bind(&manager_summary.name)
//         .execute(&state.pool)
//         .await
//         .unwrap();
//     }

//     sqlx::query("INSERT INTO add_manager_logs (start_idx) VALUES ($1)")
//         .bind(start_idx)
//         .execute(&state.pool)
//         .await
//         .unwrap();
//     Json(ids_to_add)
// }
