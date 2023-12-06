#![allow(unused)]
use axum::{
    debug_handler,
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;

// DONE  /commmanders/ => top commanders of all colors
// DONE /commanders/:colors => top commanders of {colors}
// /commanders/:colors/:time => top commanders of {colors} in the past {time}
// All of the above for cards instead of commanders
// Search for card/commander by name
// A route that takes a card, and returns the number of decks that card appears in and the number of decks that card COULD appear in

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");
    println!("{}", database_url);
    let state = AppState {
        pool: PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Couldn't connect to db"),
    };

    // DONE /card/:slug route should return alchemy-version of card if one exists
    // DONE /card_slugs should only return non-alchemy slugs (card.name is non-alchemy now)
    // DONE /card/:slug route should return count of decks it appears in, and count of decks it could appear in
    // /commander_top_cards/:oracle_id should not return the commander, or basic lands

    let app = Router::new()
        .route("/commander_slugs", get(commander_slugs))
        .route("/card_slugs", get(card_slugs))
        .route("/card/:slug", get(card_by_slug))
        .route("/commander/:slug", get(commander_by_slug))
        .route("/commanders/", get(top_commanders))
        .route("/commanders/:colors", get(top_commanders_of_color))
        // .route("/commanders/:colors/:time", get(top_commanders_of_color_time))
        .route("/commanders/colorless", get(top_commanders_colorless))
        .route("/top_cards", get(top_cards))
        .route("/top_cards/:colors", get(top_cards_of_color))
        .route("/commander_top_cards/:oracle_id", get(commander_top_cards))
        .route(
            "/top_commanders_for_card/:slug",
            get(top_commanders_for_card),
        )
        // .route(
        //     "/top_cards_for_commander/:oracle_id",
        //     get(top_cards_for_commander),
        // )
        // .route(
        //     "/top_cards_for_color_identity/:oracle_id",
        //     get(top_cards_for_color_identity_of_commander),
        // )
        .route("/search/:card_", get(get_card))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));

    println!("Server listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
#[derive(serde::Serialize)]
struct Card {
    oracle_id: String,
    name_full: String,
    name_front: String,
    name_back: Option<String>,
    slug: String,
    scryfall_uri: String,
    layout: String,
    rarity: String,
    lang: String,
    mana_cost_combined: Option<String>,
    mana_cost_front: Option<String>,
    mana_cost_back: Option<String>,
    cmc: f32,
    type_line_full: String,
    type_line_front: String,
    type_line_back: Option<String>,
    oracle_text: Option<String>,
    oracle_text_back: Option<String>,
    colors: Option<Vec<String>>,
    colors_back: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_legal_commander: bool,
    is_rebalanced: bool,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    image_small_back: Option<String>,
    image_normal_back: Option<String>,
    image_large_back: Option<String>,
    image_art_crop_back: Option<String>,
    image_border_crop_back: Option<String>,
}

#[derive(serde::Serialize)]
struct CardCount {
    oracle_id: String,
    name_full: String,
    name_front: String,
    name_back: Option<String>,
    slug: String,
    scryfall_uri: String,
    layout: String,
    rarity: String,
    lang: String,
    mana_cost_combined: Option<String>,
    mana_cost_front: Option<String>,
    mana_cost_back: Option<String>,
    cmc: f32,
    type_line_full: String,
    type_line_front: String,
    type_line_back: Option<String>,
    oracle_text: Option<String>,
    oracle_text_back: Option<String>,
    colors: Option<Vec<String>>,
    colors_back: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_legal_commander: bool,
    is_rebalanced: bool,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    image_small_back: Option<String>,
    image_normal_back: Option<String>,
    image_large_back: Option<String>,
    image_art_crop_back: Option<String>,
    image_border_crop_back: Option<String>,
    count: Option<i64>,
}

async fn card_slugs(State(AppState { pool }): State<AppState>) -> Json<Vec<Option<String>>> {
    struct Response {
        slug: Option<String>,
    }

    let res = sqlx::query_as!(
        Response,
        "SELECT DISTINCT slug FROM card WHERE is_legal=true"
    )
    .fetch_all(&pool)
    .await
    .expect("couldn't fetch card slugs")
    .into_iter()
    .map(|res| res.slug)
    .collect();
    Json(res)
}

#[derive(serde::Serialize)]
struct CardBySlug {
    oracle_id: String,
    name_full: String,
    name_front: String,
    name_back: Option<String>,
    slug: String,
    scryfall_uri: String,
    layout: String,
    rarity: String,
    lang: String,
    mana_cost_combined: Option<String>,
    mana_cost_front: Option<String>,
    mana_cost_back: Option<String>,
    cmc: f32,
    type_line_full: String,
    type_line_front: String,
    type_line_back: Option<String>,
    oracle_text: Option<String>,
    oracle_text_back: Option<String>,
    colors: Option<Vec<String>>,
    colors_back: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_legal_commander: bool,
    is_rebalanced: bool,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    image_small_back: Option<String>,
    image_normal_back: Option<String>,
    image_large_back: Option<String>,
    image_art_crop_back: Option<String>,
    image_border_crop_back: Option<String>,
    total_decks_with_card: Option<i64>,
    total_decks_could_play: Option<i64>,
}

#[axum::debug_handler]
async fn card_by_slug(
    State(AppState { pool }): State<AppState>,
    Path(slug): Path<String>,
) -> Json<CardBySlug> {
    let res = sqlx::query_as!(
        CardBySlug,
        "SELECT
        card.*,
        (
            SELECT COUNT(DISTINCT decklist.deck_id)
            FROM decklist
            WHERE decklist.oracle_id = card.oracle_id
        ) AS total_decks_with_card,
        (
            SELECT COUNT(DISTINCT deck.id)
            FROM deck
            WHERE deck.color_identity @> card.color_identity
        ) AS total_decks_could_play
    FROM
        card
    WHERE
        card.slug = $1;",
        slug
    )
    .fetch_one(&pool)
    .await
    .expect("couldn't fetch card by slug");
    Json(res)
}
#[derive(serde::Serialize)]
struct CardSlug {
    oracle_id: String,
    name_full: String,
    name_front: String,
    name_back: Option<String>,
    slug: String,
    scryfall_uri: String,
    layout: String,
    rarity: String,
    lang: String,
    mana_cost_combined: Option<String>,
    mana_cost_front: Option<String>,
    mana_cost_back: Option<String>,
    cmc: f32,
    type_line_full: String,
    type_line_front: String,
    type_line_back: Option<String>,
    oracle_text: Option<String>,
    oracle_text_back: Option<String>,
    colors: Option<Vec<String>>,
    colors_back: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_legal_commander: bool,
    is_rebalanced: bool,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    image_small_back: Option<String>,
    image_normal_back: Option<String>,
    image_large_back: Option<String>,
    image_art_crop_back: Option<String>,
    image_border_crop_back: Option<String>,
    total_decks: Option<i64>,
    all_decks: Option<i64>,
    rank: Option<i64>,
    total_commander_decks_of_ci: Option<i64>,
}

async fn commander_by_slug(
    State(AppState { pool }): State<AppState>,
    Path(slug): Path<String>,
) -> Json<CardSlug> {
    let res = sqlx::query_as!(
        CardSlug,
        "SELECT card.*, total_decks, all_decks, rank, total_commander_decks_of_ci FROM card
        JOIN (
            SELECT COUNT(commander) as all_decks FROM deck
        ) as d1 ON true
        JOIN (
            SELECT COUNT(commander) as total_decks, commander FROM deck
            GROUP BY commander
        ) AS d ON card.oracle_id = d.commander
        JOIN (
            SELECT commander, row_number() OVER (ORDER BY COUNT(commander) DESC) rank FROM deck
            GROUP BY commander
        ) AS commander_rank ON commander_rank.commander = card.oracle_id
        JOIN (
            SELECT COUNT(*) AS total_commander_decks_of_ci FROM deck
            WHERE color_identity = (SELECT color_identity FROM card WHERE slug = $1 LIMIT 1)
        ) AS total_commander_decks_of_ci ON true
        WHERE card.slug = $1;",
        slug
    )
    .fetch_optional(&pool)
    .await
    .expect("couldn't fetch commander by slug");

    let res = match res {
        Some(card) => card,
        None => {
            let card = sqlx::query_as!(Card, "SELECT * FROM card WHERE slug = $1", slug)
                .fetch_one(&pool)
                .await
                .expect("Couldn't fetch commander by slug");
            CardSlug {
                all_decks: Some(0),
                cmc: card.cmc,
                color_identity: card.color_identity,
                colors: card.colors,
                image_art_crop: card.image_art_crop,
                image_border_crop: card.image_border_crop,
                image_large: card.image_large,
                image_normal: card.image_normal,
                image_small: card.image_small,
                is_legal: card.is_legal,
                is_legal_commander: card.is_legal_commander,
                lang: card.lang,
                layout: card.layout,
                oracle_id: card.oracle_id,
                oracle_text: card.oracle_text,
                rank: Some(0),
                total_commander_decks_of_ci: Some(0),
                rarity: card.rarity,
                scryfall_uri: card.scryfall_uri,
                slug: card.slug,
                total_decks: Some(0),
                name_full: card.name_full,
                name_front: card.name_front,
                name_back: card.name_back,
                mana_cost_combined: card.mana_cost_combined,
                mana_cost_front: card.mana_cost_front,
                mana_cost_back: card.mana_cost_back,
                type_line_full: card.type_line_full,
                type_line_front: card.type_line_front,
                type_line_back: card.type_line_back,
                oracle_text_back: card.oracle_text_back,
                colors_back: card.colors_back,
                is_rebalanced: card.is_rebalanced,
                image_small_back: card.image_small_back,
                image_normal_back: card.image_normal_back,
                image_large_back: card.image_large_back,
                image_art_crop_back: card.image_art_crop_back,
                image_border_crop_back: card.image_border_crop_back,
            }
        }
    };
    Json(res)
}

async fn commander_slugs(State(AppState { pool }): State<AppState>) -> Json<Vec<Option<String>>> {
    struct Response {
        slug: Option<String>,
    }

    let res: Vec<Option<String>> = sqlx::query_as!(
        Response,
        "SELECT DISTINCT slug FROM card WHERE is_legal_commander=true"
    )
    .fetch_all(&pool)
    .await
    .expect("couldn't fetch slugs")
    .into_iter()
    .map(|slug| slug.slug)
    .collect();

    Json(res)
}

async fn top_commanders_colorless(
    State(AppState { pool }): State<AppState>,
) -> Json<Vec<CardCount>> {
    let res = sqlx::query_as!(
        CardCount,
        "SELECT c.*, COUNT(d.commander) as count
    FROM card c
    LEFT JOIN deck d ON c.oracle_id = d.commander
    WHERE c.is_legal_commander = TRUE
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

#[derive(Debug, serde::Serialize)]
struct CommanderTopCard {
    oracle_id: String,
    name_full: String,
    name_front: String,
    name_back: Option<String>,
    slug: String,
    scryfall_uri: String,
    layout: String,
    rarity: String,
    lang: String,
    mana_cost_combined: Option<String>,
    mana_cost_front: Option<String>,
    mana_cost_back: Option<String>,
    cmc: f32,
    type_line_full: String,
    type_line_front: String,
    type_line_back: Option<String>,
    oracle_text: Option<String>,
    oracle_text_back: Option<String>,
    colors: Option<Vec<String>>,
    colors_back: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_legal_commander: bool,
    is_rebalanced: bool,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    image_small_back: Option<String>,
    image_normal_back: Option<String>,
    image_large_back: Option<String>,
    image_art_crop_back: Option<String>,
    image_border_crop_back: Option<String>,
    num_decks_with_card: Option<i64>,
    num_decks_total: Option<i64>,
}

async fn top_cards_of_color(
    Path(color): Path<String>,
    State(AppState { pool }): State<AppState>,
) -> Json<Vec<CommanderTopCard>> {
    // Right now 'num_decks_total' is the number of decks with this EXACT color_identity, it needs to be the number of decks that INCLUDE this color identity
    // Eg colors = 'U' 'num_decks_total' is the number of mono-blue decks, not the number of decks with 'U' in the color_identity

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

    let res = sqlx::query_as!(CommanderTopCard,"WITH DecksWithCommanderColor AS (
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
        WHERE c.is_legal_commander = TRUE
        AND (c.color_identity @> $1::char(1)[])
        AND NOT (c.color_identity && $2::char(1)[])
        ORDER BY num_decks_with_card DESC;
        ", &colors, &not_colors).fetch_all(&pool).await.expect("error with db");
    Json(res)
}

async fn top_commanders_of_color(
    Path(color): Path<String>,
    State(AppState { pool }): State<AppState>,
) -> Json<Vec<CardCount>> {
    //Used to display the top commanders of a specific color identity, ordered by the number of decks with this commander
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
        WHERE c.is_legal_commander = TRUE
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

async fn top_commanders_of_color_time(
    Path((color, time)): Path<(String, String)>,
    State(AppState { pool }): State<AppState>,
) -> Json<Vec<CardCount>> {
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
        WHERE c.is_legal_commander = TRUE
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
async fn top_commanders(State(AppState { pool }): State<AppState>) -> Json<Vec<CardCount>> {
    // Used to display every commander, ordered number of decks with this commander
    let res = sqlx::query_as!(
        CardCount,
        "SELECT c.*, COUNT(d.commander) AS count
        FROM card c
        LEFT JOIN deck d ON c.oracle_id = d.commander
        WHERE c.is_legal_commander = TRUE
        GROUP BY c.oracle_id
        ORDER BY count DESC
        LIMIT 100;"
    )
    .fetch_all(&pool)
    .await
    .expect("error retrieving from db");

    Json(res)
}

#[debug_handler]
async fn top_cards(State(AppState { pool }): State<AppState>) -> Json<Vec<CardBySlug>> {
    // Used to display the top cards, ordered by the number of decks the card appears in
    // FIX: Should be ordered by (number of decks the card appears in / number of decks the card CAN appear in)
    let res = sqlx::query_as!(
        CardBySlug,
        "WITH CardCounts AS (
            SELECT
                card.*,
                COUNT(DISTINCT deck.id) AS total_decks_could_play,
                COUNT(DISTINCT decklist.deck_id) AS total_decks_with_card
            FROM card
            JOIN deck 
              ON deck.color_identity @> card.color_identity
            LEFT JOIN decklist 
              ON card.oracle_id = decklist.oracle_id
            GROUP BY
                card.oracle_id
          )
          SELECT *
          FROM CardCounts
          ORDER BY (total_decks_with_card * 100 / NULLIF(total_decks_could_play, 0)) DESC,
          total_decks_with_card DESC
          LIMIT 100;
          "
    )
    .fetch_all(&pool)
    .await
    .expect("error querying db");

    Json(res)
}

// #[derive(Debug, serde::Serialize)]
// struct TopCardsForCommander {
//     creatures: Vec<CommanderTopCard>,
//     instants: Vec<CommanderTopCard>,
//     sorceries: Vec<CommanderTopCard>,
//     utility_artifacts: Vec<CommanderTopCard>,
//     enchantments: Vec<CommanderTopCard>,
//     planeswalkers: Vec<CommanderTopCard>,
//     mana_artifacts: Vec<CommanderTopCard>,
//     lands: Vec<CommanderTopCard>,
//     other: Vec<CommanderTopCard>,
// }

// #[derive(Debug, serde::Serialize)]
// struct TopCard {
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
//     slug: Option<String>,
//     total_decks_of_commander: Option<i64>,
//     decks_of_commander_with_card: Option<i64>,
//     total_decks_of_color: Option<i64>,
//     decks_of_color_with_card: Option<i64>,
//     usage_in_commander: Option<f64>,
//     usage_in_color: Option<f64>,
//     synergy: Option<f64>,
// }

#[derive(Debug, serde::Serialize)]
struct TopCard2 {
    oracle_id: String,
    name_full: String,
    name_front: String,
    name_back: Option<String>,
    slug: String,
    scryfall_uri: String,
    layout: String,
    rarity: String,
    lang: String,
    mana_cost_combined: Option<String>,
    mana_cost_front: Option<String>,
    mana_cost_back: Option<String>,
    cmc: f32,
    type_line_full: String,
    type_line_front: String,
    type_line_back: Option<String>,
    oracle_text: Option<String>,
    oracle_text_back: Option<String>,
    colors: Option<Vec<String>>,
    colors_back: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_legal_commander: bool,
    is_rebalanced: bool,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    image_small_back: Option<String>,
    image_normal_back: Option<String>,
    image_large_back: Option<String>,
    image_art_crop_back: Option<String>,
    image_border_crop_back: Option<String>,
    quantity: Option<i64>,
    total_commander_decks: Option<i64>,
    ci_quantity: Option<i64>,
    total_commander_decks_of_ci: Option<i64>,
}

#[derive(Debug, serde::Serialize, Clone)]
struct TopCardWithSynergy {
    oracle_id: String,
    name_full: String,
    name_front: String,
    name_back: Option<String>,
    slug: String,
    scryfall_uri: String,
    layout: String,
    rarity: String,
    lang: String,
    mana_cost_combined: Option<String>,
    mana_cost_front: Option<String>,
    mana_cost_back: Option<String>,
    cmc: f32,
    type_line_full: String,
    type_line_front: String,
    type_line_back: Option<String>,
    oracle_text: Option<String>,
    oracle_text_back: Option<String>,
    colors: Option<Vec<String>>,
    colors_back: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_legal_commander: bool,
    is_rebalanced: bool,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    image_small_back: Option<String>,
    image_normal_back: Option<String>,
    image_large_back: Option<String>,
    image_art_crop_back: Option<String>,
    image_border_crop_back: Option<String>,
    quantity: Option<i64>,
    total_commander_decks: Option<i64>,
    ci_quantity: Option<i64>,
    total_commander_decks_of_ci: Option<i64>,
    synergy: f64,
    usage_in_commander: f64,
    usage_in_color: f64,
}

impl TopCardWithSynergy {
    fn add_synergy(other: &TopCard2) -> Self {
        let usage_in_commander =
            (other.quantity.unwrap() as f64 / other.total_commander_decks.unwrap() as f64) * 100.00;
        let usage_in_color = (other.ci_quantity.unwrap() as f64
            / other.total_commander_decks_of_ci.unwrap() as f64)
            * 100.00;
        TopCardWithSynergy {
            oracle_id: other.oracle_id.clone(),
            lang: other.lang.clone(),
            scryfall_uri: other.scryfall_uri.clone(),
            layout: other.layout.clone(),
            cmc: other.cmc,
            oracle_text: other.oracle_text.clone(),
            colors: other.colors.clone(),
            color_identity: other.color_identity.clone(),
            is_legal: other.is_legal,
            is_legal_commander: other.is_legal_commander,
            rarity: other.rarity.clone(),
            image_small: other.image_small.clone(),
            image_normal: other.image_normal.clone(),
            image_large: other.image_large.clone(),
            image_art_crop: other.image_art_crop.clone(),
            image_border_crop: other.image_border_crop.clone(),
            slug: other.slug.clone(),
            quantity: other.quantity,
            total_commander_decks: other.total_commander_decks,
            ci_quantity: other.ci_quantity,
            total_commander_decks_of_ci: other.total_commander_decks_of_ci,
            usage_in_commander,
            usage_in_color,
            synergy: usage_in_commander - usage_in_color,
            name_full: other.name_full.clone(),
            name_front: other.name_front.clone(),
            name_back: other.name_back.clone(),
            mana_cost_combined: other.mana_cost_combined.clone(),
            mana_cost_front: other.mana_cost_front.clone(),
            mana_cost_back: other.mana_cost_back.clone(),
            type_line_full: other.type_line_full.clone(),
            type_line_front: other.type_line_front.clone(),
            type_line_back: other.type_line_back.clone(),
            oracle_text_back: other.oracle_text_back.clone(),
            colors_back: other.colors_back.clone(),
            is_rebalanced: other.is_rebalanced.clone(),
            image_small_back: other.image_small_back.clone(),
            image_normal_back: other.image_normal_back.clone(),
            image_large_back: other.image_large_back.clone(),
            image_art_crop_back: other.image_art_crop_back.clone(),
            image_border_crop_back: other.image_border_crop_back.clone(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct TopCardsForCommander2 {
    creatures: Vec<TopCardWithSynergy>,
    instants: Vec<TopCardWithSynergy>,
    sorceries: Vec<TopCardWithSynergy>,
    utility_artifacts: Vec<TopCardWithSynergy>,
    enchantments: Vec<TopCardWithSynergy>,
    planeswalkers: Vec<TopCardWithSynergy>,
    mana_artifacts: Vec<TopCardWithSynergy>,
    lands: Vec<TopCardWithSynergy>,
}
// TODO Clean this up
async fn commander_top_cards(
    Path(oracle_id): Path<String>,
    State(AppState { pool }): State<AppState>,
) -> Json<TopCardsForCommander2> {
    let top_cards_for_commander = sqlx::query_as!(
        TopCard2,
        "SELECT card.*, quantity, total_commander_decks, ci_quantity, total_commander_decks_of_ci FROM card
        JOIN (
            SELECT oracle_id, COUNT(oracle_id) as quantity FROM decklist
            LEFT JOIN deck ON decklist.deck_id = deck.id
            WHERE commander = $1 AND oracle_id <> $1
            GROUP BY oracle_id
        ) AS card_quantity ON card_quantity.oracle_id = card.oracle_id
        JOIN (
            SELECT COUNT(*) AS total_commander_decks
                FROM deck
                WHERE commander = $1
        ) AS total_commander_decks ON true
        JOIN (
            SELECT oracle_id, COUNT(oracle_id) as ci_quantity FROM decklist
            LEFT JOIN deck ON decklist.deck_id = deck.id
            WHERE color_identity = (SELECT color_identity FROM card WHERE oracle_id = $1)
            GROUP BY oracle_id
        ) AS ci_card_quantity ON ci_card_quantity.oracle_id = card.oracle_id
        JOIN (
            SELECT COUNT(*) AS total_commander_decks_of_ci
            FROM deck
            WHERE color_identity = (SELECT color_identity FROM card WHERE oracle_id = $1)
        ) AS total_commander_decks_of_ci ON true
        WHERE card.type_line_full NOT LIKE 'Basic Land%'
        ORDER BY quantity DESC
        LIMIT 1000;",
        Uuid::parse_str(&oracle_id).expect("uuid parsed wrong"),
    )
    .fetch_all(&pool)
    .await
    .expect("error querying db");

    let top_cards_for_commander = top_cards_for_commander
        .iter()
        .map(TopCardWithSynergy::add_synergy);

    let mut top_cards = TopCardsForCommander2 {
        creatures: vec![],
        instants: vec![],
        sorceries: vec![],
        utility_artifacts: vec![],
        enchantments: vec![],
        planeswalkers: vec![],
        mana_artifacts: vec![],
        lands: vec![],
    };

    fn is_mana_artifact(card: &TopCardWithSynergy) -> bool {
        card.type_line_full
            .to_ascii_lowercase()
            .contains("artifact")
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

    for card in top_cards_for_commander.into_iter() {
        match &card {
            t if t.type_line_front.to_ascii_lowercase().contains("creature")
                && top_cards.creatures.len() < 50 =>
            {
                top_cards.creatures.push(card)
            }
            t if t.type_line_front.to_ascii_lowercase().contains("instant")
                && top_cards.instants.len() < 50 =>
            {
                top_cards.instants.push(card)
            }
            t if t.type_line_front.to_ascii_lowercase().contains("sorcery")
                && top_cards.sorceries.len() < 50 =>
            {
                top_cards.sorceries.push(card)
            }
            t if t
                .type_line_front
                .to_ascii_lowercase()
                .contains("planeswalker")
                && top_cards.planeswalkers.len() < 50 =>
            {
                top_cards.planeswalkers.push(card)
            }
            t if t
                .type_line_front
                .to_ascii_lowercase()
                .contains("enchantment")
                && top_cards.enchantments.len() < 50 =>
            {
                top_cards.enchantments.push(card)
            }
            t if t.type_line_front.to_ascii_lowercase().contains("land")
                && top_cards.lands.len() < 50 =>
            {
                top_cards.lands.push(card)
            }
            t if is_mana_artifact(&t) => top_cards.mana_artifacts.push(card),
            t if t.type_line_front.to_ascii_lowercase().contains("artifact")
                && top_cards.utility_artifacts.len() < 50 =>
            {
                top_cards.utility_artifacts.push(card)
            }
            _ => (),
        };
    }

    Json(top_cards)
}

// #[debug_handler]
// async fn top_cards_for_commander(
//     Path(oracle_id): Path<String>,
//     State(AppState { pool }): State<AppState>,
// ) -> Json<TopCardsForCommander> {
//     let res = sqlx::query_as!(
//         CommanderTopCard,
//         "SELECT c.*, num_decks_with_card, num_decks_total
//         FROM card c
//         JOIN (
//             SELECT c1.oracle_id, COUNT(dl1.deck_id) AS num_decks_with_card
//             FROM card c1
//             LEFT JOIN decklist dl1 ON c1.oracle_id = dl1.oracle_id
//             WHERE dl1.deck_id IN (
//                 SELECT id
//                 FROM deck
//                 WHERE commander = $1
//             )
//             GROUP BY c1.oracle_id
//         )
//         AS CardPlayCounts ON c.oracle_id = CardPlayCounts.oracle_id
//         JOIN (
//             SELECT COUNT(*) AS num_decks_total
//             FROM deck
//             WHERE commander = $1
//         )
//         AS TotalCommanderDecks ON true
//         WHERE num_decks_total > 0
//         ORDER BY num_decks_with_card DESC;",
//         Uuid::parse_str(&oracle_id).expect("uuid parsed wrong")
//     )
//     .fetch_all(&pool)
//     .await
//     .expect("error querying db");

//     fn is_mana_artifact(card: &CommanderTopCard) -> bool {
//         card.type_line.to_ascii_lowercase().contains("artifact")
//             && card
//                 .oracle_text
//                 .clone()
//                 .expect("missing oracle text")
//                 .to_ascii_lowercase()
//                 .contains("add")
//             && card
//                 .oracle_text
//                 .clone()
//                 .expect("missing oracle text")
//                 .to_ascii_lowercase()
//                 .contains("mana")
//     }

//     let mut top_cards_sorted = TopCardsForCommander {
//         creatures: vec![],
//         instants: vec![],
//         sorceries: vec![],
//         utility_artifacts: vec![],
//         enchantments: vec![],
//         planeswalkers: vec![],
//         mana_artifacts: vec![],
//         lands: vec![],
//         other: vec![],
//     };

//     for card in res {
//         match &card {
//             t if t.type_line.to_ascii_lowercase().contains("creature") => {
//                 top_cards_sorted.creatures.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("instant") => {
//                 top_cards_sorted.instants.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("sorcery") => {
//                 top_cards_sorted.sorceries.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("planeswalker") => {
//                 top_cards_sorted.planeswalkers.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("enchantment") => {
//                 top_cards_sorted.enchantments.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("land") => {
//                 top_cards_sorted.lands.push(card)
//             }
//             t if is_mana_artifact(&t) => top_cards_sorted.mana_artifacts.push(card),
//             t if t.type_line.to_ascii_lowercase().contains("artifact") => {
//                 top_cards_sorted.utility_artifacts.push(card)
//             }
//             _ => top_cards_sorted.other.push(card),
//         };
//     }

//     Json(top_cards_sorted)
// }

// async fn top_cards_for_color_identity_of_commander(
//     Path(oracle_id): Path<String>,
//     State(AppState { pool }): State<AppState>,
// ) -> Json<TopCardsForCommander> {
//     let res = sqlx::query_as!(
//         CommanderTopCard,
//         "WITH ColorIdentity AS (SELECT color_identity FROM card WHERE oracle_id = $1),
//         DecksWithCommanderColor AS (
//             SELECT d.id
//             FROM deck d
//             WHERE EXISTS (
//                 SELECT 1
//                 FROM card c
//                 WHERE d.commander = c.oracle_id
//                 AND (
//                     (c.color_identity = (SELECT * FROM ColorIdentity))
//                     OR
//                     ((SELECT * FROM ColorIdentity) = '{}'::char(1)[] AND c.color_identity = ARRAY[]::char(1)[])
//                 )
//             )
//         )
//         SELECT DISTINCT c.*,
//                (SELECT COUNT(*) FROM DecksWithCommanderColor) AS num_decks_total,
//                (SELECT COUNT(*) FROM DecksWithCommanderColor dc
//                 WHERE dc.id IN (SELECT dl.deck_id FROM decklist dl WHERE dl.oracle_id = c.oracle_id)) AS num_decks_with_card
//         FROM card c
//         JOIN decklist dl ON c.oracle_id = dl.oracle_id
//         WHERE dl.deck_id IN (SELECT id FROM DecksWithCommanderColor)
//         ORDER BY num_decks_with_card DESC;
//         ",
//         Uuid::parse_str(&oracle_id).expect("uuid parsed wrong")
//     )
//     .fetch_all(&pool)
//     .await
//     .expect("couldn't query db");

//     fn is_mana_artifact(card: &CommanderTopCard) -> bool {
//         card.type_line.to_ascii_lowercase().contains("artifact")
//             && card
//                 .oracle_text
//                 .clone()
//                 .expect("missing oracle text")
//                 .to_ascii_lowercase()
//                 .contains("add")
//             && card
//                 .oracle_text
//                 .clone()
//                 .expect("missing oracle text")
//                 .to_ascii_lowercase()
//                 .contains("mana")
//     }

//     let mut top_cards_sorted = TopCardsForCommander {
//         creatures: vec![],
//         instants: vec![],
//         sorceries: vec![],
//         utility_artifacts: vec![],
//         enchantments: vec![],
//         planeswalkers: vec![],
//         mana_artifacts: vec![],
//         lands: vec![],
//         other: vec![],
//     };

//     for card in res {
//         match &card {
//             t if t.type_line.to_ascii_lowercase().contains("creature") => {
//                 top_cards_sorted.creatures.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("instant") => {
//                 top_cards_sorted.instants.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("sorcery") => {
//                 top_cards_sorted.sorceries.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("planeswalker") => {
//                 top_cards_sorted.planeswalkers.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("enchantment") => {
//                 top_cards_sorted.enchantments.push(card)
//             }
//             t if t.type_line.to_ascii_lowercase().contains("land") => {
//                 top_cards_sorted.lands.push(card)
//             }
//             t if is_mana_artifact(&t) => top_cards_sorted.mana_artifacts.push(card),
//             t if t.type_line.to_ascii_lowercase().contains("artifact") => {
//                 top_cards_sorted.utility_artifacts.push(card)
//             }
//             _ => top_cards_sorted.other.push(card),
//         };
//     }

//     Json(top_cards_sorted)
// }

// return the front face name the of flip cards
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchResults {
    card_name: String,
    image: String,
    slug: String,
}
async fn get_card(
    Path(card_name): Path<String>,
    State(AppState { pool }): State<AppState>,
) -> Json<Vec<SearchResults>> {
    struct Response {
        name_full: String,
        image_art_crop: String,
        slug: Option<String>,
        is_legal_commander: bool,
    }
    let res = sqlx::query_as!(
        Response,
        "SELECT name_full, image_art_crop, slug, is_legal_commander FROM card WHERE is_legal = true AND name_full ILIKE $1 LIMIT 20",
        format!("%{}%", card_name)
    )
    .fetch_all(&pool)
    .await
    .expect("couldn't fetch card");

    fn get_route(is_commander: bool, slug: String) -> String {
        if is_commander {
            format!("/commander/{}", slug)
        } else {
            format!("/card/{}", slug)
        }
    }

    fn get_search_results(res: Response) -> SearchResults {
        match res.slug {
            Some(slug) => SearchResults {
                card_name: res.name_full,
                image: res.image_art_crop,
                slug: get_route(res.is_legal_commander, slug),
            },
            None => panic!("Not found"),
        }
    }

    let search_results: Vec<SearchResults> = res.into_iter().map(get_search_results).collect();
    Json(search_results)
}

async fn top_commanders_for_card(
    Path(slug): Path<String>,
    State(AppState { pool }): State<AppState>,
) -> Json<Vec<CardCount>> {
    let res = sqlx::query_as!(
        CardCount,
        "SELECT card.*, commander_stats.count
        FROM card
        JOIN (
            SELECT deck.commander, COUNT(deck.commander) as count
            FROM card
            JOIN decklist ON card.oracle_id = decklist.oracle_id
            JOIN deck ON decklist.deck_id = deck.id
            WHERE card.oracle_id = (
                SELECT card.oracle_id FROM card WHERE card.slug = $1
            )
            GROUP BY deck.commander
            ORDER BY COUNT(deck.commander) DESC
        ) AS commander_stats ON card.oracle_id = commander_stats.commander
        ",
        slug
    )
    .fetch_all(&pool)
    .await
    .expect("couldn't fetch commanders");
    Json(res)
}
