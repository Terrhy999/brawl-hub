#[tokio::main]
async fn main() -> () {
    let json_data = get_decklists(0, 40);
    // println!("{}", json_data);

    let client = reqwest::Client::new();
    let res = client
        .post("https://aetherhub.com/Meta/FetchMetaListAdv?formatId=19")
        .header("Content-Type", "application/json")
        .body(json_data)
        .send()
        .await
        .expect("couldn't send Post request");

    println!("Status: {}", res.status());

    let res_text = res.text().await.expect("couldn't ready response body");

    println!("Response Body: \n{}", res_text);
}

fn get_decklists(start: i32, length: i32) -> String {
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

struct Card {
    id: String,
    oracle_id: String,
    name: String,
    lang: String,
    scryfall_uri: String,
    layout: String,
    image_uris: CardImages,
    mana_cost: String,
    cmc: i32,
    type_line: String,
    oracle_text: String,
    colors: String,
    color_identity: String,
    is_legal: bool,
    rarity: String,
}

struct CardImages {
    small: String,
    normal: String,
    large: String,
    png: String,
    art_crop: String,
    border_crop: String,
}

struct ScryfallCard {
    id: String,
    oracle_id: String,
    name: String,
    lang: String,
    scryfall_uri: String,
    layout: String,
    image_uris: CardImages,
    mana_cost: Option<String>,
    cmc: i32,
    type_line: String,
    oracle_text: Option<String>,
    colors: String,
    color_identity: String,
    legalities: Legalaties,
}
