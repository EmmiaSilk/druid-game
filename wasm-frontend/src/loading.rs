use std::{cell::Cell, rc::Rc, future::Future, task::Poll};

use async_trait::async_trait;
use druid_game::{render::Bitmap, io::{AssetLoader, LoadError}};
use wasm_bindgen::prelude::*;
use web_sys::{ImageData, HtmlImageElement, OffscreenCanvas, OffscreenCanvasRenderingContext2d};
use vfc::Rgb;

use crate::render_context::CanvasError;

#[wasm_bindgen(module="/www/module.js")]
extern {
    fn load_image(path: &str) -> ImageData;
}

pub struct WebAssetLoader {}

impl WebAssetLoader {
    pub fn new() -> WebAssetLoader {
        WebAssetLoader {}
    }
}

#[async_trait(?Send)]
impl AssetLoader for WebAssetLoader {
    async fn load_bitmap(&mut self, path: &str) -> Result<Bitmap, LoadError> {   
        // Start loading the image.
        let image = ImageFuture::new(path).await;
        let image = match image {
            Err(()) => return Err(LoadError::ResourceNotFound(path.to_string())),
            Ok(image) => image,
        };
    
        // Generate Canvas
        let canvas = OffscreenCanvas::new(image.width(), image.height());
        let canvas = match canvas {
            Err(_) => return Err(LoadError::OtherError(CanvasError::NoCanvas.to_string())),
            Ok(canvas) => canvas,
        };
    
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap();
        let ctx = match ctx.dyn_into::<OffscreenCanvasRenderingContext2d>() {
            Err(_) => return Err(LoadError::OtherError(CanvasError::WrongContextType.to_string())),
            Ok(ctx) => ctx,
        };
    
        // Draw to canvas
        let result = ctx.draw_image_with_html_image_element(&image, 0.0, 0.0);
        if result.is_err() {
            return Err(LoadError::OtherError(CanvasError::DrawError.to_string()))
        }
    
        // Extract image data
        let data = ctx
            .get_image_data(0.0, 0.0, canvas.width().into(), canvas.height().into())
            .unwrap(); // TODO: Properly handle none result
    
        let bitmap = bitmap_from_image_data(&data);
    
        Ok(bitmap)
    }
}


pub fn bitmap_from_image_data(image_data: &ImageData) -> Bitmap {
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

    Bitmap::new(width, height, colors)
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