#[tokio::main]
async fn main() -> () {
    let json_data = r#"{
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
        "start": 0,
        "length": 40,
        "search": {
            "value": "",
            "regex": false
        }
    }"#;

    let client = reqwest::Client::new();
    let res = client
        .post("https://aetherhub.com/Meta/FetchMetaListAdv?formatId=19")
        .header("Content-Type", "application/json")
        .body(json_data)
        .send()
        .await
        .expect("couldn't send Post request");

    println!("Status: {}", res.status());

    let res_body = res.text().await.expect("couldn't ready response body");

    println!("Response Body: \n{}", res_body);
}
