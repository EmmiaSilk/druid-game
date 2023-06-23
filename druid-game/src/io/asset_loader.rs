//! Defines the [`AssetLoader`] service and imediately-related subjects.
//! 
//! Don't use this module directly. It is better to use its parent module, 
//! [`render`](super).

use std::{fmt::Display, error::Error};

use async_trait::async_trait;

use crate::render::Bitmap;

/// This service is used for loading assets, such as bitmap images.
#[async_trait(?Send)]
pub trait AssetLoader {
    // TODO: Return more useful error types
    /// Load a bitmap from the specified path
    async fn load_bitmap(&mut self, path: &str) -> Result<Bitmap, LoadError>;
}

/// A collection of possible errors that may occur during asset-loading.
#[derive(Debug)]
pub enum LoadError {
    /// Occurs when the requested resource cannot be found at the specified path.   
    ResourceNotFound(String),
    /// Occurs when the error is specific to the implementation.
    OtherError(String),
}
impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for LoadError {}