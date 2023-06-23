use std::{rc::Rc, cell::RefCell};

use druid_game::render::{RenderContext, RenderErr};
use minifb::Window;
use vfc::Rgb;

pub struct MiniFBRenderContext {
    window: Rc<RefCell<Window>>,
}

impl RenderContext for MiniFBRenderContext {
    fn draw(&mut self, bitmap: &druid_game::render::Bitmap, _x: usize, _y: usize) -> Result<(), druid_game::render::RenderErr> {
        let mut my_vec = Vec::with_capacity(bitmap.height() * bitmap.width());
        for color in bitmap.colors_ref() {
            let [a, r, g, b] = color.as_argb_u32().to_be_bytes();
            let rgba = u32::from_be_bytes([r, g, b, a]);
            my_vec.push(rgba);
        }

        let mut window = self.window.borrow_mut();
        let result = window.update_with_buffer(my_vec.as_slice(), bitmap.width(), bitmap.height());
        match result {
            Err(error) => Err(RenderErr(error.to_string())),
            Ok(()) => Ok(()),
        }
    }

    fn clear(&mut self, _color: &Rgb) -> Result<(), druid_game::render::RenderErr> {
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