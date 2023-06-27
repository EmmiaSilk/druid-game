use std::cell::RefCell;
use std::rc::Rc;

use druid_game::render::Bitmap;
use druid_game::service::render_context::RenderErr;
use druid_game::service::render_context::RenderContext;
use minifb::Window;
use vfc::Rgb;

pub struct MiniFBRenderContext {
    window: Rc<RefCell<Window>>,
}

impl RenderContext for MiniFBRenderContext {
    fn draw(&mut self, bitmap: &Bitmap, _x: usize, _y: usize) -> Result<(), RenderErr> {
        let mut my_vec = Vec::with_capacity(bitmap.height() * bitmap.width());
        for color in bitmap.colors_ref() {
            my_vec.push(color.as_argb_u32());
        }

        let mut window = self.window.borrow_mut();
        let result = window.update_with_buffer(my_vec.as_slice(), bitmap.width(), bitmap.height());
        match result {
            Err(error) => Err(RenderErr(error.to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn clear(&mut self, _color: &Rgb) -> Result<(), RenderErr> {
        todo!() // TODO
    }
}

impl MiniFBRenderContext {
    pub fn create(window: Rc<RefCell<Window>>) -> Self {
        MiniFBRenderContext {
            window,
        }
    }
}