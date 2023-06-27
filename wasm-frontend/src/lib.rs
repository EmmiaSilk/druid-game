use druid_game::service::ServiceContainer;
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
    let services = generate_services()?;

    // Run the game!
    let result = druid_game::app::run(services).await;
    match result {
        Ok(()) => log!("Complete!"),
        Err(error) => return Err(JsError::new(&format!("Application error: {}", error))),
    }

    Ok(())
}

fn generate_services() -> Result<ServiceContainer, JsError> {
    let mut services = ServiceContainer::new();

    // Render context
    let render_context = match WebRenderContext::with_id("canvas") {
        Err(error) => return Err(JsError::new(&format!("Error obtaining canvas context: {}", error))),
        Ok(context) => context,
    };

    // Asset loader
    let asset_loader = WebAssetLoader::new();

    // Input manager
    let input_manager = WebInputManager::create();

    // VFC
    let vfc = druid_game::app::build_vfc();

    services.register_render_context(Box::new(render_context))?;
    services.register_asset_loader(Box::new(asset_loader))?;
    services.register_input_manager(Box::new(input_manager))?;
    services.register_vfc(Box::new(vfc))?;

    Ok(services)
}
