use axum::{debug_handler, routing::get, Json, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/brawlhub";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(top_commanders));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(serde::Serialize)]
struct CommanderInfo {
    oracle_id: String,
    name: String,
    lang: String,
    scryfall_uri: String,
    layout: String,
    mana_cost: Option<String>,
    cmc: f32,
    type_line: String,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_commander: bool,
    rarity: String,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    count: Option<i64>,
}

#[debug_handler]
async fn top_commanders() -> Json<Vec<CommanderInfo>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db");

    let res = sqlx::query_as!(
        CommanderInfo,
        "SELECT c.*, COUNT(d.commander) AS count
        FROM card c
        INNER JOIN deck d ON c.oracle_id = d.commander
        GROUP BY c.oracle_id
        ORDER BY count DESC
        LIMIT 100;        
        "
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
}
