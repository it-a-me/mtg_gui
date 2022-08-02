impl super::Builder {
    pub fn new() -> Self {
        let deck = ["tayam", "black lotus", "cardy"]
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        let mtg_home = find_data();
        println!("{}",  mtg_home.to_string_lossy());
        Self {
            deck,
            mtg_home,
            input:String::new(),
            requested_state: None,
        }
    }

}

fn find_data() -> std::path::PathBuf {
    use home::home_dir;
    use std::fs::create_dir_all;
    let mut documents = home_dir().expect("unable to find home_dir");
    documents.push("Documents");
    if documents.as_path().exists() {
        documents.push("mtg_gui");
        create_dir_all(&documents).expect("failed to create mtg_gui in Documents");
        return documents;
    } else {
        let mut home = home_dir().unwrap();
        home.push("mtg_gui");
        create_dir_all(&home).expect("failed to create mtg_gui in Documents");
        return home;
    }
}
