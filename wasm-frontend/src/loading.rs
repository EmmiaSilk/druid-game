use core::panic;
use std::{cell::Cell, rc::Rc, future::Future, task::Poll, fmt::Display};

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, HtmlImageElement, HtmlCanvasElement, OffscreenCanvas, OffscreenCanvasRenderingContext2d};
use vfc::Rgb;

#[wasm_bindgen(module="/www/module.js")]
extern {
    fn load_image(path: &str) -> ImageData;
}

pub struct Bitmap {
    width: usize,
    height: usize,
    colors: Vec<Rgb>,
}

impl Bitmap {
    pub fn build_from_image_data(image_data: &ImageData) -> Self {
        let width = image_data.width() as usize;
        let height = image_data.height() as usize;
        
        let bytes = image_data.data();
        let mut colors = Vec::with_capacity(bytes.len()/4);

        let mut i = 0;
        while i+3 < bytes.len() {
            let a = bytes[i];
            let r = bytes[i+1];
            let g = bytes[i+2];
            let b = bytes[i+3];

            let val = u32::from_be_bytes([a, r, g, b]);
            let color = Rgb::from_argb_u32(&val);
            colors.push(color);
            i += 4;
        };

        Bitmap {
            width, height, colors,
        }
    }
    
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn colors_ref(&self) -> &Vec<Rgb> {
        &self.colors
    }
}

impl Bitmap {
    pub fn to_js_friendly(&self) -> BitmapJS {
        let mut colors = Vec::with_capacity(self.colors.len());
        for color in self.colors_ref() {
            colors.push(color.as_argb_u32());
        }

        BitmapJS { 
            width: self.width() as u32, 
            height: self.height() as u32, 
            colors,
        }
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

#[derive(Debug)]
pub enum LoadError {
    ImageNotFound,
    CanvasError(CanvasError),
}
impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

pub fn get_render_context(id: &str) -> Result<CanvasRenderingContext2d, CanvasError> {
    let document = web_sys::window().unwrap().document().unwrap();
    // Find canvas
    let canvas = match document.get_element_by_id(id) {
        None => return Err(CanvasError::NoCanvas),
        Some(canvas) => canvas,
    };
    let canvas = match canvas.dyn_into::<HtmlCanvasElement>() {
        Err(_) => return Err(CanvasError::WrongElementType),
        Ok(canvas) => canvas,
    };

    // Request context
    let context = match canvas.get_context("2d") {
        Err(_) => return Err(CanvasError::UndocumentedContextError0),
        Ok(None) => return Err(CanvasError::UndocumentedContextError1),
        Ok(Some(context)) => context,
    };
    match context.dyn_into::<CanvasRenderingContext2d>() {
        Err(_) => Err(CanvasError::WrongContextType),
        Ok(ctx) => Ok(ctx),
    }
}

pub async fn grab_image(path: &str) -> Result<Bitmap, LoadError> {
    // Start loading the image.
    let image = ImageFuture::new(path).await;
    let image = match image {
        Err(()) => return Err(LoadError::ImageNotFound),
        Ok(image) => image,
    };

    // Generate Canvas
    let canvas = OffscreenCanvas::new(image.width(), image.height());
    let canvas = match canvas {
        Err(_) => return Err(LoadError::CanvasError(CanvasError::NoCanvas)),
        Ok(canvas) => canvas,
    };

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap();
    let ctx = match ctx.dyn_into::<OffscreenCanvasRenderingContext2d>() {
        Err(_) => return Err(LoadError::CanvasError(CanvasError::WrongContextType)),
        Ok(ctx) => ctx,
    };

    // Draw to canvas
    let result = ctx.draw_image_with_html_image_element(&image, 0.0, 0.0);
    if result.is_err() {
        return Err(LoadError::CanvasError(CanvasError::DrawError))
    }

    // Extract image data
    let data = ctx
        .get_image_data(0.0, 0.0, canvas.width().into(), canvas.height().into())
        .unwrap(); // TODO: Properly handle none result

    let bitmap = Bitmap::build_from_image_data(&data);

    Ok(bitmap)
}

pub struct ImageFuture {
    image: Option<HtmlImageElement>,
    load_failed: Rc<Cell<bool>>,
}

impl ImageFuture {
    fn new(path: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(path);
        ImageFuture {
            image: Some(image),
            load_failed: Rc::new(Cell::new(false)),
        }
    }
}

impl Future for ImageFuture {
    type Output = Result<HtmlImageElement, ()>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match &self.image {
            Some(image) if image.complete() => {
                let image = self.image.take().unwrap();
                let failed = self.load_failed.get();

                 if failed {
                    Poll::Ready(Err(()))
                 } else {
                    Poll::Ready(Ok(image))
                 }
            },
            Some(image) => {
                let waker = cx.waker().clone();
                let on_load_closure = Closure::wrap(Box::new(move || {
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
                on_load_closure.forget();

                let waker = cx.waker().clone();
                let failed_flag = self.load_failed.clone();
                let on_error_closure = Closure::wrap(Box::new(move || {
                    failed_flag.set(true);
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
                on_error_closure.forget();

                Poll::Pending
            },
            None => Poll::Ready(Err(())),
        }
    }
}