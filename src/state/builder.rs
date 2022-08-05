use macroquad::prelude::*;
use std::path::PathBuf;
mod init;
#[allow(dead_code)]
pub struct Builder {
    deck: Vec<String>,
    mtg_home: PathBuf,
    input:String,
}
impl Builder {
    pub async fn update(&mut self) -> Option<u8> {
        clear_background(Color::new(0.3f32,0.4f32,0.5f32, 0f32));
        let mut ui = macroquad::ui::root_ui();
        ui.input_text(2000u64, "gg", &mut self.input);

        None
    }
}
