use crate::data::load::find_data;
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
        }
    }

}

