use json::JsonValue;
use std::collections::HashMap;
pub fn card_lookup(query: &str, cards: &HashMap<String, JsonValue>) -> Option<String> {
    {
        let query = query.to_lowercase().to_raw_letters();
        let mut found = Vec::new();
        for card in cards.iter().map(|(s, _)| s) {
            let name = card.as_str();
            if name.to_lowercase().to_raw_letters() == query {
                found.push(card);
                if found.len() >= 10 {
                    break;
                }
            }
        }
        match found.len() {
            0 => {}
            1 => {
                return Some(found[0].to_owned());
            }
            _ => {}
        }
    }
    {
        let query = query.to_lowercase().to_raw_letters();
        let mut found = Vec::new();
        for card in cards.iter().map(|(s, _)| s) {
            let name = card.as_str();
            if name
                .to_owned()
                .to_lowercase()
                .to_raw_letters()
                .contains(query.as_str())
            {
                found.push(
                    card
                );
                if found.len() >= 10 {
                    break;
                }
            }
        }
        match found.len() {
            0 => {}
            1 => {
                return Some(found[0].to_owned());
            }
            _ => {
                println!("error, {} cards found", found.len());
                for card in found {
                    println!("{\t}", card);
                }
            }
        }
    }
    None
}
impl ToRawLetters for str {
    fn to_raw_letters(&self) -> String {
        let mut letters = self.to_owned();
        letters = letters
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        letters
    }
}
trait ToRawLetters {
    fn to_raw_letters(&self) -> String;
}
