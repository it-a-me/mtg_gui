use json::JsonValue;
use macroquad::prelude::*;
use std::path::PathBuf;
mod init;
pub struct Builder {
    deck: Vec<String>,
    mtg_home: PathBuf,
    input:String,
    requested_state: Option<u8>,
}
impl Builder {
    pub async fn update(&mut self) -> Option<u8> {
        use macroquad::ui::Ui;
        clear_background(Color::new(0.3f32,0.4f32,0.5f32, 0f32));
        let mut ui = macroquad::ui::root_ui();
        ui.input_text(2000u64, "gg", &mut self.input);

        self.requested_state
    }
}
