use async_trait::async_trait;
use druid_game::input::InputManager;
pub struct WebInputManager {
    requesting_close: bool,
}

impl WebInputManager {
    pub fn create() -> Self {
        WebInputManager{requesting_close: false}
    }
}

#[async_trait(?Send)]
impl InputManager for WebInputManager {
    fn is_requesting_close(&self) -> bool {
        // TODO: Properly handle
        self.requesting_close
    }

    fn request_close(&mut self) {
        self.requesting_close = true
    }
}