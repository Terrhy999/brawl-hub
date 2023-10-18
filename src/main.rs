// use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::postgres::PgPoolOptions;
// use sqlx::types::Uuid;
use std::fs;
use uuid::Uuid;

fn main() -> () {
    let decklists = get_decklists(0,40);
    // println!("{:#?}", decklists);
    // let cards = get_legal_cards("src/oracle-cards-20230919090156.json");
    // println!("{:#?}", cards);
    // connect_to_db(cards);
    let decks = get_decks(decklists);
    add_decks(decks);
}

#[tokio::main]
async fn connect_to_db(cards: Vec<Card>) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/brawlhub")
        .await
        .expect("couldn't connect to db");

    let create_query = "
      CREATE TABLE IF NOT EXISTS card (
        oracle_id uuid NOT NULL PRIMARY KEY,
        name text NOT NULL,
        lang text NOT NULL,
        scryfall_uri text NOT NULL,
        layout text NOT NULL,
        mana_cost text,
        cmc real NOT NULL,
        type_line text NOT NULL,
        oracle_text text,
        colors char(1)[],
        color_identity char(1)[] NOT NULL,
        is_legal bool NOT NULL,
        is_commander bool NOT NULL,
        rarity text
      )";

    sqlx::query(create_query)
        .execute(&pool)
        .await
        .expect("couldn't create card table");

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
        card.rarity).execute(&pool).await.expect("couldn't insert");
    }
}

#[tokio::main]
async fn add_decks(decks: Vec<Deck>) {
  let pool = PgPoolOptions::new()
  .max_connections(5)
  .connect("postgres://postgres:postgres@localhost/brawlhub")
  .await
  .expect("couldn't connect to db");

  let create_query = "
    CREATE TABLE IF NOT EXISTS deck (
      id uuid NOT NULL PRIMARY KEY,
      deck_id int NOT NULL,
      url text NOT NULL,
      username text NOT NULL,
      date_created bigint NOT NULL,
      date_updated bigint NOT NULL
    )";

  sqlx::query(create_query)
    .execute(&pool)
    .await
    .expect("couldn't create deck table");

  for deck in decks {
    sqlx::query_as!(Deck, "
    INSERT INTO deck (id, deck_id, url, username, date_created, date_updated)
    VALUES ($1, $2, $3, $4, $5, $6)",
    Uuid::parse_str(&deck.id),
    deck.deck_id,
    deck.url,
    deck.username,
    deck.date_created,
    deck.date_updated).execute(&pool).await.expect("couldn't insert to deck table");
  }
}

#[tokio::main]
async fn get_decklists(start: i32, length: i32) -> String {

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
    "#);

    let start = format!(",\n\"start\": {},\n", start);
    let length = format!("\"length\": {}\n}}", length);
    request_data.push_str(&start);
    request_data.push_str(&length);

    let req_client = reqwest::Client::new();
    let res = req_client
        .post("https://aetherhub.com/Meta/FetchMetaListAdv?formatId=19")
        .header("Content-Type", "application/json")
        .body(request_data)
        .send()
        .await
        .expect("couldn't send Post request");

    res.text().await.expect("couldn't ready response body")

}

fn get_legal_cards(path: &str) -> Vec<Card> {
    let data = fs::read_to_string(path).expect("unable to read JSON");
    let scryfall_cards: Vec<ScryfallCard> =
        serde_json::from_str(&data).expect("unable to parse JSON");

    let filtered_cards: Vec<ScryfallCard> = scryfall_cards
        .into_iter()
        .filter(|card| match card.layout.as_str() {
            "planar" => false,
            "scheme" => false,
            "vanguard" => false,
            "token" => false,
            "double_faced_token" => false,
            "emblem" => false,
            "augment" => false,
            "host" => false,
            "art_series" => false,
            "reversible_card" => false,
            _ => true,
        })
        .collect();

    filtered_cards
        .into_iter()
        .map(|c| {
            let card = Card::from(c);
            card
        })
        .collect()

}

fn get_decks(json_data: String) -> Vec<Deck> {
  let ah_decks: AH_Decks = serde_json::from_str(&json_data).expect("unaple to parse JSON");
  let ah_decks_vec = ah_decks.metadecks;

  ah_decks_vec.into_iter().map(|d| {let deck = Deck::from(d); deck}).collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct Deck {
  id: String,
  ah_deck_id: i32,
  url: String,
  username: String,
  date_created: i64,
  date_updated: i64,
}

impl From<AH_Deck> for Deck {
  fn from(d: AH_Deck) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      ah_deck_id: d.id,
      url: d.url,
      username: d.username,
      date_created: d.created,
      date_updated: d.updated
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct AH_Decks {
  metadecks: Vec<AH_Deck>
}

#[derive(Serialize, Deserialize, Debug)]
struct AH_Deck {
  id: i32,
  name: String,
  url: String,
  username: String,
  updated: i64,
  created: i64
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
}

impl From<ScryfallCard> for Card {
    fn from(c: ScryfallCard) -> Self {
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
            is_commander: is_commander(c.type_line),
        }
    }
}

fn is_commander(type_line: String) -> bool {
    if (type_line.to_lowercase().find("legendary").is_some()
        && type_line.to_lowercase().find("creature").is_some())
        || type_line.to_lowercase().find("planeswalker").is_some()
    {
        true
    } else {
        false
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
