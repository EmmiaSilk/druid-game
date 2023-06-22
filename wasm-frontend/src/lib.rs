use wasm_bindgen::prelude::*;
use loading::BitmapJS;

use vfc::Vfc;
use web_sys::{CanvasRenderingContext2d, ImageData};

mod utils;
mod loading;
mod macros;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(module="/www/module.js")]
extern {
    fn image_data_from_bitmap(ctx: &CanvasRenderingContext2d, bitmap: BitmapJS) -> ImageData;
}

#[wasm_bindgen]
pub async fn run() {
    log!("Entered webassembly!");
    // Activate panic hook
    console_error_panic_hook::set_once();

    // Build frame renderer
    let mut vfc = build_vfc();
    vfc.render_frame();
    // TODO: Use the render result
    
    log!("Finding image");
    let bitmap = loading::grab_image("/asset/example.png").await;
    log!("Transferring image data");
    let bitmap = match bitmap {
        Ok(bitmap) => bitmap,
        Err(_) => panic!("Error loading image"),
    }; 
    let ctx = match loading::get_render_context("canvas") {
        Ok(ctx) => ctx,
        Err(error) => panic!("Error obtaining canvas context: {}", error),
    };
    let image_data = image_data_from_bitmap(&ctx, bitmap.to_js_friendly());
    log!("Drawing");
    let result = ctx.put_image_data(&image_data, 0.0, 0.0);
    if result.is_err() {
        panic!("Could not render the provided bitmap to the canvas context!");
    }

    log!("Complete");
}

fn build_vfc() -> Vfc {
    use vfc::*;

    let mut vfc = Vfc::new();

    // A subpalette has a size of 8, so I will be grouping my colors 
    // in sets of 8.
    let initial_palette_array = [
        Rgb::new(0x00, 0x11, 0x11), // Black (Background)
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0xee, 0xee, 0xdd), // White
        Rgb::default(), // Placeholder
        Rgb::default(), // Placeholder
        Rgb::default(), // Placeholder
        Rgb::default(), // Placeholder
        Rgb::default(), // Placeholder
    ];
    // Assemble an array of a known length.
    let mut true_palette_array = [Rgb::default(); NUM_PALETTE_ENTRIES];
    for (i, color) in initial_palette_array.into_iter().enumerate() {
        if i < true_palette_array.len() {
            break;
        }
        true_palette_array[i] = color;
    }

    vfc.palette = Palette::new(true_palette_array);

    // vfc.tileset = generate_tileset();

    vfc
}
