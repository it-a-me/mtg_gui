use crate::useful_structs::card::{Card, ParseError};
use json::JsonValue;
#[allow(clippy::needless_pass_by_value)]
pub(super) fn parse_cards(cards_data: JsonValue) -> Vec<Card> {
    let mut cards = Vec::new();
    let mut err = Vec::new();
    for (i, card) in cards_data.members().enumerate() {
        match Card::new(card) {
            Ok(v) => cards.push(v),
            Err(e) => err.push((e, i)),
        }
    }
    println!("{}", cards.len());
    println!("{}", err.len());
    println!("finished");
    cards
}
