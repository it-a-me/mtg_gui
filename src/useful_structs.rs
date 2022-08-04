mod text;
pub mod card;
#[derive(Clone)]
pub struct Text {
    text: String,
    alignment:Alignment,
    pub x: f32,
    pub y: f32,
    width: Option<f32>,
    height: Option<f32>,
    pub params: macroquad::text::TextParams,
}
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

