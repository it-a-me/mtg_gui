#![warn(clippy::all)]
#![warn(clippy::pedantic)]
use macroquad::prelude::*;
mod data;
mod state;
mod useful_structs;

#[macroquad::main("mtg_gui")]
async fn main() {
    let mut state = state::init();
    while let Some(s) = state::update(state).await {
        state = s;
        next_frame().await;
    }
}
