use macroquad::prelude::*;
mod init;
mod update;
pub struct Menu {
    options: Vec<crate::useful_structs::Text>,
    position: usize,
    last_mouse_pos: Vec2,
    requested_state:Option<u8>
}
impl Menu {
    fn menu_action(&mut self) {
        match self.position {
            0 => {self.requested_state = Some(1);}
            1 => {println!("clicked #2");}
            2 => {panic!();}
            _ => {
                panic!()
            }
        }
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
