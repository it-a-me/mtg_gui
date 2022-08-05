                #![allow(clippy::cast_precision_loss)]
use super::Menu;
use crate::useful_structs::{Alignment, Text};
use macroquad::prelude::*;
impl Menu {
    pub fn new() -> Self {
        let options_strings = ["New Deck", "Load Deck", "Refresh Data", "Quit"];
        let mut  options = Vec::new();
        for (i, option_str) in options_strings.into_iter().enumerate() {
            options.push(Text::new(
                option_str,
                Alignment::Left,
                10.0,
                90.0 + 60.0 * i as f32,
                70,
                BLACK,
            ));
        }
        Self {
            options,
            position: 0,
            last_mouse_pos: Vec2::from(mouse_position()),
        }
    }
}
