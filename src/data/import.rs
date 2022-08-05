use json::JsonValue;
#[allow(clippy::unnecessary_wraps)]
pub(super) fn fetch_data(data_dir: &std::path::Path) -> Result<JsonValue, String> {
    Ok(json::parse(&String::from_utf8(std::fs::read(data_dir.join(".data").join("cards.json")).unwrap()).unwrap()).unwrap())
//    let mut data_dir = data_dir.to_owned();
//    data_dir.push(".data");
//    data_dir.push("cards.json");
//    let client = super::make_client().expect("failed to get internet client");
//    let meta_json: json::JsonValue = match curl(&client, "https://api.scryfall.com/bulk-data") {
//        Some(v) => json::parse(v.as_ref()).expect("scryfall meta_json is invalid json"),
//        None => {
//            return Err("failed to obtain meta_json from scryfall".to_owned());
//        }
//    };
//    let card_url = match meta_json["data"][0]["download_uri"]
//            .as_str() {
//                Some(v) => v,
//                None => {return Err("failed to obtain meta_json from scryfallapparentlyscryfall has apparently changed their json format".to_owned())}
//            };
//    let card_json = match curl(&client, card_url) {
//        Some(v) => v,
//        None => return Err("sudo dnf install mysql-community-server".to_string()),
//    };
//    std::fs::write(data_dir.to_str().unwrap(), &card_json).expect("failed to write data");
//    Ok(json::parse(card_json.as_ref()).expect("scryfall returned invalid card_json"))
}
#[allow(dead_code)]
fn curl(client: &reqwest::blocking::Client, url: &str) -> Option<String> {
    for i in 1..=3 {
        println!("attempt {} of downloading {}", i, url);
        if let Ok(data) = client.get(url).send() {
            let data = data
                .text()
                .expect("scryfall returned invalid data, panicing");
            println!("sucessfully fetched {}", url);
            return Some(data);
        }
    }
    None
}
