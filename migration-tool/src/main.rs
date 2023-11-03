// use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use slug::slugify;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use sqlx::types::Uuid;
use futures::future::join_all;
use std::fs;
use uuid::Uuid;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/brawlhub";

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("couldn't connect to db");

    if false {
        migrate_scryfall_cards(&pool).await;
    }
    let decks = get_aetherhub_decks(0, 10).await;
    for deck in decks {
        migrate_aetherhub_decklists(&pool, &deck).await
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
            } else if card.name.contains("///") {
                let name = card.name.split(" ///").collect::<Vec<&str>>()[0].to_string();
                result.push(CardInDeck {
                    quantity: card.quantity,
                    name: name.clone(),
                    a_name: format!("A-{}", name.clone()),
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
                    a_name: format!("A-{}", name.clone()),
                    is_commander,
                    is_companion,
                });
            }
        }
        result
    }

    let aetherhub_decklist = convert_aetherhub_decklist(aetherhub_decklist);

    // Always search for alchemy version first, if not, then search for non-alchemy version
    // Flip cards do not have the '// Back Half'
    // Eg. "Sheoldred // The True Scriptures" -> "Sheoldred"
    // Aftermath cards DO have both halfs, seperated by '///'
    // Eg. "Cut /// Ribbons"
    // No alchemy-aftermath cards exist yet, so I don't know what they would look like.

    // Card Name = 'Lightning Bolt'
    // 1. Search LIKE 'A-Lightning Bolt //%'
    // 2. Search LIKE 'Lightning Bold //%'
    // 3. Search = 'A-Lightning Bolt'
    // 4. Search = 'Lightning Bolt'

    // ALMOST WORKS, flip alchemy cards not returning alchemy version
    // PROBLEM: "Alrund, God of the Cosmos" and "A-Alrund, God of the Cosmos" -> "Alrund, God of the Cosmos // Hakka, Whispering Raven"

    let card_ids = aetherhub_decklist.iter().map(|card| async {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct OracleId {
            oracle_id: Option<Uuid>,
            name: Option<String>,
        }

        let alchemy_flip = format!("{} //%", card.a_name);
        let flip = format!("{} //%", card.name);

        sqlx::query_as!(
            OracleId,
            "SELECT name, oracle_id FROM (
            SELECT oracle_id, name, 1 AS priority FROM card WHERE unaccent(name) LIKE unaccent($1)
            UNION SELECT oracle_id, name, 2 AS priority FROM card WHERE unaccent(name) LIKE unaccent($2)
            UNION SELECT oracle_id, name, 3 AS priority FROM card WHERE unaccent(name) = unaccent($3)
            UNION SELECT oracle_id, name, 4 AS priority FROM card WHERE unaccent(name) = unaccent($4)
            ) as result
            ORDER BY priority",
            alchemy_flip, // Search for alchemy flip cards with "A-name //%"
            flip,         // Search for regular flip cards with "name //%"
            card.a_name,  // Search for alchemy card with "A-name"
            card.name     // Search for regular card with "name"
        )
        .fetch_optional(pool)
        .await
        .unwrap_or_else(|_| panic!("Error when querying db for {}", card.name))
        .unwrap_or_else(|| panic!("Couldn't find oracle_id of card {}", card.name))
    });

    let card_ids = join_all(card_ids).await;
    // println!("{:#?}", card_ids);

    #[derive(Debug)]
    struct CombinedCardData {
        oracle_id: Option<Uuid>,
        name: String,
        a_name: String,
        is_commander: bool,
        is_companion: bool,
        quantity: Option<i32>,
    }

    let combined_card_data: Vec<CombinedCardData> = aetherhub_decklist
        .into_iter()
        .zip(card_ids)
        .map(|(decklist_card, card_id)| CombinedCardData {
            oracle_id: card_id.oracle_id,
            name: decklist_card.name,
            a_name: decklist_card.a_name,
            is_commander: decklist_card.is_commander,
            is_companion: decklist_card.is_companion,
            quantity: decklist_card.quantity,
        })
        .collect();

    // println!("{:#?}", combined_card_data);

    struct DeckID {
        id: i32,
    }

    let commander = combined_card_data
        .iter()
        .find(|card| card.is_commander)
        .map(|card| {
            card.oracle_id
                .expect("couldn't find oracle_id of card where 'is_commander' = true")
        })
        .unwrap();

    let companion = combined_card_data
        .iter()
        .find(|card| card.is_companion)
        .map(|card| {
            card.oracle_id
                .expect("couldn't find oracle_id of card where 'is_companion' = true")
        });

    sqlx::query_as!(
        AetherHubDeck,
        "INSERT INTO deck (id, deck_id, url, username, date_created, date_updated, commander, companion)
        VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (deck_id) DO NOTHING",
        // Uuid::parse_str(&deck.id).expect("uuid parsed wrong"),
        deck.id,
        deck.url,
        deck.username,
        deck.created,
        deck.updated,
        commander,
        companion
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
        println!("{}, {:#?}, {}", deck.id, card.oracle_id, card.name);
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

async fn migrate_scryfall_cards(pool: &Pool<Postgres>) {
    let data = fs::read_to_string("oracle-cards.json").expect("unable to read JSON");
    let scryfall_cards: Vec<ScryfallCard> =
        serde_json::from_str(&data).expect("unable to parse JSON");

    const UNWANTED_LAYOUTS: [&str; 10] = [
        "planar",
        "scheme",
        "vanguard",
        "token",
        "double_faced_token",
        "emblem",
        "augment",
        "host",
        "art_series",
        "reversible_card",
    ];

    let cards = scryfall_cards
        .into_iter()
        .filter(|card| match card {
            ScryfallCard::Normal(Normal { layout, .. })
            | ScryfallCard::TwoFace(TwoFace { layout, .. }) => {
                !UNWANTED_LAYOUTS.contains(&layout.as_str())
            }
        })
        .map(Card::from)
        .collect::<Vec<Card>>();

    // println!("{:#?}", cards);

    for card in cards {
        sqlx::query_as!(Card, "INSERT INTO card(oracle_id, name, lang, scryfall_uri, layout, mana_cost, cmc, type_line, oracle_text, colors, color_identity, is_legal, is_legal_commander, rarity, image_small, image_normal, image_large, image_art_crop, image_border_crop, slug, is_alchemy)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)",
            Uuid::parse_str(&card.oracle_id).expect("uuid parsed wrong"),
            card.name,
            card.lang,
            card.scryfall_uri,
            card.layout,
            card.mana_cost,
            card.cmc,
            card.type_line,
            card.oracle_text,
            card.colors.as_deref(),
            &card.color_identity,
            card.is_legal,
            card.is_legal_commander,
            card.rarity,
            card.image_small,
            card.image_normal,
            card.image_large,
            card.image_art_crop,
            card.image_border_crop,
            card.slug,
            card.is_alchemy,
        )
        .execute(pool)
        .await
        .expect("couldn't insert");
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
    // id: String,
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
    is_legal_commander: bool,
    rarity: String,
    image_small: String,
    image_normal: String,
    image_large: String,
    image_art_crop: String,
    image_border_crop: String,
    is_alchemy: bool,
    slug: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ScryfallCard {
    Normal(Normal),
    TwoFace(TwoFace),
}

#[derive(Serialize, Deserialize, Debug)]
struct Normal {
    oracle_id: String,
    name: String,
    lang: String,
    scryfall_uri: String,
    layout: String,
    image_uris: CardImages,
    mana_cost: Option<String>,
    cmc: f32,
    type_line: String,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    legalities: Legalaties,
    rarity: String,
    games: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwoFace {
    oracle_id: String,
    name: String,
    lang: String,
    scryfall_uri: String,
    layout: String,
    // mana_cost: Option<String>,
    cmc: f32,
    type_line: String,
    // oracle_text: Option<String>,
    // colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    legalities: Legalaties,
    rarity: String,
    games: Vec<String>,
    card_faces: Vec<CardFace>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardFace {
    mana_cost: String,
    type_line: Option<String>,
    oracle_text: String,
    colors: Option<Vec<String>>,
    image_uris: CardImages,
    name: String,
}

impl From<ScryfallCard> for Card {
    fn from(card: ScryfallCard) -> Self {
        let is_legal_commander = match &card {
            ScryfallCard::Normal(Normal { type_line, .. })
            | ScryfallCard::TwoFace(TwoFace { type_line, .. }) => {
                type_line.to_lowercase().contains("legendary")
                    && type_line.to_lowercase().contains("creature")
                    || type_line.to_lowercase().contains("planeswalker")
            }
        };

        // card.slug is generated from card.name
        // card.slug should not have an 'A-' in front, even if the card is alchemy
        // card.slug should only have the front-half of a card's name

        let slug_sanitized = match &card {
            ScryfallCard::Normal(Normal { name, .. })
            | ScryfallCard::TwoFace(TwoFace { name, .. }) => {
                let slug = name.split(" //").collect::<Vec<&str>>()[0];

                if slug.starts_with("A-") {
                    slugify(slug.strip_prefix("A-").unwrap())
                } else {
                    slugify(slug)
                }
            }
        };

        match card {
            ScryfallCard::Normal(c) => Self {
                // id: c.id,
                oracle_id: c.oracle_id,
                name: c.name.clone(),
                mana_cost: c.mana_cost,
                lang: c.lang,
                scryfall_uri: c.scryfall_uri,
                layout: c.layout,
                // image_uris: c.image_uris,
                cmc: c.cmc,
                type_line: c.type_line.clone(),
                oracle_text: c.oracle_text,
                colors: c.colors,
                color_identity: c.color_identity,
                rarity: c.rarity,
                is_legal: matches!(c.legalities.historicbrawl.as_str(), "legal"),
                is_legal_commander,
                image_small: c.image_uris.small,
                image_normal: c.image_uris.normal,
                image_large: c.image_uris.large,
                image_art_crop: c.image_uris.art_crop,
                image_border_crop: c.image_uris.border_crop,
                is_alchemy: c.name.clone().starts_with("A-"),
                slug: Some(slug_sanitized),
            },
            ScryfallCard::TwoFace(c) => Self {
                oracle_id: c.oracle_id,
                name: c.name.clone(),
                mana_cost: Some(c.card_faces[0].mana_cost.clone()),
                lang: c.lang,
                scryfall_uri: c.scryfall_uri,
                layout: c.layout,
                // image_uris: c.image_uris,
                cmc: c.cmc,
                type_line: c.type_line.clone(),
                oracle_text: Some(c.card_faces[0].oracle_text.clone()),
                colors: c.card_faces[0].colors.clone(),
                color_identity: c.color_identity,
                rarity: c.rarity,
                is_legal: matches!(c.legalities.historicbrawl.as_str(), "legal"),
                is_legal_commander,
                image_small: c.card_faces[0].image_uris.small.clone(),
                image_normal: c.card_faces[0].image_uris.normal.clone(),
                image_large: c.card_faces[0].image_uris.large.clone(),
                image_art_crop: c.card_faces[0].image_uris.art_crop.clone(),
                image_border_crop: c.card_faces[0].image_uris.border_crop.clone(),
                is_alchemy: c.name.clone().starts_with("A-"),
                slug: Some(slug_sanitized),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CardImages {
    small: String,
    normal: String,
    large: String,
    png: String,
    art_crop: String,
    border_crop: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Legalaties {
    brawl: String,
    historicbrawl: String,
}
