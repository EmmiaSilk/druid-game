use std::cell::RefCell;
use std::rc::Rc;
use std::env;
use std::error::Error;

use druid_game::app::ServiceContainer;
use minifb::Scale;
use minifb::WindowOptions;
use minifb::Window;

mod render_context;
mod asset_loader;
mod input_manager;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let services = generate_services()?;

    println!("Dir: {}", env::current_dir().unwrap().display());
    druid_game::app::run(services).await?;
    println!("Complete!");

    Ok(())
}

fn generate_services() -> Result<ServiceContainer, Box<dyn Error>> {
    // NOTE: This may or may not be a memory leak =x
    let window = Rc::new(RefCell::new(create_window()));

    let render_context = render_context::MiniFBRenderContext::create(window.clone());
    let asset_loader = asset_loader::LocalAssetLoader::create();
    let input_manager = input_manager::WindowInputManager::create(window.clone());
    let vfc = druid_game::app::build_vfc();

    Ok(ServiceContainer {
        render_context: Box::new(render_context),
        asset_loader: Box::new(asset_loader),
        input_manager: Box::new(input_manager),
        vfc: Box::new(vfc),
    })
}

const HEIGHT: usize = vfc::SCREEN_HEIGHT;
const WIDTH: usize = vfc::SCREEN_WIDTH;

fn create_window() -> Window {
    // Create Window
    let mut window = Window::new(
        "Druid Game!",
        WIDTH, HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        }
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit update rate to ~60 fps
    window.limit_update_rate(Some(std::time::Duration::from_secs_f64(1.0/60.0)));

    window
}
