use macroquad::prelude::*;
mod init;
mod update;
pub struct Menu {
    options: Vec<crate::useful_structs::Text>,
    position: usize,
    last_mouse_pos: Vec2,
}
impl Menu {
    fn menu_action(&mut self) -> Option<u8> {
        match self.position {
            0 => return Some(2),
            1 => println!("clicked #2"),
            2 => crate::data::refresh_data(crate::data::load::find_data()),
            3 => return Some(0),
            _ => panic!(),
        }
        None
    }
}
fn is_inside(x1: isize, y1: isize, x2: isize, y2: isize, width: isize, height: isize) -> bool {
    if x1 > x2 && x1 < x2 + width {
        if y1 > y2 && y1 < y2 + height {
            //draw_rectangle(x2 as f32, y2 as f32, width as f32, height as f32, WHITE);
            return true;
        }
    }
    false
}

struct Rect {
    pub x: isize,
    pub y: isize,
    pub width: isize,
    pub height: isize,
}
