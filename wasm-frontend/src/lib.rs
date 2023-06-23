use std::error::Error;

use druid_game::ServiceContainer;
use input::WebInputManager;
use wasm_bindgen::prelude::*;

mod utils;
mod macros;
mod loading;
mod render_context;
mod input;

use render_context::WebRenderContext;

use crate::loading::WebAssetLoader;

#[wasm_bindgen]
pub async fn run() -> Result<(), JsError> {
    log!("Entered webassembly!");

    // Activate panic hook
    console_error_panic_hook::set_once();
    
    // Generate Services
    let mut services = generate_services()?;

    // Run the game!
    let result = druid_game::run(services).await;
    match result {
        Ok(()) => log!("Complete!"),
        Err(error) => return Err(JsError::new(&format!("Application error: {}", error))),
    }

    Ok(())
}

fn generate_services() -> Result<ServiceContainer, JsError> {
    // Render context
    let render_context = match WebRenderContext::with_id("canvas") {
        Err(error) => return Err(JsError::new(&format!("Error obtaining canvas context: {}", error))),
        Ok(context) => context,
    };

    // Asset loader
    let asset_loader = WebAssetLoader::new();

    // Input manager
    let input_manager = WebInputManager::create();

    let vfc = druid_game::build_vfc();

    // Container
    let services = druid_game::ServiceContainer {
        render_context: Box::new(render_context), 
        asset_loader: Box::new(asset_loader),
        input_manager: Box::new(input_manager),
        vfc: Box::new(vfc),
    };

    Ok(services)
}
