use async_trait::async_trait;

#[async_trait(?Send)]
pub trait InputManager {
    fn is_requesting_close(&self) -> bool;
    fn request_close(&mut self);
}