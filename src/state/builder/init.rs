impl super::Builder {
    pub fn new() -> Self {
        let deck = ["tayam", "black lotus", "cardy"]
            .into_iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        let mtg_home = crate::data::find();
        println!("{}",  mtg_home.to_string_lossy());
        Self {
            deck,
            mtg_home,
            input:String::new(),
        }
    }

}

