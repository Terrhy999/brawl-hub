use axum::{debug_handler, routing::get, Json, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
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
struct Commander {
    commander: String,
    count: Option<i64>,
}

#[debug_handler]
async fn top_commanders() -> Json<Vec<Commander>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db");

    let res = sqlx::query_as!(
        Commander,
        // "SELECT c.name as commander FROM deck d JOIN card c ON d.commander = c.oracle_id"
        "SELECT c.name as commander, COUNT(commander) AS count FROM deck d JOIN card c ON d.commander = c.oracle_id GROUP BY c.name ORDER BY count DESC LIMIT 100"
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
}
