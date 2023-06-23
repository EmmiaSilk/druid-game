use std::fmt::Display;
use std::error::Error;
use std::collections::HashMap;

use game_loop::Time;
use game_loop::GameLoop;
use vfc::{SCREEN_HEIGHT, SCREEN_WIDTH};
use vfc::Vfc;

use crate::input::InputManager;
use crate::io::AssetLoader;
use crate::render::Bitmap;
use crate::render::RenderContext;



/// A selecton of services used to run the game, particularly in the 
/// [`run`] function. 
/// 
/// Each service should be implemented by whichever front-end uses the 
/// library.
pub struct ServiceContainer {
    /// Render context
    pub render_context: Box<dyn RenderContext>,
    /// Asset loader
    pub asset_loader: Box<dyn AssetLoader>,
    /// Input manager
    pub input_manager: Box<dyn InputManager>,
    /// Vfc
    pub vfc: Box<Vfc>,
}


/// The starting point for the game.
pub async fn run(mut services: ServiceContainer) -> Result<(), Box<dyn Error>> {
    println!("Running!!!");
    // Load
    let result = services.asset_loader.load_bitmap("asset/example.png").await;
    let bitmap = match result {
        Ok(bitmap) => bitmap,
        Err(_) => return Err(Box::new(AppError("Problem loading bitmap".into()))), // TODO: Provide a clearer error
    };
    // Draw to the canvas
    let result = services.render_context.draw(&bitmap, 0, 0);
    if let Err(error) = result {
        return Err(Box::new(AppError(error.to_string())));
    }

    // TODO: Game loop
    // let mut updateClosure = |g: &mut GameLoop<&mut ServiceContainer, Time, ()>| {
    //     // loop_fn(services);
    // };
    // let mut render_closure = |g: &mut GameLoop<&mut ServiceContainer, Time, ()>| {
    //     loop_fn(g.game);
    // };
    game_loop::game_loop(services, 60, 0.1, 
            |g: &mut GameLoop<ServiceContainer, Time, ()>| {
                // loop_fn(services);
                if g.game.input_manager.is_requesting_close() {
                    g.exit();
                }
            }, 
            |g: &mut GameLoop<ServiceContainer, Time, ()>| {
                loop_fn(&mut g.game);
            }
        );
    Ok(())
}

pub fn loop_fn(services: &mut ServiceContainer) -> Result<(), Box<AppError>>{
    // Render to framebuffer
    services.vfc.render_frame();
    let fb = &services.vfc.framebuffer;
    // Draw to screen
    let bitmap = Bitmap::from_framebuffer(SCREEN_WIDTH, SCREEN_HEIGHT, fb);
    let result = services.render_context.draw(&bitmap, 0, 0);
    if let Err(error) = result {
        return Err(Box::new(AppError(error.to_string())));
    }

    Ok(())
}

/// Contains information about an error that prevents functionality in the 
/// main app process.
#[derive(Debug)]
pub struct AppError(String);

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AppError {}

/// Construct a vfc with a predetermined state.
pub fn build_vfc() -> Vfc {
    use vfc::*;

    let mut vfc = Vfc::new();

    // A subpalette has a size of 8, so I will be grouping my colors 
    // in sets of 8.
    let initial_palette_array = [
        Rgb::new(0x00, 0x11, 0x99), // Black (Background)
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
        if i >= true_palette_array.len() {
            break;
        }
        true_palette_array[i] = color;
    }

    vfc.palette = Palette::new(true_palette_array);

    // vfc.tileset = generate_tileset();
    vfc.tileset.pixel_data[0][0][0] = 0b1111_1111;
    vfc.tileset.pixel_data[0][0][1] = 0b10000000;
    vfc.tileset.pixel_data[0][0][2] = 0b10010100;
    vfc.tileset.pixel_data[0][0][3] = 0b10010100;
    vfc.tileset.pixel_data[0][0][4] = 0b10000000;
    vfc.tileset.pixel_data[0][0][5] = 0b10111110;
    vfc.tileset.pixel_data[0][0][6] = 0b10011100;
    vfc.tileset.pixel_data[0][0][7] = 0b1000_0000;

    vfc.background_color = PaletteIndex(0);

    vfc
}
