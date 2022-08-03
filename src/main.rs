use macroquad::prelude::*;
mod state;
mod useful_structs;
mod data;

#[macroquad::main("mtg_gui")]
async fn main() {
    let mut state = state::init();
    loop {
        clear_background(PINK);
        state = state::update(state, get_frame_time()).await;
        next_frame().await
    }
}
