mod import;
mod parse;
mod write;
use std::path::PathBuf;

pub fn refresh(data_dir: PathBuf) {
    std::thread::spawn(move || {
        let json = match import::fetch_data(&data_dir) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        write::store(&parse::parse_cards(json), &data_dir).unwrap();
        
    });
}
#[allow(dead_code)]
pub fn make_client() -> Option<reqwest::blocking::Client> {
    if let Ok(client) = reqwest::blocking::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_millis(1500))
        .build()
    {
        Some(client)
    } else {
        None
    }
}
pub fn find() -> PathBuf {
    use home::home_dir;
    use std::fs::create_dir_all;
    let mut home = home_dir().expect("unable to find home_dir");
    if home.join("Documents").exists() {
        home.push("Documents");
    }
    home.push("mtg_gui");
    create_dir_all(home.join(".data")).expect("failed to create mtg_gui home_dir");
    create_dir_all(home.join("Decks")).expect("failed to create mtg_gui home_dir");
    home
}
