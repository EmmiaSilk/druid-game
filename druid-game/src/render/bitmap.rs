//! Defines the [`Bitmap`] structure and its functions.
//! 
//! Don't use this module directly. It is better to use its parent module, 
//! [`render`](super).

use vfc::Rgb;

/// A representation of a bitmap image with a defined resolution.
pub struct Bitmap {
    width: usize,
    height: usize,
    colors: Vec<Rgb>,
}

impl Bitmap {
    /// Constructs a bitmap from the given colors and resolution.
    pub fn new(width: usize, height: usize, colors: Vec<Rgb>) -> Self {
        Self { width, height, colors }
    }

    /// Constructs a bitmap from the given framebuffer and resolution.
    pub fn from_framebuffer(width: usize, height: usize, buffer: &[Rgb]) -> Self {
        let colors = buffer.to_vec();
        Self{ width, height, colors }
    }

    /// Returns the width of the bitmap, in pixels
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the bitmap, in pixels
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns an reference to a buffer containing the colors in the bitmap. 
    /// 
    /// The length of the vector is equal to `width`*`height`.
    pub fn colors_ref(&self) -> &Vec<Rgb> {
        &self.colors
    }
}
