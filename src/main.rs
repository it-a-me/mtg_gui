use macroquad::prelude::*;
mod data;
mod state;
mod useful_structs;

#[macroquad::main("mtg_gui")]
async fn main() {
    let mut state = state::init();
    loop {
        if let Some(s) = state::update(state).await {
            state = s
        } else {
            break;
        }
        next_frame().await
    }
}
