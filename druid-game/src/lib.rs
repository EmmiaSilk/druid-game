//! This library contains the meat of the code for the druid game. 
//! 
//! # Usage
//! Define the services laid out in [`ServiceContainer`], then execute the 
//! [`run`] function.

#![warn(missing_docs)]
use std::{error::Error, fmt::Display};
use combatant::Combatant;
use game_loop::{GameLoop, Time};
use render::{RenderContext, Bitmap};
use io::AssetLoader;
use vfc::{Vfc, SCREEN_HEIGHT, SCREEN_WIDTH};
use weapon::Weapon;
use battle::{AttackResult, calculate_damage};
use input::InputManager;

use crate::combatant::HealthStatus;

pub mod combatant;
pub mod battle;
pub mod weapon;
pub mod render;
pub mod io;
pub mod input;

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

#[derive(Debug)]
pub struct AppError(String);

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AppError {}

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

/// A combat routine for testing purposes.
pub fn combat_example() -> Result<(), Box<dyn Error>> {
    // TODO: Use a logger somehow
    let mut hero_alice = Combatant::new("Alice".to_string());
    hero_alice.give_weapon(Weapon::new("Longsword".to_string(), 70, 8));
    let mut villain_vim = Combatant::new("Vim".to_string());
    villain_vim.give_weapon(Weapon::new("Longsword".to_string(), 70, 8));

    attack(&mut hero_alice, &mut villain_vim);
    println!();

    attack(&mut villain_vim, &mut hero_alice);
    println!();

    attack(&mut hero_alice, &mut villain_vim);
    println!();

    Ok(())
}

fn attack(attacker: &mut Combatant, defender: &mut Combatant) {
    println!("{0} attacks {1}", attacker, defender);

    let dice_roll = 50;
    let attack_result = battle::resolve_attack(dice_roll, attacker, defender);
    match attack_result {
        AttackResult::Miss => println!("{0} missed!", attacker),
        AttackResult::NoWeapon => println!("{0} didn't equip a weapon!", attacker),
        AttackResult::DirectHit => {
            println!("It's a direct hit!");
            damage_step(&attack_result, attacker, defender);
        },
        AttackResult::GlancingBlow => {
            println!("It's a glancing blow.");
            damage_step(&attack_result, attacker, defender);
        },
    }
}

fn damage_step(attack_result: &AttackResult, attacker: &mut Combatant, defender: &mut Combatant) {
    if let Some(damage) = calculate_damage(attack_result, attacker, defender) {
        println!("{0} takes {1} damage.", defender, damage);
        let status = defender.health.damage(damage);
        println!("{0} has {1} hit points remaining.", defender, defender.health.current());
        if let HealthStatus::Defeated = status {
            println!("{defender} is defeated!");
        }
    }
}

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
