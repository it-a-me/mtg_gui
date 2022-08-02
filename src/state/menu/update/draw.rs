use super::Menu;
use macroquad::prelude::*;
impl Menu {
    pub(super) fn draw(&mut self) {
        for (i, option) in self.options.iter_mut().enumerate() {
            if i == self.position {
                option.draw_alt(Some(BLUE), None);
            } else {
                option.draw();
            }
        }
        //       let fps = Text::new(get_fps().to_string(),
        //       let fps_width = measure_text(&fps_display, None, 30, 1f32).width;
        //       draw_text(
        //           fps_display.as_ref(),
        //           screen_width() - 20f32 - fps_width,
        //           40f32,
        //           30f32,
        //           BLACK,
        //       );
        //   }
    }
}
