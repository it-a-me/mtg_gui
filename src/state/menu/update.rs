use super::Menu;
use crate::useful_structs::*;
use macroquad::prelude::*;
mod draw;
impl Menu {
    pub async fn update(&mut self) -> Option<u8> {
        if is_key_released(KeyCode::Enter) {
            return self.menu_action();
        }

        if is_key_released(KeyCode::J) || is_key_released(KeyCode::Down) {
            self.position += 1;
            self.position = self.position % self.options.len();
        } else if macroquad::input::is_key_released(KeyCode::K) || is_key_released(KeyCode::Up) {
            match self.position {
                0 => self.position = self.options.len() - 1,
                _ => self.position -= 1,
            }
        }

        self.draw();
        let mouse_pos = Vec2::from(mouse_position());
        if mouse_pos != self.last_mouse_pos {
            for (i, option) in self.options.iter_mut().enumerate() {
                if option.contains_point(mouse_pos) {
                    self.position = i;
                    break;
                }
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) && self.options[self.position].contains_point(mouse_pos) {
            return self.menu_action();
        }
        self.last_mouse_pos = mouse_pos;
        None
    }
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn is_inside(&self, coords: Vec2) -> bool {
        let r = coords.x > self.x
            && coords.x < self.x + self.width
            && coords.y > self.y
            && coords.y < self.y + self.height;
        if r {
            println!(
                "{}, {}",
                r,
                std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos()
            );
        }
        r
    }
    fn draw(&self, color: Color) {
        draw_rectangle(self.x, self.y, self.width, self.height, color);
    }
}
