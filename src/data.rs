pub fn find_data() -> std::path::PathBuf {
    use home::home_dir;
    use std::fs::create_dir_all;
    let mut documents = home_dir().expect("unable to find home_dir");
    documents.push("Documents");
    if documents.as_path().exists() {
        documents.push("mtg_gui");
        documents.push(".data");
        create_dir_all(&documents).expect("failed to create mtg_gui in Documents");
        documents
    } else {
        let mut home = home_dir().unwrap();
        home.push("mtg_gui");
        home.push(".data");
        create_dir_all(&home).expect("failed to create mtg_gui in home");
        home
    }
}

pub fn refresh_data(home_dir:std::path::PathBuf){
    std::thread::spawn(||{
    let client = reqwest::blocking::ClientBuilder::new().connect_timeout(std::time::Duration::from_millis(1500)).build().unwrap();
    let meta_json:json::JsonValue =match curl(&client, "https://api.scryfall.com/bulk-data"){
        Some(v) => json::parse(v.as_ref()).expect("scryfall meta_json is invalid json"),
        None => {panic!();}
    };
    let mut data_dir = home_dir;
    data_dir.push(".data");
    data_dir.push("cards.json");
    println!("{}",  data_dir.to_str().unwrap());
    //println!("{}",  meta_json.pretty(8));
    let card_url = meta_json["data"][0]["download_uri"].as_str().expect("scryfall has apparently changed their json format");
    let card_json = curl(&client, card_url).expect("failed to get get card_json");
    std::fs::write(data_dir.to_str().unwrap(), card_json).expect("failed to write data");
    println!("refreshed card_json");
    });



}
fn get_data() {

}

fn curl(client:&reqwest::blocking::Client, url:&str) -> Option<String>{
    for i in 1..=3 {
        println!("attempt {} of refreshing data",  i);
        if let Ok(data) = client.get(url).send() {
            let data = data.text().expect("scryfall returned invalid data, panicing");
            return Some(data);
        }
    }
    None

}
