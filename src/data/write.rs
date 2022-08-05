use crate::useful_structs::card::Card;
use serde::{Deserialize, Serialize};

//#[derive(Debug)]
//struct WriteError {
//    message: String,
//}
//impl WriteError {
//    fn new(message:&str) -> Self{
//        Self{message:message.to_string(),}
//    }
//}
//impl std::error::Error for WriteError {}
//impl std::fmt::Display for WriteError {
//    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
//        write!(f, "")
//    }
//}
//
#[derive(Debug)]
pub enum WriteError {
    Serialize,
    Write
}
impl From<bincode::Error> for WriteError {
    fn from(e:bincode::Error) -> Self {
        WriteError::Serialize
    }
}
impl From<std::io::Error> for WriteError {
    fn from(e:std::io::Error) -> Self {
        WriteError::Write
    }
}

pub fn store(cards: &Vec<Card>, data_dir:&std::path::Path) -> Result<(), WriteError> {
    let bytes = bincode::serialize(cards)?;
    std::fs::write(data_dir.join(".data").join("stored_struct"), bytes)?;
    Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
struct Fool {}
