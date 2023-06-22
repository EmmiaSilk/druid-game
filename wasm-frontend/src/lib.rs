use druid_game::ServiceContainer;
use wasm_bindgen::prelude::*;

mod utils;
mod loading;
mod macros;
mod render_context;

use render_context::WebRenderContext;

use crate::loading::WebAssetLoader;

#[wasm_bindgen]
pub async fn run() -> Result<(), JsError> {
    log!("Entered webassembly!");

    // Activate panic hook
    console_error_panic_hook::set_once();
    
    // Generate Services
    let services = generate_services()?;

    // Run the game!
    let _ = druid_game::run(services).await;

    log!("Complete!");

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

    // Container
    let services = druid_game::ServiceContainer {
        render_context: Box::new(render_context), 
        asset_loader: Box::new(asset_loader),
    };

    Ok(services)
}
