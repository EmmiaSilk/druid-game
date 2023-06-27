//! Defines the [`InputManager`] service.

use async_trait::async_trait;

#[async_trait(?Send)]
/// The `InputManager` trait is used for managing input between the user 
/// and the system.  
pub trait InputManager {
    /// Returns true if the user or window is requesting the software close.
    fn is_requesting_close(&self) -> bool;
    /// Causes any future calls to `is_requesting_close` to return true.
    fn request_close(&mut self);
}