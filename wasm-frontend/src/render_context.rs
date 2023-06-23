use std::fmt::Display;

use druid_game::render::{RenderContext, RenderErr, Bitmap};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::log;

#[wasm_bindgen(module="/www/module.js")]
extern {
    fn image_data_from_bitmap(ctx: &CanvasRenderingContext2d, bitmap: BitmapJS) -> ImageData;
}

pub struct WebRenderContext {
    canvas_ctx: CanvasRenderingContext2d,
}

impl WebRenderContext {
    pub fn with_id(canvas_id: &str) -> Result<Self, CanvasError> {
        let document = web_sys::window().unwrap().document().unwrap();
        // Find canvas
        let canvas = match document.get_element_by_id(canvas_id) {
            None => return Err(CanvasError::NoCanvas),
            Some(canvas) => canvas,
        };
        let canvas = match canvas.dyn_into::<HtmlCanvasElement>() {
            Err(_) => return Err(CanvasError::WrongElementType),
            Ok(canvas) => canvas,
        };
    
        // Request context
        let ctx = match canvas.get_context("2d") {
            Err(_) => return Err(CanvasError::UndocumentedContextError0),
            Ok(None) => return Err(CanvasError::UndocumentedContextError1),
            Ok(Some(ctx)) => ctx,
        };
        let ctx = match ctx.dyn_into::<CanvasRenderingContext2d>() {
            Err(_) => return Err(CanvasError::WrongContextType),
            Ok(ctx) => ctx,
        };

        // Construct
        let web_render_context = WebRenderContext {
            canvas_ctx: ctx,
        };

        Ok(web_render_context)
    }    
}

impl RenderContext for WebRenderContext {
    fn draw(&mut self, bitmap: &Bitmap, x: usize, y: usize) -> Result<(), RenderErr>{
        let bitmap_js = BitmapJS::from_bitmap(bitmap);
        let image_data = image_data_from_bitmap(&self.canvas_ctx, bitmap_js);
        let result = self.canvas_ctx.put_image_data(&image_data, x as f64, y as f64);
        if result.is_err() {
            // TODO: This error really needs to be friendly with the main API
            return Err(RenderErr("Could not render the provided bitmap to the canvas context!".to_string()));
        }
        
        Ok(())
    }

    fn clear(&mut self, _color: &vfc::Rgb) -> Result<(), RenderErr> {
        // TODO
        Ok(())
    }
}

#[derive(Debug)]
pub enum CanvasError {
    /// Occurs when a draw call fails.
    DrawError,
    /// Occurs when a `HtmlCanvasElement` is requested but none is found at the given `element_id`
    NoCanvas,
    /// Occurs when a `HtmlCanvasElement` is requested but another element is found instead.
    WrongElementType,
    /// Occurs when `get_context(..)` is passed the wrong `context_id`
    WrongContextType,
    /// One of these errors appears when `context_id` is invalid, but I don't know which.
    UndocumentedContextError0,
    /// One of these errors appears when `context_id` is invalid, but I don't know which.
    UndocumentedContextError1,
}

impl Display for CanvasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[wasm_bindgen]
pub struct BitmapJS {
    pub width: u32,
    pub height: u32,
    colors: Vec<u32>,
}

#[wasm_bindgen]
impl BitmapJS {
    #[wasm_bindgen(getter)]
    pub fn colors(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(&self.colors[..])
    }
}

impl BitmapJS {
    pub fn from_bitmap(source: &Bitmap) -> BitmapJS {
        let source_colors = source.colors_ref();
        let mut colors = Vec::with_capacity(source_colors.len());
        for color in source_colors {
            colors.push(color.as_argb_u32());
        }

        BitmapJS { 
            width: source.width() as u32, 
            height: source.height() as u32, 
            colors,
        }
    }
}