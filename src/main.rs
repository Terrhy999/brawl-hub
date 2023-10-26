// use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use sqlx::types::Uuid;
use futures::future::join_all;
use std::fs;
use uuid::Uuid;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost/brawlhub";

#[tokio::main]
async fn main() -> () {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("couldn't connect to db");

    // migrate_scryfall_cards(&pool).await;
    let decks = get_aetherhub_decks(0, 40).await;
    for deck in decks {
        migrate_aetherhub_decklists(&pool, &deck).await
    }
    // migrate_aetherhub_decklists(&pool, &decks[0]).await;
}

async fn migrate_aetherhub_decklists(pool: &Pool<Postgres>, deck: &AetherHubDeck) {
    #[derive(Serialize, Deserialize, Debug)]
    struct CardInDeck {
        quantity: Option<i32>,
        name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        converted_deck: Vec<CardInDeck>,
    }

    let aetherhub_decklist: Vec<CardInDeck> = serde_json::from_str::<Response>(
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
    .filter(|card| card.quantity.is_some())
    .map(|card| CardInDeck {
        name: match card.name.strip_prefix("A-") {
            Some(x) => x.to_string(),
            None => card.name,
        },
        quantity: card.quantity,
    })
    .collect();

    // println!("{}", deck.id);
    // println!("{:#?}, id: {}", aetherhub_decklist, deck.id);
    sqlx::query_as!(
        AetherHubDeck,
        "INSERT INTO deck (id, deck_id, url, username, date_created, date_updated, commander)
  VALUES (DEFAULT, $1, $2, $3, $4, $5, $6)
  ON CONFLICT (deck_id) DO NOTHING",
        // Uuid::parse_str(&deck.id).expect("uuid parsed wrong"),
        deck.id,
        deck.url,
        deck.username,
        deck.created,
        deck.updated,
        aetherhub_decklist[0].name,
    )
    .execute(pool)
    .await
    .expect("insert deck into db failed");

    let card_ids = aetherhub_decklist.iter().map(|card| async {
        let aftermath_cards = format!(
            "{}%",
            card.name.split_inclusive("/").collect::<Vec<&str>>()[0]
        );
        #[derive(Debug)]
        struct OracleId {
            oracle_id: Option<Uuid>,
            name: Option<String>,
        }

        sqlx::query_as!(
            OracleId,
            "SELECT oracle_id, name FROM card WHERE unaccent(name) LIKE unaccent($1)
            UNION SELECT oracle_id, name FROM card WHERE unaccent(name) LIKE unaccent($2)",
            format!("%{}%", card.name),
            aftermath_cards
        )
        .fetch_optional(pool)
        .await
        .expect(format!("Error when querying db for {}", card.name).as_str())
        .expect(format!("Couldn't find oracle_id of card {}", card.name).as_str())
    });

    let card_ids = join_all(card_ids).await;
    // println!("{:#?}", card_ids);

    #[derive(Debug)]
    struct CombinedCardData {
        oracle_id: Option<Uuid>,
        name: String,
        quantity: Option<i32>,
    }

    let combined_card_data: Vec<CombinedCardData> = aetherhub_decklist
        .into_iter()
        .zip(card_ids)
        .map(|(decklist_card, card_id)| {
            let name = decklist_card.name.clone();
            let quantity = decklist_card.quantity;
            let oracle_id = card_id.oracle_id;

            CombinedCardData {
                oracle_id,
                name,
                quantity,
            }
        })
        .collect();

    struct DeckID {
        id: i32,
    }

    let deck_id: DeckID =
        sqlx::query_as!(DeckID, "SELECT id FROM deck WHERE deck_id = $1", deck.id)
            .fetch_one(pool)
            .await
            .expect(
                format!(
                    "couldn't find primary key of deck with deck_id = {}",
                    deck.id,
                )
                .as_str(),
            );

    for card in combined_card_data {
        // let deck_id = Uuid::parse_str(deck.id.as_str()).expect("uuid parsed wrong");
        println!("{}, {:#?}, {}", deck.id, card.oracle_id, card.name);
        sqlx::query!(
            "INSERT INTO decklist (oracle_id, deck_id, quantity)
            VALUES ($1, $2, $3)
            ON CONFLICT (oracle_id, deck_id) DO UPDATE
            SET quantity = decklist.quantity + $3",
            card.oracle_id,
            deck_id.id,
            card.quantity,
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

    let cards = scryfall_cards
        .into_iter()
        .filter(|card| match card.layout.as_str() {
            "token" => false,
            _ => true,
        })
        .filter(|card| card.legalities.historicbrawl != "not_legal")
        .map(|c| Card::from(c))
        .collect::<Vec<Card>>();

    for card in cards {
        sqlx::query_as!(Card, "
        INSERT INTO card (oracle_id, name, lang, scryfall_uri, layout, mana_cost, cmc, type_line, oracle_text, colors, color_identity, is_legal, is_commander, rarity)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
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
        card.is_commander,
        card.rarity).execute(pool).await.expect("couldn't insert");
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
    // image_uris: Option<CardImages>,
    mana_cost: Option<String>,
    cmc: f32,
    type_line: String,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Vec<String>,
    is_legal: bool,
    is_commander: bool,
    rarity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScryfallCard {
    // id: String,
    oracle_id: String,
    name: String,
    lang: String,
    scryfall_uri: String,
    layout: String,
    image_uris: Option<CardImages>,
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

impl From<ScryfallCard> for Card {
    fn from(c: ScryfallCard) -> Self {
        let is_commander = (c.type_line.to_lowercase().find("legendary").is_some()
            && c.type_line.to_lowercase().find("creature").is_some())
            || c.type_line.to_lowercase().find("planeswalker").is_some();

        Self {
            // id: c.id,
            oracle_id: c.oracle_id,
            name: c.name,
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
            is_legal: match c.legalities.brawl.as_str() {
                "legal" => true,
                _ => false,
            },
            is_commander,
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
