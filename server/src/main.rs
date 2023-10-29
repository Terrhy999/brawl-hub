use axum::{debug_handler, extract::Path, routing::get, Json, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/brawlhub";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/get_top_commanders", get(top_commanders))
        .route("/get_top_cards", get(top_cards))
        .route("/commanders/:colors", get(get_commanders_of_color))
        .route("/all_commanders", get(get_historic_brawl_commanders));
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

#[derive(serde::Serialize)]
struct Commander {
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
}

async fn get_historic_brawl_commanders()-> Json<Vec<Commander>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db");

    let res = sqlx::query_as!(
        Commander,
        "SELECT * FROM card WHERE is_commander = true AND is_legal",
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
}

async fn get_commanders_of_color(Path(id): Path<String>) -> Json<Vec<CommanderInfo>> {
    // Have to figure out how handle routes that aren't wubrg or colorless
    let mut colors = vec![];
    if id == "colorless" {
    } else {
        for char in id.chars() {
            colors.push(char.to_uppercase().to_string());
        }
    }

    // println!("{:#?}", colors);
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
        WHERE is_commander = true 
        AND color_identity <@ $1::char(1)[] 
        AND color_identity @> $1::char(1)[]
        GROUP BY c.oracle_id
        ORDER BY count DESC;",
        &colors
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
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

#[derive(Debug, serde::Serialize)]
struct TopCard {
    oracle_id: String, // Assuming Uuid is represented as a string
    name: String,
    lang: String,
    scryfall_uri: String,
    layout: String,
    mana_cost: Option<String>,
    cmc: f64,
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
    number_of_decks: Option<i64>,
}

#[debug_handler]
async fn top_cards() -> Json<Vec<TopCard>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db");

    let res = sqlx::query_as!(
        TopCard,
        "SELECT c.*, COUNT(dl.oracle_id) AS number_of_decks
    FROM card c
    LEFT JOIN decklist dl ON c.oracle_id = dl.oracle_id
    GROUP BY c.oracle_id
    ORDER BY number_of_decks DESC
    LIMIT 100;"
    )
    .fetch_all(&pool)
    .await
    .expect("error querying db");

    Json(res)
}
