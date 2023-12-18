// #![allow(unused)]
// use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use sqlx::types::Uuid;
use chrono::prelude::*;
use futures::future::join_all;
use std::{collections::HashMap, fmt::Debug, fs};
use uuid::Uuid;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/brawlhub";

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("couldn't connect to db");

    // let decks = get_aetherhub_decks(0, 500).await;
    // for deck in decks {
    //     migrate_aetherhub_decklists(&pool, &deck).await
    // }
    migrate_scryfall_alchemy_cards(&pool).await;
}

async fn migrate_scryfall_alchemy_cards(pool: &Pool<Postgres>) {
    let data = fs::read_to_string("default-cards.json").expect("unable to read JSON");
    let scryfall_cards: Vec<ScryfallCard> =
        serde_json::from_str(&data).expect("unable to parse JSON");

    // rarity order: common < uncommon < rare < mythic
    fn is_lower_rarity(current: &str, new: &str) -> bool {
        match (current, new) {
            ("common", _) => false,
            ("uncommon", "common") => true,
            ("uncommon", _) => false,
            ("rare", "common") | ("rare", "uncommon") => true,
            ("rare", _) => false,
            ("mythic", "rare") | ("mythic", "uncommon") | ("mythic" , "common") => true,
            _ => false
        }
    }

    let unwanted_layouts = [
        "token",
        "flip",
        "planar",
        "scheme",
        "vanguard",
        "double_faced_token",
        "emblem",
        "augment",
        "host",
        "art_series",
        "reversible_card"
    ];

    let scryfall_cards: Vec<ScryfallCard> = scryfall_cards
        .into_iter()
        .filter(|card| {
            card.games().contains(&String::from("arena")) && !unwanted_layouts.contains(&card.layout().as_str())})
        .collect();

    let mut unique_scryfall_cards: HashMap<String, ScryfallCard> = HashMap::new();

    scryfall_cards.into_iter().for_each(|mut card| {
        card.set_lowest_rarity(card.rarity());
        if card.rarity() != card.lowest_rarity() {println!("rarities don't match");}
        let (oracle_id, released_at) = (card.oracle_id(), card.released_at());
        let new_released_at = released_at;

        unique_scryfall_cards
            .entry(oracle_id.clone())
            .and_modify(|existing_card| {
                if existing_card.released_at() < new_released_at {
                    *existing_card = card.clone();
                }

                // Update lowest_rarity if it's empty or the new card has a lower rarity
            if is_lower_rarity(&existing_card.lowest_rarity(), &card.rarity()) {
                existing_card.set_lowest_rarity(card.rarity());
            }            

            })
            .or_insert(card);
    });

    // Get a list of all the alchemy card names, with the 'A-' prefix stripped, and remove cards with that name from the HashMap
    // println!("Cards: {}", unique_scryfall_cards.len());

    let alchemy_card_names: Vec<String> = unique_scryfall_cards
        .iter()
        .filter_map(|(_, card)| {
            card.is_rebalanced()
                .then(|| strip_alchemy_prefix(&card.name()))
        })
        .collect();
    unique_scryfall_cards.retain(|_, card| !alchemy_card_names.contains(&card.name().to_string()));

    // unique_scryfall_cards
    //     .into_iter()
    //     .map(|(oracle_id, mut card)| {
    //         if card.is_rebalanced() {
    //             card.name = card.name.strip_prefix("A-")
    //         }
    //     });

    //need to remove the A- from these alchemy cards because i'm searching by non-alchemy names

    let cards: Vec<Card> = unique_scryfall_cards
        .into_iter()
        .map(|(oracle_id, scryfall_card)| Card::from(scryfall_card))
        .collect();

    for card in cards {
        // println!(
        //     "Insert {}, {} into brawlhub.card",
        //     card.name_full, card.oracle_id
        // );
        sqlx::query_as!(
            Card,
            "INSERT INTO card(
            oracle_id,
            name_full,
            name_front,
            name_back,
            slug,
            scryfall_uri,
            layout,
            rarity,
            lang,
            mana_cost_combined,
            mana_cost_front,
            mana_cost_back,
            cmc,
            type_line_full,
            type_line_front,
            type_line_back,
            oracle_text,
            oracle_text_back,
            colors,
            colors_back,
            color_identity,
            is_legal,
            is_legal_commander,
            is_rebalanced,
            image_small,
            image_normal,
            image_large,
            image_art_crop,
            image_border_crop,
            image_small_back,
            image_normal_back,
            image_large_back,
            image_art_crop_back,
            image_border_crop_back,
            lowest_rarity
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35)
        ",
        Uuid::parse_str(&card.oracle_id).expect("Parse uuid from oracle_id string"),
        card.name_full,
        card.name_front,
        card.name_back,
        card.slug,
        card.scryfall_uri,
        card.layout,
        card.rarity,
        card.lang,
        card.mana_cost_combined,
        card.mana_cost_front,
        card.mana_cost_back,
        card.cmc,
        card.type_line_full,
        card.type_line_front,
        card.type_line_back,
        card.oracle_text,
        card.oracle_text_back,
        card.colors.as_deref(),
        card.colors_back.as_deref(),
        &card.color_identity,
        card.is_legal,
        card.is_legal_commander,
        card.is_rebalanced,
        card.image_small,
        card.image_normal,
        card.image_large,
        card.image_art_crop,
        card.image_border_crop,
        card.image_small_back,
        card.image_normal_back,
        card.image_large_back,
        card.image_art_crop_back,
        card.image_border_crop_back,
        card.lowest_rarity,
        )
        .execute(pool)
        .await
        .expect("couldn't insert");
    }
}

async fn migrate_aetherhub_decklists(pool: &Pool<Postgres>, deck: &AetherHubDeck) {
    #[derive(Serialize, Deserialize, Debug)]
    struct AetherhubCard {
        quantity: Option<i32>,
        name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CardInDeck {
        quantity: Option<i32>,
        name: String,
        a_name: String,
        is_commander: bool,
        is_companion: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        converted_deck: Vec<AetherhubCard>,
    }

    let aetherhub_decklist: Vec<AetherhubCard> = serde_json::from_str::<Response>(
        reqwest::Client::new()
            .get(format!(
                "https://aetherhub.com/Deck/FetchMtgaDeckJson?deckId={}",
                deck.id // 975951
            ))
            .send()
            .await
            .expect("couldn't fetch aetherhub_decklist")
            .text()
            .await
            .expect("couldn't read response body")
            .as_str(),
    )
    .expect("couldn't parse aetherhub aetherhub_decklist response")
    .converted_deck
    .into_iter()
    .filter_map(|card| {
        if card.name != "" {
            Some(AetherhubCard {
                quantity: card.quantity,
                name: card.name,
            })
        } else {
            None
        }
    })
    .collect();

    fn convert_aetherhub_decklist(decklist: Vec<AetherhubCard>) -> Vec<CardInDeck> {
        let mut result = Vec::new();
        let mut is_commander = false;
        let mut is_companion = false;

        for card in decklist {
            if card.name == "Commander" {
                is_commander = true;
                is_companion = false;
            } else if card.name == "Companion" {
                is_companion = true;
                is_commander = false;
            } else if card.name == "Deck" {
                is_commander = false;
                is_companion = false;
            } else if card.name.contains(" /// ") {
                let (front, back): (&str, &str) = card.name.split_once(" /// ").unwrap();
                let (front, back) = (
                    front.strip_prefix("A-").unwrap_or(front),
                    back.strip_prefix("A-").unwrap_or(back),
                );
                let (name, a_name) = (
                    format!("{} // {}", front, back),
                    format!("A-{} // A-{}", front, back),
                );
                result.push(CardInDeck {
                    quantity: card.quantity,
                    name: name.clone(),
                    a_name: a_name.clone(),
                    is_commander,
                    is_companion,
                })
            } else {
                let name = card
                    .name
                    .strip_prefix("A-")
                    .unwrap_or(&card.name)
                    .to_string();
                result.push(CardInDeck {
                    quantity: card.quantity,
                    name: name.clone(),
                    a_name: format!("A-{name}"),
                    is_commander,
                    is_companion,
                });
            }
        }
        result
    }

    let aetherhub_decklist = convert_aetherhub_decklist(aetherhub_decklist);

    // Always search for alchemy version first, if not, then search for non-alchemy version
    // DFC cards do not have the '// Back Half'
    // Eg. "Sheoldred // The True Scriptures" -> "Sheoldred"
    // Aftermath cards DO have both halfs, seperated by '///'
    // Eg. "Cut /// Ribbons"
    // No alchemy-aftermath cards exist yet, so I don't know what they would look like.

    let card_ids = aetherhub_decklist.iter().map(|card| async {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct OracleId {
            oracle_id: Uuid,
            name_full: String,
            color_identity: Vec<String>,
        }

        // println!("Card: {}", card.name);

        let result = sqlx::query_as!(
            OracleId,
            "SELECT name_full, oracle_id, color_identity 
            FROM card
            WHERE unaccent(name_full) = unaccent($1)
            OR (unaccent(name_front) = unaccent($1) AND layout IN ('transform','modal_dfc', 'adventure')
        )",
            card.name
        )
        .fetch_one(pool)
        .await;

        if let Ok(res) = result {
            Some(CombinedCardData {
                oracle_id: res.oracle_id,
                name: res.name_full,
                is_commander: card.is_commander,
                is_companion: card.is_companion,
                quantity: card.quantity,
                color_identity: res.color_identity,
            })
        } else {
            eprintln!("Error for card {}", card.name);
            None
        }
    
        // The result is wrapped in an Option here, filtering out None (skipping the entry)

        // sqlx::query_as!(
        //     OracleId,
        //     "SELECT name, oracle_id, color_identity FROM (
        //     SELECT oracle_id, name, color_identity, 1 AS priority FROM card WHERE unaccent(name) LIKE unaccent($1)
        //     UNION SELECT oracle_id, name, color_identity, 2 AS priority FROM card WHERE unaccent(name) LIKE unaccent($2)
        //     UNION SELECT oracle_id, name, color_identity, 3 AS priority FROM card WHERE unaccent(name) = unaccent($3)
        //     UNION SELECT oracle_id, name, color_identity, 4 AS priority FROM card WHERE unaccent(name) = unaccent($4)
        //     ) as result
        //     ORDER BY priority",
        //     alchemy_flip, // Search for alchemy flip cards with "A-name //%"
        //     flip,         // Search for regular flip cards with "name //%"
        //     card.a_name,  // Search for alchemy card with "A-name"
        //     card.name     // Search for regular card with "name"
        // )
        // .fetch_optional(pool)
        // .await
        // .unwrap_or_else(|_| panic!("Error when querying db for {}", card.name))
        // .unwrap_or_else(|| panic!("Couldn't find oracle_id of card {}", card.name))
    });

    let card_ids = join_all(card_ids).await;
    let combined_card_data: Vec<CombinedCardData> = card_ids.into_iter().filter_map(|card| card).collect();

    #[derive(Debug)]
    struct CombinedCardData {
        oracle_id: Uuid,
        name: String,
        is_commander: bool,
        is_companion: bool,
        quantity: Option<i32>,
        color_identity: Vec<String>,
    }

    // let combined_card_data: Vec<CombinedCardData> = aetherhub_decklist
    //     .into_iter()
    //     .zip(card_ids)
    //     .map(|(decklist_card, card_id)| CombinedCardData {
    //         oracle_id: card_id.oracle_id,
    //         name: decklist_card.name,
    //         is_commander: decklist_card.is_commander,
    //         is_companion: decklist_card.is_companion,
    //         quantity: decklist_card.quantity,
    //         color_identity: card_id.color_identity,
    //     })
    //     .collect();

    struct DeckID {
        id: i32,
    }

    let commander_info = combined_card_data
        .iter()
        .find(|card| card.is_commander)
        .unwrap();

    let companion = combined_card_data.iter().find(|card| card.is_companion);

    sqlx::query_as!(
        AetherHubDeck,
        "INSERT INTO deck (id, deck_id, url, username, date_created, date_updated, commander, color_identity, companion)
        VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (deck_id) DO NOTHING",
        // Uuid::parse_str(&deck.id).expect("uuid parsed wrong"),
        deck.id,
        deck.url,
        deck.username,
        deck.created,
        deck.updated,
        commander_info.oracle_id,
        // companion.unwrap_or(Null),
        &commander_info.color_identity,
        companion.map(|c| c.oracle_id),
    )
    .execute(pool)
    .await
    .expect("insert deck into db failed");

    let deck_id: DeckID =
        sqlx::query_as!(DeckID, "SELECT id FROM deck WHERE deck_id = $1", deck.id)
            .fetch_one(pool)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "couldn't find primary key of deck with deck_id = {}",
                    deck.id
                )
            });

    for card in combined_card_data {
        // let deck_id = Uuid::parse_str(deck.id.as_str()).expect("uuid parsed wrong");
        // println!(
        //     "Insert {}, {} into {}",
        //     card.name, card.oracle_id, deck_id.id
        // );
        sqlx::query!(
            "INSERT INTO decklist (oracle_id, deck_id, quantity, is_companion, is_commander)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (oracle_id, deck_id) DO UPDATE
            SET quantity = decklist.quantity + $3",
            card.oracle_id,
            deck_id.id,
            card.quantity,
            card.is_companion,
            card.is_commander
        )
        .execute(pool)
        .await
        .expect("insert card failed");
    }
}

async fn get_aetherhub_decks(start: i32, length: i32) -> Vec<AetherHubDeck> {
    let mut request_data: String = String::from(
        r#"
      {
        "draw": 4,
        "columns": [
          {
            "data": "name",
            "name": "name",
            "searchable": true,
            "orderable": false,
            "search": {
              "value": "",
              "regex": false
            }
          },
          {
            "data": "color",
            "name": "color",
            "searchable": true,
            "orderable": false,
            "search": {
              "value": "",
              "regex": false
            }
          },
          {
            "data": "tags",
            "name": "tags",
            "searchable": true,
            "orderable": false,
            "search": {
              "value": "",
              "regex": false
            }
          },
          {
            "data": "rarity",
            "name": "rarity",
            "searchable": true,
            "orderable": false,
            "search": {
              "value": "",
              "regex": false
            }
          },
          {
            "data": "price",
            "name": "price",
            "searchable": true,
            "orderable": false,
            "search": {
              "value": "",
              "regex": false
            }
          },
          {
            "data": "views",
            "name": "views",
            "searchable": true,
            "orderable": true,
            "search": {
              "value": "",
              "regex": false
            }
          },
          {
            "data": "exports",
            "name": "exports",
            "searchable": true,
            "orderable": true,
            "search": {
              "value": "",
              "regex": false
            }
          },
          {
            "data": "updated",
            "name": "updated",
            "searchable": true,
            "orderable": true,
            "search": {
              "value": "365",
              "regex": false
            }
          },
          {
            "data": "updatedhidden",
            "name": "updatedhidden",
            "searchable": false,
            "orderable": true,
            "search": {
              "value": "",
              "regex": false
            }
          },
          {
            "data": "popularity",
            "name": "popularity",
            "searchable": false,
            "orderable": true,
            "search": {
              "value": "",
              "regex": false
            }
          }
        ],
        "order": [
          {
            "column": 7,
            "dir": "desc"
          }
        ],
        "search": {
          "value": "",
          "regex": false
        }
    "#,
    );

    let start = format!(",\n\"start\": {},\n", start);
    let length = format!("\"length\": {}\n}}", length);
    request_data.push_str(&start);
    request_data.push_str(&length);

    let res = reqwest::Client::new()
        .post("https://aetherhub.com/Meta/FetchMetaListAdv?formatId=19")
        .header("Content-Type", "application/json")
        .body(request_data)
        .send()
        .await
        .expect("couldn't send Post request")
        .text()
        .await
        .expect("couldn't read response body");

    #[derive(Deserialize, Debug)]
    struct Response {
        metadecks: Vec<AetherHubDeck>,
    }

    serde_json::from_str::<Response>(&res)
        .expect("unable to parse JSON")
        .metadecks
}

#[derive(Serialize, Deserialize, Debug)]
struct Deck {
    id: i32,
    ah_deck_id: i32,
    url: String,
    username: String,
    date_created: i64,
    date_updated: i64,
    commander: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AetherHubDeck {
    id: i32,
    name: String,
    url: String,
    username: String,
    updated: i64,
    created: i64,
}

#[derive(Serialize, Deserialize, Debug)]
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
    lowest_rarity: String,
}

#[derive( Deserialize, Debug, Clone)]
#[serde(tag = "layout", rename_all = "snake_case")]
enum ScryfallCard {
    Normal(Normal),
    Split(Split),
    Flip(Flip),
    Transform(Transform),
    #[serde(rename = "modal_dfc")]
    ModalDFC(ModalDFC),
    Meld(Meld),
    Leveler(Normal),
    Class(Normal),
    Saga(Normal),
    Adventure(Adventure),
    Mutate(Normal),
    Prototype(Normal),
    Planar(Normal),
    Scheme(Normal),
    Vanguard(Normal),
    Token(Normal),
    DoubleFacedToken(DoubleFacedToken),
    Emblem(Normal),
    Augment(Normal),
    Host(Normal),
    ArtSeries(ArtSeries),
    ReversibleCard(ReversibleCard),
}

fn default_lowest_rarity() -> String {
    "".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Normal {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    mana_cost: Option<String>,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Split {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    mana_cost: Option<String>,
    // oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    card_faces: Vec<SplitFace>,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Flip {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    mana_cost: Option<String>,
    // oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    card_faces: Vec<FlipFace>,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transform {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    // mana_cost: Option<String>,
    // oracle_text: Option<String>,
    // colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    // image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    card_faces: Vec<TransformFace>,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModalDFC {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    // mana_cost: Option<String>,
    // oracle_text: Option<String>,
    // colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    // image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    card_faces: Vec<ModalDFCFace>,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Meld {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    mana_cost: Option<String>,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    all_parts: Vec<MeldPart>,
    id: String,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Adventure {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    mana_cost: Option<String>,
    // oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    card_faces: Vec<AdventureFace>,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DoubleFacedToken {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    // mana_cost: Option<String>,
    // oracle_text: Option<String>,
    // colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    // image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    card_faces: Vec<DoubleFacedTokenFace>,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArtSeries {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    oracle_id: String,
    cmc: f32,
    name: String,
    // mana_cost: Option<String>,
    // oracle_text: Option<String>,
    // colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    // image_uris: CardImages,
    type_line: String,
    legalities: Legalities,
    set_type: String,
    card_faces: Vec<ArtSeriesFace>,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ReversibleCard {
    lang: String,
    released_at: NaiveDate,
    arena_id: Option<i32>,
    scryfall_uri: String,
    // oracle_id: String,
    // cmc: f32,
    name: String,
    // mana_cost: Option<String>,
    // oracle_text: Option<String>,
    // colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    rarity: String,
    games: Vec<String>,
    // image_uris: CardImages,
    // type_line: String,
    legalities: Legalities,
    set_type: String,
    card_faces: Vec<ReversibleCardFace>,
    promo_types: Option<Vec<String>>,
    #[serde(skip_deserializing, default = "default_lowest_rarity")]
    lowest_rarity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SplitFace {
    name: String,
    mana_cost: String,
    type_line: String,
    oracle_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FlipFace {
    name: String,
    mana_cost: String,
    type_line: String,
    oracle_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TransformFace {
    name: String,
    mana_cost: String,
    type_line: String,
    oracle_text: String,
    colors: Option<Vec<String>>,
    image_uris: CardImages,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModalDFCFace {
    name: String,
    mana_cost: String,
    type_line: String,
    oracle_text: String,
    colors: Option<Vec<String>>,
    image_uris: CardImages,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MeldPart {
    name: String,
    type_line: String,
    component: String, // "meld_part" or "meld_result"
    uri: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AdventureFace {
    name: String,
    mana_cost: String,
    type_line: String,
    oracle_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DoubleFacedTokenFace {
    name: String,
    mana_cost: String,
    oracle_text: String,
    type_line: Option<String>,
    colors: Option<Vec<String>>,
    image_uris: CardImages,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArtSeriesFace {
    name: String,
    mana_cost: String,
    type_line: String,
    oracle_text: String,
    colors: Option<Vec<String>>,
    image_uris: Option<CardImages>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ReversibleCardFace {
    name: String,
    oracle_id: String,
    mana_cost: String,
    cmc: f32,
    type_line: String,
    oracle_text: String,
    colors: Option<Vec<String>>,
    image_uris: CardImages,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CardImages {
    small: String,
    normal: String,
    large: String,
    png: String,
    art_crop: String,
    border_crop: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Legalities {
    brawl: String,
    historicbrawl: String,
}

trait ScryfallCardProperties {
    fn name(&self) -> String;
    fn oracle_id(&self) -> String;
    fn released_at(&self) -> &NaiveDate;
    fn games(&self) -> &Vec<String>;
    // fn set_type(&self) -> String;
    // fn type_line(&self) -> String;
    fn promo_types(&self) -> &Option<Vec<String>>;
    // fn is_legal_commander(&self) -> bool;
    // fn strip_alchemy_prefix(&self) -> String;
    fn is_rebalanced(&self) -> bool;
    fn layout(&self) -> String;
    // fn is_legal(&self) -> bool;
    // fn slug(&self) -> String;
    // fn to_card(&self) -> Card;
    fn rarity(&self) -> String;
    fn lowest_rarity(&self) -> String;
    fn set_lowest_rarity(&mut self, new_lowest_rarity: String);
}

// fn split_name(name: &str) -> (String, String) {
//     let (front, back): (&str, &str) = name.split_once(" // ").expect("Split name at ' // '");
//     (front.to_string(), back.to_string())
// }

fn is_legal_commander(type_line: &str) -> bool {
    let lowercase_type_line = type_line.to_lowercase();
    lowercase_type_line.contains("legendary") && lowercase_type_line.contains("creature")
        || lowercase_type_line.contains("planeswalker")
}

fn slug(name: &str) -> String {
    let name = name.strip_prefix("A-").unwrap_or(name);
    slugify(name.split(" // ").next().unwrap_or(name))
}

fn strip_alchemy_prefix(name: &str) -> String {
    if name.starts_with("A-") {
        if name.contains("//") {
            name.split(" // ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|c| {
                    c.strip_prefix("A-")
                        .expect("Strip 'A-' prefix from split card")
                })
                .collect::<Vec<&str>>()
                .join(" // ")
        } else {
            name.strip_prefix("A-")
                .expect("Strip A- prefix")
                .to_string()
        }
    } else {
        name.to_string()
    }
}


impl ScryfallCardProperties for ScryfallCard {
    fn name(&self) -> String {
        match self {
            ScryfallCard::Normal(normal) => normal.name(),
            ScryfallCard::Split(split) => split.name(),
            ScryfallCard::Flip(flip) => flip.name(),
            ScryfallCard::Transform(transform) => transform.name(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.name(),
            ScryfallCard::Meld(meld) => meld.name(),
            ScryfallCard::Leveler(normal) => normal.name(),
            ScryfallCard::Class(normal) => normal.name(),
            ScryfallCard::Saga(normal) => normal.name(),
            ScryfallCard::Adventure(adventure) => adventure.name(),
            ScryfallCard::Mutate(normal) => normal.name(),
            ScryfallCard::Prototype(normal) => normal.name(),
            ScryfallCard::Planar(normal) => normal.name(),
            ScryfallCard::Scheme(normal) => normal.name(),
            ScryfallCard::Vanguard(normal) => normal.name(),
            ScryfallCard::Token(normal) => normal.name(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.name(),
            ScryfallCard::Emblem(normal) => normal.name(),
            ScryfallCard::Augment(normal) => normal.name(),
            ScryfallCard::Host(normal) => normal.name(),
            ScryfallCard::ArtSeries(art_series) => art_series.name(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.name(),
        }
    }

    fn oracle_id(&self) -> String {
        match self {
            ScryfallCard::Normal(normal) => normal.oracle_id(),
            ScryfallCard::Split(split) => split.oracle_id(),
            ScryfallCard::Flip(flip) => flip.oracle_id(),
            ScryfallCard::Transform(transform) => transform.oracle_id(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.oracle_id(),
            ScryfallCard::Meld(meld) => meld.oracle_id(),
            ScryfallCard::Leveler(normal) => normal.oracle_id(),
            ScryfallCard::Class(normal) => normal.oracle_id(),
            ScryfallCard::Saga(normal) => normal.oracle_id(),
            ScryfallCard::Adventure(adventure) => adventure.oracle_id(),
            ScryfallCard::Mutate(normal) => normal.oracle_id(),
            ScryfallCard::Prototype(normal) => normal.oracle_id(),
            ScryfallCard::Planar(normal) => normal.oracle_id(),
            ScryfallCard::Scheme(normal) => normal.oracle_id(),
            ScryfallCard::Vanguard(normal) => normal.oracle_id(),
            ScryfallCard::Token(normal) => normal.oracle_id(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.oracle_id(),
            ScryfallCard::Emblem(normal) => normal.oracle_id(),
            ScryfallCard::Augment(normal) => normal.oracle_id(),
            ScryfallCard::Host(normal) => normal.oracle_id(),
            ScryfallCard::ArtSeries(art_series) => art_series.oracle_id(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.oracle_id(),
        }
    }

    fn released_at(&self) -> &NaiveDate {
        match self {
            ScryfallCard::Normal(normal) => normal.released_at(),
            ScryfallCard::Split(split) => split.released_at(),
            ScryfallCard::Flip(flip) => flip.released_at(),
            ScryfallCard::Transform(transform) => transform.released_at(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.released_at(),
            ScryfallCard::Meld(meld) => meld.released_at(),
            ScryfallCard::Leveler(normal) => normal.released_at(),
            ScryfallCard::Class(normal) => normal.released_at(),
            ScryfallCard::Saga(normal) => normal.released_at(),
            ScryfallCard::Adventure(adventure) => adventure.released_at(),
            ScryfallCard::Mutate(normal) => normal.released_at(),
            ScryfallCard::Prototype(normal) => normal.released_at(),
            ScryfallCard::Planar(normal) => normal.released_at(),
            ScryfallCard::Scheme(normal) => normal.released_at(),
            ScryfallCard::Vanguard(normal) => normal.released_at(),
            ScryfallCard::Token(normal) => normal.released_at(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.released_at(),
            ScryfallCard::Emblem(normal) => normal.released_at(),
            ScryfallCard::Augment(normal) => normal.released_at(),
            ScryfallCard::Host(normal) => normal.released_at(),
            ScryfallCard::ArtSeries(art_series) => art_series.released_at(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.released_at(),
        }
    }

    fn games(&self) -> &Vec<String> {
        match self {
            ScryfallCard::Normal(normal) => normal.games(),
            ScryfallCard::Split(split) => split.games(),
            ScryfallCard::Flip(flip) => flip.games(),
            ScryfallCard::Transform(transform) => transform.games(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.games(),
            ScryfallCard::Meld(meld) => meld.games(),
            ScryfallCard::Leveler(normal) => normal.games(),
            ScryfallCard::Class(normal) => normal.games(),
            ScryfallCard::Saga(normal) => normal.games(),
            ScryfallCard::Adventure(adventure) => adventure.games(),
            ScryfallCard::Mutate(normal) => normal.games(),
            ScryfallCard::Prototype(normal) => normal.games(),
            ScryfallCard::Planar(normal) => normal.games(),
            ScryfallCard::Scheme(normal) => normal.games(),
            ScryfallCard::Vanguard(normal) => normal.games(),
            ScryfallCard::Token(normal) => normal.games(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.games(),
            ScryfallCard::Emblem(normal) => normal.games(),
            ScryfallCard::Augment(normal) => normal.games(),
            ScryfallCard::Host(normal) => normal.games(),
            ScryfallCard::ArtSeries(art_series) => art_series.games(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.games(),
        }
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        match self {
            ScryfallCard::Normal(normal) => normal.promo_types(),
            ScryfallCard::Split(split) => split.promo_types(),
            ScryfallCard::Flip(flip) => flip.promo_types(),
            ScryfallCard::Transform(transform) => transform.promo_types(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.promo_types(),
            ScryfallCard::Meld(meld) => meld.promo_types(),
            ScryfallCard::Leveler(normal) => normal.promo_types(),
            ScryfallCard::Class(normal) => normal.promo_types(),
            ScryfallCard::Saga(normal) => normal.promo_types(),
            ScryfallCard::Adventure(adventure) => adventure.promo_types(),
            ScryfallCard::Mutate(normal) => normal.promo_types(),
            ScryfallCard::Prototype(normal) => normal.promo_types(),
            ScryfallCard::Planar(normal) => normal.promo_types(),
            ScryfallCard::Scheme(normal) => normal.promo_types(),
            ScryfallCard::Vanguard(normal) => normal.promo_types(),
            ScryfallCard::Token(normal) => normal.promo_types(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.promo_types(),
            ScryfallCard::Emblem(normal) => normal.promo_types(),
            ScryfallCard::Augment(normal) => normal.promo_types(),
            ScryfallCard::Host(normal) => normal.promo_types(),
            ScryfallCard::ArtSeries(art_series) => art_series.promo_types(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.promo_types(),
        }
    }

    fn layout(&self) -> String {
        match self {
            ScryfallCard::Normal(normal) => "normal".to_string(),
            ScryfallCard::Split(split) => split.layout(),
            ScryfallCard::Flip(flip) => flip.layout(),
            ScryfallCard::Transform(transform) => transform.layout(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.layout(),
            ScryfallCard::Meld(meld) => meld.layout(),
            ScryfallCard::Leveler(normal) => "leveler".to_string(),
            ScryfallCard::Class(normal) => "class".to_string(),
            ScryfallCard::Saga(normal) => "saga".to_string(),
            ScryfallCard::Adventure(adventure) => adventure.layout(),
            ScryfallCard::Mutate(normal) => "mutate".to_string(),
            ScryfallCard::Prototype(normal) => "prototype".to_string(),
            ScryfallCard::Planar(normal) => "planar".to_string(),
            ScryfallCard::Scheme(normal) => "scheme".to_string(),
            ScryfallCard::Vanguard(normal) => "vanguard".to_string(),
            ScryfallCard::Token(normal) => "token".to_string(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.layout(),
            ScryfallCard::Emblem(normal) => "emblem".to_string(),
            ScryfallCard::Augment(normal) => "augment".to_string(),
            ScryfallCard::Host(normal) => "host".to_string(),
            ScryfallCard::ArtSeries(art_series) => art_series.layout(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.layout(),
        }
    }

    fn is_rebalanced(&self) -> bool {
        match self {
            ScryfallCard::Normal(normal) => normal.is_rebalanced(),
            ScryfallCard::Split(split) => split.is_rebalanced(),
            ScryfallCard::Flip(flip) => flip.is_rebalanced(),
            ScryfallCard::Transform(transform) => transform.is_rebalanced(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.is_rebalanced(),
            ScryfallCard::Meld(meld) => meld.is_rebalanced(),
            ScryfallCard::Leveler(normal) => normal.is_rebalanced(),
            ScryfallCard::Class(normal) => normal.is_rebalanced(),
            ScryfallCard::Saga(normal) => normal.is_rebalanced(),
            ScryfallCard::Adventure(adventure) => adventure.is_rebalanced(),
            ScryfallCard::Mutate(normal) => normal.is_rebalanced(),
            ScryfallCard::Prototype(normal) => normal.is_rebalanced(),
            ScryfallCard::Planar(normal) => normal.is_rebalanced(),
            ScryfallCard::Scheme(normal) => normal.is_rebalanced(),
            ScryfallCard::Vanguard(normal) => normal.is_rebalanced(),
            ScryfallCard::Token(normal) => normal.is_rebalanced(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => {
                double_faced_token.is_rebalanced()
            }
            ScryfallCard::Emblem(normal) => normal.is_rebalanced(),
            ScryfallCard::Augment(normal) => normal.is_rebalanced(),
            ScryfallCard::Host(normal) => normal.is_rebalanced(),
            ScryfallCard::ArtSeries(art_series) => art_series.is_rebalanced(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.is_rebalanced(),
        }
    }

    fn rarity(&self) -> String {
        match self {
            ScryfallCard::Normal(normal) => normal.rarity(),
            ScryfallCard::Split(split) => split.rarity(),
            ScryfallCard::Flip(flip) => flip.rarity(),
            ScryfallCard::Transform(transform) => transform.rarity(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.rarity(),
            ScryfallCard::Meld(meld) => meld.rarity(),
            ScryfallCard::Leveler(normal) => normal.rarity(),
            ScryfallCard::Class(normal) => normal.rarity(),
            ScryfallCard::Saga(normal) => normal.rarity(),
            ScryfallCard::Adventure(adventure) => adventure.rarity(),
            ScryfallCard::Mutate(normal) => normal.rarity(),
            ScryfallCard::Prototype(normal) => normal.rarity(),
            ScryfallCard::Planar(normal) => normal.rarity(),
            ScryfallCard::Scheme(normal) => normal.rarity(),
            ScryfallCard::Vanguard(normal) => normal.rarity(),
            ScryfallCard::Token(normal) => normal.rarity(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.rarity(),
            ScryfallCard::Emblem(normal) => normal.rarity(),
            ScryfallCard::Augment(normal) => normal.rarity(),
            ScryfallCard::Host(normal) => normal.rarity(),
            ScryfallCard::ArtSeries(art_series) => art_series.rarity(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.rarity(),
        }
    }

    fn lowest_rarity(&self) -> String {
        match self {
            ScryfallCard::Normal(normal) => normal.lowest_rarity(),
            ScryfallCard::Split(split) => split.lowest_rarity(),
            ScryfallCard::Flip(flip) => flip.lowest_rarity(),
            ScryfallCard::Transform(transform) => transform.lowest_rarity(),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.lowest_rarity(),
            ScryfallCard::Meld(meld) => meld.lowest_rarity(),
            ScryfallCard::Leveler(normal) => normal.lowest_rarity(),
            ScryfallCard::Class(normal) => normal.lowest_rarity(),
            ScryfallCard::Saga(normal) => normal.lowest_rarity(),
            ScryfallCard::Adventure(adventure) => adventure.lowest_rarity(),
            ScryfallCard::Mutate(normal) => normal.lowest_rarity(),
            ScryfallCard::Prototype(normal) => normal.lowest_rarity(),
            ScryfallCard::Planar(normal) => normal.lowest_rarity(),
            ScryfallCard::Scheme(normal) => normal.lowest_rarity(),
            ScryfallCard::Vanguard(normal) => normal.lowest_rarity(),
            ScryfallCard::Token(normal) => normal.lowest_rarity(),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.lowest_rarity(),
            ScryfallCard::Emblem(normal) => normal.lowest_rarity(),
            ScryfallCard::Augment(normal) => normal.lowest_rarity(),
            ScryfallCard::Host(normal) => normal.lowest_rarity(),
            ScryfallCard::ArtSeries(art_series) => art_series.lowest_rarity(),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.lowest_rarity(),
        }
    }

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        match self {
            ScryfallCard::Normal(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Split(split) => split.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Flip(flip) => flip.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Transform(transform) => transform.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::ModalDFC(modal_dfc) => modal_dfc.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Meld(meld) => meld.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Leveler(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Class(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Saga(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Adventure(adventure) => adventure.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Mutate(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Prototype(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Planar(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Scheme(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Vanguard(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Token(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::DoubleFacedToken(double_faced_token) => double_faced_token.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Emblem(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Augment(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::Host(normal) => normal.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::ArtSeries(art_series) => art_series.set_lowest_rarity(new_lowest_rarity),
            ScryfallCard::ReversibleCard(reversible_card) => reversible_card.set_lowest_rarity(new_lowest_rarity),
        }
    }

}

impl ScryfallCardProperties for Normal {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    // this is not accurate, lots of variants share the 'Normal' type but their layout is not 'normal'
    fn layout(&self) -> String {
        "normal".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }

    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }
}
impl ScryfallCardProperties for Split {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }

    fn layout(&self) -> String {
        "split".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }
}
impl ScryfallCardProperties for Flip {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }

    

    fn layout(&self) -> String {
        "flip".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }
}
impl ScryfallCardProperties for Transform {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }

    

    fn layout(&self) -> String {
        "transform".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }
}
impl ScryfallCardProperties for ModalDFC {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }

    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }

    fn layout(&self) -> String {
        "modal_dfc".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }
}
impl ScryfallCardProperties for Meld {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn layout(&self) -> String {
        "meld".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }
    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }
    
}
impl ScryfallCardProperties for Adventure {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn layout(&self) -> String {
        "adventure".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }
    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }
    
}
impl ScryfallCardProperties for ArtSeries {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn layout(&self) -> String {
        "art_series".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }
    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }
    
}
impl ScryfallCardProperties for DoubleFacedToken {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn layout(&self) -> String {
        "double_faced_token".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }

    fn oracle_id(&self) -> String {
        self.oracle_id.clone()
    }
    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }
    
}
impl ScryfallCardProperties for ReversibleCard {

    fn set_lowest_rarity(&mut self, new_lowest_rarity: String) {
        self.lowest_rarity = new_lowest_rarity;
    }

    fn layout(&self) -> String {
        "reversible_card".to_string()
    }

    fn is_rebalanced(&self) -> bool {
        if self.promo_types.is_some() {
            self.promo_types
                .as_ref()
                .unwrap()
                .contains(&"rebalanced".to_string())
        } else {
            false
        }
    }

    fn games(&self) -> &Vec<String> {
        &self.games
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn oracle_id(&self) -> String {
        self.card_faces[0].oracle_id.clone()
    }

    fn promo_types(&self) -> &Option<Vec<String>> {
        &self.promo_types
    }

    fn released_at(&self) -> &NaiveDate {
        &self.released_at
    }
    fn rarity(&self) -> String {
        self.rarity.clone()
    }

    fn lowest_rarity(&self) -> String {
        self.lowest_rarity.clone()
    }
    
}

impl From<ScryfallCard> for Card {
    fn from(card: ScryfallCard) -> Self {
        match card {
            ScryfallCard::Normal(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "normal".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Split(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.card_faces[0].name),
                name_back: Some(strip_alchemy_prefix(&c.card_faces[1].name)),
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: c.mana_cost.clone(),
                mana_cost_front: Some(c.card_faces[0].mana_cost.clone()),
                mana_cost_back: Some(c.card_faces[1].mana_cost.clone()),
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.card_faces[0].type_line.clone(),
                type_line_back: Some(c.card_faces[1].type_line.clone()),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                oracle_text_back: Some(c.card_faces[1].oracle_text.clone()),
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Flip(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.card_faces[0].name),
                name_back: Some(strip_alchemy_prefix(&c.card_faces[1].name)),
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.card_faces[0].type_line.clone(),
                type_line_back: Some(c.card_faces[1].type_line.clone()),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                oracle_text_back: Some(c.card_faces[1].oracle_text.clone()),
                colors: c.colors.clone(),
                colors_back: c.colors.clone(),
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Transform(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.card_faces[0].name),
                name_back: Some(strip_alchemy_prefix(&c.card_faces[1].name)),
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: None,
                mana_cost_front: Some(c.card_faces[0].mana_cost.clone()),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.card_faces[0].type_line.clone(),
                type_line_back: Some(c.card_faces[1].type_line.clone()),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                oracle_text_back: Some(c.card_faces[1].oracle_text.clone()),
                colors: c.card_faces[0].colors.clone(),
                colors_back: c.card_faces[1].colors.clone(),
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.card_faces[0].image_uris.small.clone(),
                image_normal: c.card_faces[0].image_uris.normal.clone(),
                image_large: c.card_faces[0].image_uris.large.clone(),
                image_art_crop: c.card_faces[0].image_uris.art_crop.clone(),
                image_border_crop: c.card_faces[0].image_uris.border_crop.clone(),
                image_small_back: Some(c.card_faces[1].image_uris.small.clone()),
                image_normal_back: Some(c.card_faces[1].image_uris.normal.clone()),
                image_large_back: Some(c.card_faces[1].image_uris.large.clone()),
                image_art_crop_back: Some(c.card_faces[1].image_uris.border_crop.clone()),
                image_border_crop_back: Some(c.card_faces[1].image_uris.art_crop.clone()),
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::ModalDFC(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.card_faces[0].name),
                name_back: Some(strip_alchemy_prefix(&c.card_faces[1].name)),
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: None,
                mana_cost_front: Some(c.card_faces[0].mana_cost.clone()),
                mana_cost_back: Some(c.card_faces[1].mana_cost.clone()),
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.card_faces[0].type_line.clone(),
                type_line_back: Some(c.card_faces[1].type_line.clone()),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                oracle_text_back: Some(c.card_faces[1].oracle_text.clone()),
                colors: c.card_faces[0].colors.clone(),
                colors_back: c.card_faces[1].colors.clone(),
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.card_faces[0].image_uris.small.clone(),
                image_normal: c.card_faces[0].image_uris.normal.clone(),
                image_large: c.card_faces[0].image_uris.large.clone(),
                image_art_crop: c.card_faces[0].image_uris.art_crop.clone(),
                image_border_crop: c.card_faces[0].image_uris.border_crop.clone(),
                image_small_back: Some(c.card_faces[1].image_uris.small.clone()),
                image_normal_back: Some(c.card_faces[1].image_uris.normal.clone()),
                image_large_back: Some(c.card_faces[1].image_uris.large.clone()),
                image_art_crop_back: Some(c.card_faces[1].image_uris.border_crop.clone()),
                image_border_crop_back: Some(c.card_faces[1].image_uris.art_crop.clone()),
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Meld(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.all_parts[0].name),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line)
                    && c
                        .all_parts
                        .iter()
                        .find(|part| part.name == c.name())
                        .expect("Find Meld part associated to this card")
                        .component
                        != "meld_result".to_string(),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Leveler(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "leveler".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Class(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "class".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Saga(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "saga".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Adventure(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.card_faces[0].name),
                name_back: Some(strip_alchemy_prefix(&c.card_faces[1].name)),
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: c.mana_cost.clone(),
                mana_cost_front: Some(c.card_faces[0].mana_cost.clone()),
                mana_cost_back: Some(c.card_faces[1].mana_cost.clone()),
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.card_faces[0].type_line.clone(),
                type_line_back: Some(c.card_faces[1].type_line.clone()),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                oracle_text_back: Some(c.card_faces[1].oracle_text.clone()),
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.card_faces[1].type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Mutate(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "adventure".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Prototype(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "prototype".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Planar(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "planar".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Scheme(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "scheme".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Vanguard(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "vanguard".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Token(c) =>
                Card {
                    oracle_id: c.oracle_id(),
                    slug: slug(&c.name()),
                    name_full: strip_alchemy_prefix(&c.name()),
                    name_front: strip_alchemy_prefix(&c.name()),
                    name_back: None,
                    lang: c.lang.clone(),
                    scryfall_uri: c.scryfall_uri.clone(),
                    layout: "token".to_string(),
                    mana_cost_combined: None,
                    mana_cost_front: c.mana_cost.clone(),
                    mana_cost_back: None,
                    cmc: c.cmc,
                    type_line_full: c.type_line.clone(),
                    type_line_front: c.type_line.clone(),
                    type_line_back: None,
                    oracle_text: c.oracle_text.clone(),
                    oracle_text_back: None,
                    colors: c.colors.clone(),
                    colors_back: None,
                    color_identity: c.color_identity.clone(),
                    is_legal: c.legalities.historicbrawl == "legal",
                    is_legal_commander: is_legal_commander(&c.type_line),
                    is_rebalanced: c.is_rebalanced(),
                    rarity: c.rarity.clone(),
                    image_small: c.image_uris.small.clone(),
                    image_normal: c.image_uris.normal.clone(),
                    image_large: c.image_uris.large.clone(),
                    image_art_crop: c.image_uris.art_crop.clone(),
                    image_border_crop: c.image_uris.border_crop.clone(),
                    image_small_back: None,
                    image_normal_back: None,
                    image_large_back: None,
                    image_art_crop_back: None,
                    image_border_crop_back: None,
                    lowest_rarity: c.lowest_rarity.clone(),
                },
            ScryfallCard::DoubleFacedToken(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.card_faces[0].name),
                name_back: Some(strip_alchemy_prefix(&c.card_faces[1].name)),
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: None,
                mana_cost_front: None,
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: Some(c.type_line.clone()),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                oracle_text_back: Some(c.card_faces[1].oracle_text.clone()),
                colors: None,
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: false,
                is_legal_commander: false,
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.card_faces[0].image_uris.small.clone(),
                image_normal: c.card_faces[0].image_uris.normal.clone(),
                image_large: c.card_faces[0].image_uris.large.clone(),
                image_art_crop: c.card_faces[0].image_uris.art_crop.clone(),
                image_border_crop: c.card_faces[0].image_uris.border_crop.clone(),
                image_small_back: Some(c.card_faces[1].image_uris.small.clone()),
                image_normal_back: Some(c.card_faces[1].image_uris.normal.clone()),
                image_large_back: Some(c.card_faces[1].image_uris.large.clone()),
                image_art_crop_back: Some(c.card_faces[1].image_uris.border_crop.clone()),
                image_border_crop_back: Some(c.card_faces[1].image_uris.art_crop.clone()),
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Emblem(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "emblem".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Augment(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "augment".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::Host(c) => 
            Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.name()),
                name_back: None,
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: "host".to_string(),
                mana_cost_combined: None,
                mana_cost_front: c.mana_cost.clone(),
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.type_line.clone(),
                type_line_back: None,
                oracle_text: c.oracle_text.clone(),
                oracle_text_back: None,
                colors: c.colors.clone(),
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: c.legalities.historicbrawl == "legal",
                is_legal_commander: is_legal_commander(&c.type_line),
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.image_uris.small.clone(),
                image_normal: c.image_uris.normal.clone(),
                image_large: c.image_uris.large.clone(),
                image_art_crop: c.image_uris.art_crop.clone(),
                image_border_crop: c.image_uris.border_crop.clone(),
                image_small_back: None,
                image_normal_back: None,
                image_large_back: None,
                image_art_crop_back: None,
                image_border_crop_back: None,
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::ArtSeries(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.card_faces[0].name),
                name_back: Some(strip_alchemy_prefix(&c.card_faces[1].name)),
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: None,
                mana_cost_front: None,
                mana_cost_back: None,
                cmc: c.cmc,
                type_line_full: c.type_line.clone(),
                type_line_front: c.card_faces[0].type_line.clone(),
                type_line_back: Some(c.card_faces[1].type_line.clone()),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                oracle_text_back: Some(c.card_faces[1].oracle_text.clone()),
                colors: None,
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: false,
                is_legal_commander: false,
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.small.clone(),
                ),
                image_normal: c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.normal.clone(),
                ),
                image_large: c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.large.clone(),
                ),
                image_art_crop: c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.art_crop.clone(),
                ),
                image_border_crop: c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.border_crop.clone(),
                ),
                image_small_back: Some(c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.small.clone(),
                )),
                image_normal_back: Some(c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.normal.clone(),
                )),
                image_large_back: Some(c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.large.clone(),
                )),
                image_art_crop_back: Some(c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.art_crop.clone(),
                )),
                image_border_crop_back: Some(c.card_faces[0].image_uris.as_ref().map_or(
                    "https://errors.scryfall.com/missing.jpg".to_string(),
                    |uris| uris.border_crop.clone(),
                )),
                lowest_rarity: c.lowest_rarity.clone(),
            },
            ScryfallCard::ReversibleCard(c) => Card {
                oracle_id: c.oracle_id(),
                slug: slug(&c.name()),
                name_full: strip_alchemy_prefix(&c.name()),
                name_front: strip_alchemy_prefix(&c.card_faces[0].name),
                name_back: Some(strip_alchemy_prefix(&c.card_faces[1].name)),
                lang: c.lang.clone(),
                scryfall_uri: c.scryfall_uri.clone(),
                layout: c.layout(),
                mana_cost_combined: None,
                mana_cost_front: None,
                mana_cost_back: None,
                cmc: c.card_faces[0].cmc,
                type_line_full: c.card_faces[0].type_line.clone(),
                type_line_front: c.card_faces[0].type_line.clone(),
                type_line_back: Some(c.card_faces[1].type_line.clone()),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                oracle_text_back: Some(c.card_faces[1].oracle_text.clone()),
                colors: None,
                colors_back: None,
                color_identity: c.color_identity.clone(),
                is_legal: false,
                is_legal_commander: false,
                is_rebalanced: c.is_rebalanced(),
                rarity: c.rarity.clone(),
                image_small: c.card_faces[0].image_uris.small.clone(),
                image_normal: c.card_faces[0].image_uris.normal.clone(),
                image_large: c.card_faces[0].image_uris.large.clone(),
                image_art_crop: c.card_faces[0].image_uris.art_crop.clone(),
                image_border_crop: c.card_faces[0].image_uris.border_crop.clone(),
                image_small_back: Some(c.card_faces[1].image_uris.small.clone()),
                image_normal_back: Some(c.card_faces[1].image_uris.normal.clone()),
                image_large_back: Some(c.card_faces[1].image_uris.large.clone()),
                image_art_crop_back: Some(c.card_faces[1].image_uris.border_crop.clone()),
                image_border_crop_back: Some(c.card_faces[1].image_uris.art_crop.clone()),
                lowest_rarity: c.lowest_rarity.clone(),
            },
        }
    }
}
