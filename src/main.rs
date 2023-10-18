// use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use std::fs;

fn main() -> () {
    // get_decklists();
    let cards = get_legal_cards("src/oracle-cards-20230919090156.json");
    // println!("{:#?}", cards);
    connect_to_db(cards);
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

    // let insert_cards_query = "
    //   INSERT INTO card (id, oracle_id, name, lang, scryfall_uri, layout, mana_cost, cmc, type_line, oracle_text, colors, color_identity, is_legal, is_commander, rarity)
    //   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)";

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
async fn get_decklists() {
    let json_data = get_decklists_body(0, 40);
    // println!("{}", json_data);

    let req_client = reqwest::Client::new();
    let res = req_client
        .post("https://aetherhub.com/Meta/FetchMetaListAdv?formatId=19")
        .header("Content-Type", "application/json")
        .body(json_data)
        .send()
        .await
        .expect("couldn't send Post request");

    println!("Status: {}", res.status());

    let res_text = res.text().await.expect("couldn't ready response body");

    println!("Response Body: \n{:?}", res_text);
}

fn get_decklists_body(start: i32, length: i32) -> String {
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

    request_data
}

// fn connect_to_db() -> Result<(), postgres::Error> {
//     let post_client = Client::connect("host=localhost user=postgres", NoTls)?;
//     Ok(())
// }

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

    let cards: Vec<Card> = filtered_cards
        .into_iter()
        .map(|c| {
            let card = Card::from(c);
            card
        })
        .collect();
    cards
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
