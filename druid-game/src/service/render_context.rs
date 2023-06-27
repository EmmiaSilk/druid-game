//! Defines the [`RenderContext`] service and immediately-related subjects. 

use std::fmt::Display;
use std::error::Error;
use vfc::Rgb;
use crate::render::Bitmap;

/// Specification for the `RenderContext` service. 
/// 
/// This service is used for drawing visuals to the screen. 
pub trait RenderContext {
    /// Draw the contents of a bitmap to the screen.
    fn draw(&mut self, bitmap: &Bitmap, x: usize, y: usize) -> Result<(), RenderErr>; 
    /// Fill the render context with the given color.
    fn clear(&mut self, color: &Rgb) -> Result<(), RenderErr>;
}

// TODO: Return more useful error types
/// Any error that occurs during the process of rendering.
#[derive(Debug)]
pub struct RenderErr(pub String);

impl Error for RenderErr {}

impl Display for RenderErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}