//! This module is the main entry point of the app.

use std::fmt::Display;
use std::error::Error;

use game_loop::Time;
use game_loop::GameLoop;
use vfc::{SCREEN_HEIGHT, SCREEN_WIDTH};
use vfc::Vfc;

use crate::render::Bitmap;
use crate::service::ServiceContainer;

/// The starting point for the game.
pub async fn run(mut services: ServiceContainer) -> Result<(), Box<dyn Error>> {
    println!("Running!!!");
    // Load
    let asset_loader = &mut *services.asset_loader_mut()?;
    let result = asset_loader.load_bitmap("asset/example.png").await;
    let bitmap = match result {
        Ok(bitmap) => bitmap,
        Err(_) => return Err(Box::new(AppError("Problem loading bitmap".into()))), // TODO: Provide a clearer error
    };
    // Draw to the canvas
    let render_context = &mut *services.render_context_mut()?;
    let result = render_context.draw(&bitmap, 0, 0);
    if let Err(error) = result {
        return Err(Box::new(AppError(error.to_string())));
    }

    game_loop::game_loop(services, 60, 0.1, 
        |g| {
            if let Err(error) = update(g) {
                eprintln!("App error: {}", error.to_string());
                g.exit();
            };
        }, 
        |g| {
            if let Err(error) = render(g) {
                eprint!("Render error: {}", error.to_string());
                g.exit();
            }
        });
    Ok(())
}

/// Update the main game state. 
pub fn update(game_loop: &mut GameLoop<ServiceContainer, Time, ()>) -> Result<(), Box<dyn Error>> {
    let services = &mut game_loop.game;
    let input_manager = services.input_manager_mut()?;
    // Check for exit
    if input_manager.is_requesting_close() {
        game_loop.exit();
        return Ok(());
    }

    Ok(())
}

/// Draw the game visuals to the screen. 
pub fn render(game_loop: &mut GameLoop<ServiceContainer, Time, ()>) -> Result<(), Box<dyn Error>>{
    let services = &mut game_loop.game;
    
    // Modify vfc
    let vfc = services.vfc_mut()
        .expect("The VFC Should have already been verified by now.");
    // Render to framebuffer
    vfc.render_frame();
    let fb = &vfc.framebuffer;
    let bitmap = Bitmap::from_framebuffer(SCREEN_WIDTH, SCREEN_HEIGHT, fb);
    
    // Draw to screen
    let render_context = services.render_context_mut()
        .expect("The Render Context should have already been verified by now.");
    let result = render_context.draw(&bitmap, 0, 0);
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
