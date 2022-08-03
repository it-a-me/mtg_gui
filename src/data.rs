pub fn find_data() -> std::path::PathBuf {
    use home::home_dir;
    use std::fs::create_dir_all;
    let mut documents = home_dir().expect("unable to find home_dir");
    documents.push("Documents");
    if documents.as_path().exists() {
        documents.push("mtg_gui");
        create_dir_all(&documents).expect("failed to create mtg_gui in Documents");
        documents
    } else {
        let mut home = home_dir().unwrap();
        home.push("mtg_gui");
        create_dir_all(&home).expect("failed to create mtg_gui in Documents");
        home
    }
}
