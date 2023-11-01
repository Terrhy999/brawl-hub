use axum::{debug_handler, extract::{Path, State}, routing::get, Json, Router};
use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};
use std::net::SocketAddr;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/brawlhub";

// DONE  /commmanders/ => top commanders of all colors
// DONE /commanders/:colors => top commanders of {colors}
// /commanders/:colors/:time => top commanders of {colors} in the past {time}
// All of the above for cards instead of commanders
// Search for card/commander by name

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>
}

#[tokio::main]
async fn main() {

    let state = AppState {
        pool: PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Couldn't connect to db"),
    };

    let app = Router::new()
        .route("/commander_slugs", get(commander_slugs))
        .route("/commander/:slug", get(commander_by_slug))
        .route("/commanders/", get(top_commanders))
        .route("/commanders/:colors", get(top_commanders_of_color))
        // .route("/commanders/:colors/:time", get(top_commanders_of_color_time))
        .route("/commanders/colorless", get(top_commanders_colorless))
        .route("/top_cards", get(top_cards))
        .route("/top_cards/:colors", get(top_cards_of_color))
        .route(
            "/top_cards_for_commander/:oracle_id",
            get(top_cards_for_commander),
        )
        .route(
            "/top_cards_for_color_identity/:oracle_id",
            get(top_cards_for_color_identity_of_commander),
        )
        .with_state(state);
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
    slug: Option<String>,
}

// #[derive(serde::Serialize)]
// struct Commander {
//     oracle_id: String,
//     name: String,
//     lang: String,
//     scryfall_uri: String,
//     layout: String,
//     mana_cost: Option<String>,
//     cmc: f32,
//     type_line: String,
//     oracle_text: Option<String>,
//     colors: Option<Vec<String>>,
//     color_identity: Vec<String>,
//     is_legal: bool,
//     is_commander: bool,
//     rarity: String,
//     image_small: String,
//     image_normal: String,
//     image_large: String,
//     image_art_crop: String,
//     image_border_crop: String,
// }

async fn commander_by_slug(State(AppState{pool}): State<AppState>, Path(slug): Path<String>) -> Json<CardCount> {
    let res = sqlx::query_as!(CardCount, 
        "SELECT c.*, COUNT(d.*)
        FROM card c
        LEFT JOIN deck d
        ON c.oracle_id = d.commander
        WHERE slug = $1
        GROUP BY c.oracle_id;", slug).fetch_one(&pool).await.expect("couldn't fetch commander by slug");
    Json(res)
}

async fn commander_slugs(State(AppState{pool}): State<AppState>) -> Json<Vec<Option<String>>> {

    struct Response {
        slug: Option<String>
    }

    let res: Vec<Option<String>> = sqlx::query_as!(Response, "SELECT slug FROM card WHERE is_commander=true")
    .fetch_all(&pool)
    .await
    .expect("couldn't fetch slugs")
    .into_iter().map(|slug| slug.slug
    ).collect();

    Json(res)
}

async fn top_commanders_colorless(State(AppState{pool}): State<AppState>) -> Json<Vec<CardCount>> {


    let res = sqlx::query_as!(
        CardCount,
        "SELECT c.*, COUNT(d.commander) as count
    FROM card c
    LEFT JOIN deck d ON c.oracle_id = d.commander
    WHERE c.is_commander = TRUE 
    -- AND c.is_legal=TRUE
    AND (c.color_identity = '{}'::char(1)[])
    GROUP BY c.oracle_id
    ORDER BY count DESC
    "
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
}

async fn top_cards_of_color(Path(color): Path<String>, State(AppState{pool}): State<AppState>) -> Json<Vec<CommanderTopCard>> {
    let mut not_colors = vec![
        "W".to_string(),
        "U".to_string(),
        "B".to_string(),
        "R".to_string(),
        "G".to_string(),
    ];
    let mut colors = vec![];

    for char in color.chars() {
        not_colors.retain(|x| &char.to_ascii_uppercase().to_string() != x);
        colors.push(char.to_uppercase().to_string());
    }

    let res = sqlx::query_as!(CommanderTopCard, 
        "WITH DecksWithCommanderColor AS (
            SELECT d.id
            FROM deck d
            WHERE EXISTS (
                SELECT 1
                FROM card c
                WHERE d.commander = c.oracle_id
                AND (c.color_identity @> $1::char(1)[])
                AND NOT (c.color_identity && $2::char(1)[])
            )
        )
        SELECT c.*,
               (SELECT COUNT(*) FROM DecksWithCommanderColor) AS num_decks_total,
               (SELECT COUNT(*) FROM DecksWithCommanderColor dc
                WHERE dc.id IN (SELECT dl.deck_id FROM decklist dl WHERE dl.oracle_id = c.oracle_id)) AS num_decks_with_card
        FROM card c
        WHERE c.is_commander = TRUE
        AND (c.color_identity @> $1::char(1)[])
        AND NOT (c.color_identity && $2::char(1)[])
        ORDER BY num_decks_with_card DESC;        
        ", &colors, &not_colors).fetch_all(&pool).await.expect("error with db");
        Json(res)
}

async fn top_commanders_of_color(Path(color): Path<String>, State(AppState{pool}): State<AppState>) -> Json<Vec<CardCount>> {
    let mut not_colors = vec![
        "W".to_string(),
        "U".to_string(),
        "B".to_string(),
        "R".to_string(),
        "G".to_string(),
    ];
    let mut colors = vec![];

    for char in color.chars() {
        not_colors.retain(|x| &char.to_ascii_uppercase().to_string() != x);
        colors.push(char.to_uppercase().to_string());
    }
    println!("colors: {:#?} \n not_colors = {:#?}", colors, not_colors);

    let res = sqlx::query_as!(
        CardCount,
        "SELECT c.*, COUNT(d.commander) AS count
        FROM card c
        LEFT JOIN deck d ON c.oracle_id = d.commander
        WHERE c.is_commander = TRUE
        -- AND c.is_legal=TRUE
        AND c.color_identity @> $1::char(1)[]  -- Checks if it contains all colors in 'colors'
        AND NOT c.color_identity && $2::char(1)[]  -- Checks if it intersects with 'not_colors'
        GROUP BY c.oracle_id
        ORDER BY count DESC;        
        ",
        &colors,
        &not_colors
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
}

async fn top_commanders_of_color_time(Path((color, time)): Path<(String, String)>, State(AppState{pool}): State<AppState>) -> Json<Vec<CardCount>> {

    let mut not_colors = vec![
        "W".to_string(),
        "U".to_string(),
        "B".to_string(),
        "R".to_string(),
        "G".to_string(),
    ];
    let mut colors = vec![];

    for char in color.chars() {
        not_colors.retain(|x| &char.to_ascii_uppercase().to_string() != x);
        colors.push(char.to_uppercase().to_string());
    }
    println!("colors: {:#?} \n not_colors = {:#?}", colors, not_colors);

    let res = sqlx::query_as!(
        CardCount,
        "SELECT c.*, COUNT(d.commander) AS count
        FROM card c
        LEFT JOIN deck d ON c.oracle_id = d.commander
        WHERE c.is_commander = TRUE
        -- AND c.is_legal=TRUE
        AND c.color_identity @> $1::char(1)[]  -- Checks if it contains all colors in 'colors'
        AND NOT c.color_identity && $2::char(1)[]  -- Checks if it intersects with 'not_colors'
        GROUP BY c.oracle_id
        ORDER BY count DESC;        
        ",
        &colors,
        &not_colors
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
}

#[debug_handler]
async fn top_commanders(State(AppState{pool}): State<AppState>) -> Json<Vec<CardCount>> {

    let res = sqlx::query_as!(
        CardCount,
        "SELECT c.*, COUNT(d.commander) AS count
        FROM card c
        LEFT JOIN deck d ON c.oracle_id = d.commander
        WHERE c.is_commander = TRUE
        GROUP BY c.oracle_id
        ORDER BY count DESC;        
        "
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
}

#[debug_handler]
async fn top_cards(State(AppState{pool}): State<AppState>) -> Json<Vec<CardCount>> {

    let res = sqlx::query_as!(
        CardCount,
        "SELECT c.*, COUNT(dl.oracle_id) AS count
        FROM card c
        LEFT JOIN decklist dl ON c.oracle_id = dl.oracle_id
        GROUP BY c.oracle_id
        ORDER BY count DESC;"
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
    slug: Option<String>
}

#[derive(Debug, serde::Serialize)]
struct TopCardsForCommander {
    creatures: Vec<CommanderTopCard>,
    instants: Vec<CommanderTopCard>,
    sorceries: Vec<CommanderTopCard>,
    utility_artifacts: Vec<CommanderTopCard>,
    enchantments: Vec<CommanderTopCard>,
    planeswalkers: Vec<CommanderTopCard>,
    mana_artifacts: Vec<CommanderTopCard>,
    lands: Vec<CommanderTopCard>,
    other: Vec<CommanderTopCard>,
}

#[debug_handler]
async fn top_cards_for_commander(Path(oracle_id): Path<String>, State(AppState{pool}): State<AppState>) -> Json<TopCardsForCommander> {

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

    fn is_mana_artifact(card: &CommanderTopCard) -> bool {
        card.type_line.to_ascii_lowercase().contains("artifact")
            && card
                .oracle_text
                .clone()
                .expect("missing oracle text")
                .to_ascii_lowercase()
                .contains("add")
            && card
                .oracle_text
                .clone()
                .expect("missing oracle text")
                .to_ascii_lowercase()
                .contains("mana")
    }

    let mut top_cards_sorted = TopCardsForCommander {
        creatures: vec![],
        instants: vec![],
        sorceries: vec![],
        utility_artifacts: vec![],
        enchantments: vec![],
        planeswalkers: vec![],
        mana_artifacts: vec![],
        lands: vec![],
        other: vec![],
    };

    for card in res {
        match &card {
            t if t.type_line.to_ascii_lowercase().contains("creature") => {
                top_cards_sorted.creatures.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("instant") => {
                top_cards_sorted.instants.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("sorcery") => {
                top_cards_sorted.sorceries.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("planeswalker") => {
                top_cards_sorted.planeswalkers.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("enchantment") => {
                top_cards_sorted.enchantments.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("land") => {
                top_cards_sorted.lands.push(card)
            }
            t if is_mana_artifact(&t) => top_cards_sorted.mana_artifacts.push(card),
            t if t.type_line.to_ascii_lowercase().contains("artifact") => {
                top_cards_sorted.utility_artifacts.push(card)
            }
            _ => top_cards_sorted.other.push(card),
        };
    }

    Json(top_cards_sorted)
}

async fn top_cards_for_color_identity_of_commander(
    Path(oracle_id): Path<String>,
    State(AppState{pool}): State<AppState>
) -> Json<TopCardsForCommander> {

    let res = sqlx::query_as!(
        CommanderTopCard,
        "WITH ColorIdentity AS (SELECT color_identity FROM card WHERE oracle_id = $1),
        DecksWithCommanderColor AS (
            SELECT d.id
            FROM deck d
            WHERE EXISTS (
                SELECT 1
                FROM card c
                WHERE d.commander = c.oracle_id
                AND (
                    (c.color_identity = (SELECT * FROM ColorIdentity))
                    OR
                    ((SELECT * FROM ColorIdentity) = '{}'::char(1)[] AND c.color_identity = ARRAY[]::char(1)[])
                )
            )
        )
        SELECT DISTINCT c.*,
               (SELECT COUNT(*) FROM DecksWithCommanderColor) AS num_decks_total,
               (SELECT COUNT(*) FROM DecksWithCommanderColor dc
                WHERE dc.id IN (SELECT dl.deck_id FROM decklist dl WHERE dl.oracle_id = c.oracle_id)) AS num_decks_with_card
        FROM card c
        JOIN decklist dl ON c.oracle_id = dl.oracle_id
        WHERE dl.deck_id IN (SELECT id FROM DecksWithCommanderColor)
        ORDER BY num_decks_with_card DESC;
        ",
        Uuid::parse_str(&oracle_id).expect("uuid parsed wrong")
    )
    .fetch_all(&pool)
    .await
    .expect("couldn't query db");

    fn is_mana_artifact(card: &CommanderTopCard) -> bool {
        card.type_line.to_ascii_lowercase().contains("artifact")
            && card
                .oracle_text
                .clone()
                .expect("missing oracle text")
                .to_ascii_lowercase()
                .contains("add")
            && card
                .oracle_text
                .clone()
                .expect("missing oracle text")
                .to_ascii_lowercase()
                .contains("mana")
    }

    let mut top_cards_sorted = TopCardsForCommander {
        creatures: vec![],
        instants: vec![],
        sorceries: vec![],
        utility_artifacts: vec![],
        enchantments: vec![],
        planeswalkers: vec![],
        mana_artifacts: vec![],
        lands: vec![],
        other: vec![],
    };

    for card in res {
        match &card {
            t if t.type_line.to_ascii_lowercase().contains("creature") => {
                top_cards_sorted.creatures.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("instant") => {
                top_cards_sorted.instants.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("sorcery") => {
                top_cards_sorted.sorceries.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("planeswalker") => {
                top_cards_sorted.planeswalkers.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("enchantment") => {
                top_cards_sorted.enchantments.push(card)
            }
            t if t.type_line.to_ascii_lowercase().contains("land") => {
                top_cards_sorted.lands.push(card)
            }
            t if is_mana_artifact(&t) => top_cards_sorted.mana_artifacts.push(card),
            t if t.type_line.to_ascii_lowercase().contains("artifact") => {
                top_cards_sorted.utility_artifacts.push(card)
            }
            _ => top_cards_sorted.other.push(card),
        };
    }

    Json(top_cards_sorted)
}
