//! This library contains the meat of the code for the druid game. 
//! 
//! # Usage
//! Define the services laid out in [`ServiceContainer`], then execute the 
//! [`run`] function.

#![warn(missing_docs)]
use std::error::Error;
use combatant::Combatant;
use render::{RenderContext, Bitmap};
use io::AssetLoader;
use vfc::{Vfc, SCREEN_HEIGHT, SCREEN_WIDTH};
use weapon::Weapon;
use battle::{AttackResult, calculate_damage};

use crate::combatant::HealthStatus;

pub mod combatant;
pub mod battle;
pub mod weapon;
pub mod render;
pub mod io;

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
}

/// The starting point for the game.
pub async fn run(services: ServiceContainer) -> Result<(), Box<dyn Error>> {
    // Load
    let bitmap = services.asset_loader.load_bitmap("/asset/example.png").await?;
    // Draw to the canvas
    services.render_context.draw(&bitmap, 0, 0)?;

    let mut vfc = build_vfc();
    vfc.render_frame();
    let fb = &mut vfc.framebuffer;

    let bitmap = Bitmap::from_framebuffer(SCREEN_WIDTH, SCREEN_HEIGHT, fb);
    services.render_context.draw(&bitmap, 0, 0)?;

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

    vfc.bg_layers[0].tiles[0] = TileIndex(0);
    vfc.bg_layers[0].hidden = false;

    vfc.background_color = PaletteIndex(0);

    vfc
}
