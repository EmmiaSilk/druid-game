use std::{cell::RefCell, rc::Rc};

use async_trait::async_trait;
use druid_game::service::input::InputManager;
use minifb::Window;
use minifb::Key;

pub struct WindowInputManager {
    window: Rc<RefCell<Window>>,
    requesting_close: bool,
}

impl WindowInputManager {
    pub fn create(window: Rc<RefCell<Window>>) -> WindowInputManager {
        WindowInputManager{window, requesting_close: false}
    }
}

#[async_trait(?Send)]
impl InputManager for WindowInputManager {
    fn is_requesting_close(&self) -> bool {
        let window = self.window.borrow_mut();
        self.requesting_close || !(window.is_open() && !window.is_key_down(Key::Escape))
    }

    fn request_close(&mut self) {
        self.requesting_close = true
    }
}
