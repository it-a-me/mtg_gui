use std::path::PathBuf;
pub fn find_data() -> std::path::PathBuf {
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
