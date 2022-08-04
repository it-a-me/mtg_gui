use json::JsonValue;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Card {
    name: String,
    id: String,
    image_uris: ImageUris,
    mana_cost: String,
    cmc: u8,
    types: Vec<String>,
    oracle_text: String,
    formats: Formats,
    usd: Option<f32>,
}
#[derive(Debug)]
pub enum CardParseError {
    NameErr,
    IdErr,
    ManaCostErr,
    CmcErr,
    TypeErr,
    OracleTextErr,
    LegalityErr,
    PriceErr,
}
impl Card {
    pub fn new(json: &JsonValue) -> Result<Self, CardParseError> {
        let name: Result<&str, CardParseError> =
            json["name"].as_str().ok_or(CardParseError::NameErr);
        let id = json["id"].as_str().ok_or(CardParseError::IdErr);
        let image_uri = ImageUris::new(&json["image_uris"]).ok_or(CardParseError::NameErr);
        let mana_cost = json["mana_cost"]
            .as_str()
            .ok_or(CardParseError::ManaCostErr);
        let cmc = json["cmc"].as_f32().unwrap() as u8;
        let types = json["type_line"].as_str().ok_or(CardParseError::TypeErr);
        let oracle_text = json["oracle_text"]
            .as_str()
            .ok_or(CardParseError::OracleTextErr);
        let formats = Formats::new(&json["legalities"]).ok_or(CardParseError::LegalityErr);
        let usd = match json["prices"]["usd"].as_str() {
            Some(v) => Some(v.parse::<f32>().expect("usd is invalid for a float")),
            None => None,
        };
        Ok(Self {
            name: name?.to_owned(),
            id: id?.to_owned(),
            image_uris: image_uri?,
            mana_cost: mana_cost?.to_owned(),
            cmc,
            types: types?
                .split_whitespace()
                .filter(|t| *t != "—")
                .map(|t| t.to_owned())
                .collect::<Vec<String>>(),
            oracle_text: oracle_text?.to_owned(),
            formats: formats?,
            usd,
        })
    }
    fn generate_types(json: &JsonValue) -> Vec<String> {
        let mut types = Vec::new();
        for t in json.members() {
            if let Some(s) = t.as_str() {
                types.push(s.to_owned());
            }
        }
        types
    }
}
#[derive(Debug)]
pub struct ImageUris {
    small: String,
    normal: String,
    large: String,
    png: String,
    art_crop: String,
    border_crop: String,
}
impl ImageUris {
    pub fn new(json: &JsonValue) -> Option<Self> {
        Some(Self {
            small: json["small"].as_str()?.to_owned(),
            normal: json["normal"].as_str()?.to_owned(),
            large: json["large"].as_str()?.to_owned(),
            png: json["png"].as_str()?.to_owned(),
            art_crop: json["art_crop"].as_str()?.to_owned(),
            border_crop: json["border_crop"].as_str()?.to_owned(),
        })
    }
}
#[derive(Debug)]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}
#[derive(Debug)]
pub struct Formats {
    standard: Legality,
    future: Legality,
    historic: Legality,
    gladiator: Legality,
    pioneer: Legality,
    explorer: Legality,
    modern: Legality,
    legacy: Legality,
    pauper: Legality,
    vintage: Legality,
    penny: Legality,
    commander: Legality,
    brawl: Legality,
    historicbrawl: Legality,
    alchemy: Legality,
    paupercommander: Legality,
    duel: Legality,
    oldschool: Legality,
    premodern: Legality,
}
impl Formats {
    pub fn new(json: &JsonValue) -> Option<Self> {
        Some(Self {
            standard: json["standard"].is_legal()?,
            future: json["future"].is_legal()?,
            historic: json["historic"].is_legal()?,
            gladiator: json["gladiator"].is_legal()?,
            pioneer: json["pioneer"].is_legal()?,
            explorer: json["explorer"].is_legal()?,
            modern: json["modern"].is_legal()?,
            legacy: json["legacy"].is_legal()?,
            pauper: json["pauper"].is_legal()?,
            vintage: json["vintage"].is_legal()?,
            penny: json["penny"].is_legal()?,
            commander: json["commander"].is_legal()?,
            brawl: json["brawl"].is_legal()?,
            historicbrawl: json["historicbrawl"].is_legal()?,
            alchemy: json["alchemy"].is_legal()?,
            paupercommander: json["paupercommander"].is_legal()?,
            duel: json["duel"].is_legal()?,
            oldschool: json["oldschool"].is_legal()?,
            premodern: json["premodern"].is_legal()?,
        })
    }
}

trait IsLegal {
    fn is_legal(&self) -> Option<Legality>;
}
impl IsLegal for JsonValue {
    fn is_legal(&self) -> Option<Legality> {
        match self.as_str()? {
            "legal" => Some(Legality::Legal),
            "not_legal" => Some(Legality::NotLegal),
            "restricted" => Some(Legality::Restricted),
            "banned" => Some(Legality::Banned),
            _ => None,
        }
    }
}
