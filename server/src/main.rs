use axum::{debug_handler, extract::Path, routing::get, Json, Router};
use sqlx::{postgres::PgPoolOptions, types::Uuid};
use std::net::SocketAddr;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/brawlhub";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/get_top_commanders", get(top_commanders))
        .route("/get_top_cards", get(top_cards))
        .route("/commanders/:colors", get(top_commanders_of_color))
        .route(
            "/top_cards_for_commander/:oracle_id",
            get(top_cards_for_commander),
        )
        .route(
            "/top_cards_for_color_identity/:oracle_id",
            get(top_cards_for_color_identity),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(serde::Serialize)]
struct CardCount {
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

async fn top_commanders_of_color(Path(id): Path<String>) -> Json<Vec<CardCount>> {
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
        CardCount,
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
async fn top_commanders() -> Json<Vec<CardCount>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db");

    let res = sqlx::query_as!(
        CardCount,
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

#[debug_handler]
async fn top_cards() -> Json<Vec<CardCount>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db");

    let res = sqlx::query_as!(
        CardCount,
        "SELECT c.*, COUNT(dl.oracle_id) AS count
    FROM card c
    LEFT JOIN decklist dl ON c.oracle_id = dl.oracle_id
    GROUP BY c.oracle_id
    ORDER BY count DESC
    LIMIT 100;"
    )
    .fetch_all(&pool)
    .await
    .expect("error querying db");

    Json(res)
}

#[derive(Debug, serde::Serialize)]
struct CommanderTopCard {
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
    num_decks_with_card: Option<i64>,
    num_decks_total: Option<i64>,
}

#[debug_handler]
async fn top_cards_for_commander(Path(oracle_id): Path<String>) -> Json<Vec<CommanderTopCard>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db");

    let res = sqlx::query_as!(
        CommanderTopCard,
        "SELECT c.*, num_decks_with_card, num_decks_total
        FROM card c
        JOIN (
            SELECT c1.oracle_id, COUNT(dl1.deck_id) AS num_decks_with_card
            FROM card c1
            LEFT JOIN decklist dl1 ON c1.oracle_id = dl1.oracle_id
            WHERE dl1.deck_id IN (
                SELECT id
                FROM deck
                WHERE commander = $1
            )
            GROUP BY c1.oracle_id
        )
        AS CardPlayCounts ON c.oracle_id = CardPlayCounts.oracle_id
        JOIN (
            SELECT COUNT(*) AS num_decks_total
            FROM deck
            WHERE commander = $1
        ) 
        AS TotalCommanderDecks ON true
        WHERE num_decks_total > 0
        ORDER BY num_decks_with_card DESC;",
        Uuid::parse_str(&oracle_id).expect("uuid parsed wrong")
    )
    .fetch_all(&pool)
    .await
    .expect("error querying db");

    Json(res)
}

async fn top_cards_for_color_identity(
    Path(oracle_id): Path<String>,
) -> Json<Vec<CommanderTopCard>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db");

    let res = sqlx::query_as!(
        CommanderTopCard,
        "WITH CommanderColor AS (
            SELECT color_identity
            FROM card
            WHERE oracle_id = $1
        )
        SELECT c.*, num_decks_with_card, num_decks_total
        FROM card c
        JOIN (
            SELECT dl1.oracle_id, COUNT(dl1.deck_id) AS num_decks_with_card
            FROM decklist dl1
            JOIN deck d1 ON dl1.deck_id = d1.id
            WHERE d1.commander IN (
                SELECT commander
                FROM deck
                WHERE commander = $1
            )
            GROUP BY dl1.oracle_id
        ) AS CardPlayCounts ON c.oracle_id = CardPlayCounts.oracle_id
        JOIN (
            SELECT COUNT(DISTINCT d2.id) AS num_decks_total
            FROM deck d2
            WHERE d2.commander IN (
                SELECT commander
                FROM deck
                WHERE commander = $1
            )
        ) AS TotalCommanderDecks ON true
        WHERE c.color_identity = (SELECT * FROM CommanderColor)
        ORDER BY num_decks_with_card DESC;
        ",
        Uuid::parse_str(&oracle_id).expect("uuid parsed wrong")
    )
    .fetch_all(&pool)
    .await
    .expect("couldn't query db");

    Json(res)
}
