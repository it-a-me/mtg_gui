use crate::useful_structs::card::*;
use json::JsonValue;
use std::collections::HashMap;
pub(super) fn parse_cards(cards_data: JsonValue) {
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
    println!("{:?}", err[500]);
    println!("{}", cards_data[err[500].1].pretty(9));
    for er in err {
        match er {
            (CardParseError::NameErr, _) => {}
            _ => {
                println!("{:?}", er);
                println!("{}", cards_data[er.1].pretty(9));
                break;
            }
        }
    }

    println!("finished");
}
