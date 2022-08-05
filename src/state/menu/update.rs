use super::Menu;
use macroquad::prelude::*;
mod draw;
impl Menu {
    pub async fn update(&mut self) -> Option<u8> {
        if is_key_released(KeyCode::Enter) {
            return self.menu_action();
        }

        if is_key_released(KeyCode::J) || is_key_released(KeyCode::Down) {
            self.position += 1;
            self.position %= self.options.len();
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
