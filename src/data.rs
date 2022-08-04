mod import;
pub mod load;
mod parse;
mod write;

pub fn refresh_data(data_dir: std::path::PathBuf) {
    std::thread::spawn(move || {
        let json = match import::fetch_data(&data_dir) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        parse::parse_cards(json);
    });
}

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
