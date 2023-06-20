//! This library contains the meat of the code for the druid game. 

#![warn(missing_docs)]
use std::error::Error;
use combatant::Combatant;
use weapon::Weapon;
use battle::{AttackResult, calculate_damage};

use crate::combatant::HealthStatus;

pub mod combatant;
pub mod battle;
pub mod weapon;

/// The starting point for the game.
pub fn run() -> Result<(), Box<dyn Error>> {
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