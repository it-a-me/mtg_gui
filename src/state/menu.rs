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
            2 => crate::data::refresh(crate::data::find()),
            3 => return Some(0),
            _ => panic!(),
        }
        None
    }
}
