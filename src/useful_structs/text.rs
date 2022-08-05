use super::Alignment;
use macroquad::prelude::*;
use macroquad::text::TextParams;
#[allow(dead_code)]
impl super::Text {
    pub fn new(
        text: &str,
        alignment: Alignment,
        x: f32,
        y: f32,
        font_size: u16,
        color: Color,
    ) -> Self {
        Self {
            text: text.to_string(),
            alignment,
            x,
            y,
            width: None,
            height: None,
            params: TextParams {
                font_size,
                color,
                ..Default::default()
            },
        }
    }
    pub fn change_text(&mut self, text: &str) {
        self.text = text.to_string();
        (self.width, self.height) = (None, None);
    }
    pub fn draw_alt(&mut self, alt_color: Option<Color>, alt_size: Option<u16>) {
        let mut alt_params = self.params;
        if let Some(alt_color) = alt_color {
            alt_params.color = alt_color;
        }
        if let Some(alt_size) = alt_size {
            alt_params.font_size = alt_size;
        }
        Self {
            params: alt_params,
            ..self.clone()
        }
        .draw();
    }
    pub fn draw(&mut self) {
        match self.alignment {
            Alignment::Left => {
                draw_text_ex(&self.text, self.x, self.y, self.params);
            }
            Alignment::Right => {
                let width = self.get_width();
                draw_text_ex(&self.text, self.x - width, self.y, self.params);
            }
            Alignment::Center => {
                let width = self.get_width();
                draw_text_ex(&self.text, self.x - width / 2f32, self.y, self.params);
            }
        }
    }
    pub fn get_width(&mut self) -> f32 {
        if self.width.is_none() {
            self.measure();
        }
        self.width.unwrap()
    }
    pub fn get_height(&mut self) -> f32 {
        if self.height.is_none() {
            self.measure();
        }
        self.height.unwrap()
    }
    fn measure(&mut self) {
        let dim = measure_text(
            &self.text,
            Some(self.params.font),
            self.params.font_size,
            self.params.font_scale,
        );
        self.width = Some(dim.width);
        self.height = Some(dim.height);
    }
    pub fn contains_point(&mut self, point: macroquad::math::Vec2) -> bool {
        point.x > self.x
            && point.x < self.x + self.get_width()
            && point.y > self.y - self.get_height()
            && point.y < self.y
    }
}
