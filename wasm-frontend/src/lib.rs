mod utils;

use web_sys::console;
use wasm_bindgen::prelude::*;

use druid_game::combatant::Combatant;
use druid_game::weapon::Weapon;
use druid_game::battle;
use druid_game::battle::AttackResult;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut hero_alice = Combatant::new("Alice".to_string());
    hero_alice.give_weapon(Weapon::new("Longsword".to_string(), 70, 8));
    let mut villain_vim = Combatant::new("Vim".to_string());
    villain_vim.give_weapon(Weapon::new("Longsword".to_string(), 70, 8));

    attack(&mut hero_alice, &mut villain_vim);
    console::log_0();

    attack(&mut villain_vim, &mut hero_alice);
    console::log_0();

    attack(&mut hero_alice, &mut villain_vim);
    console::log_0();
}

fn attack(attacker: &mut Combatant, defender: &mut Combatant) {
    log!("{0} attacks {1}", attacker, defender);
    let dice_roll = 50;
    let attack_result = battle::resolve_attack(dice_roll, attacker, defender);
    match attack_result {
        AttackResult::Miss => log!("{0} missed!", attacker),
        AttackResult::NoWeapon => log!("{0} didn't equip a weapon!", attacker),
        AttackResult::DirectHit => {
            log!("It's a direct hit!");
            damage_step(&attack_result, attacker, defender);
        },
        AttackResult::GlancingBlow => {
            log!("It's a glancing blow.");
            damage_step(&attack_result, attacker, defender);
        },
    }
}

fn damage_step(attack_result: &AttackResult, attacker: &mut Combatant, defender: &mut Combatant) {
    use druid_game::combatant::HealthStatus;

    if let Some(damage) = battle::calculate_damage(attack_result, attacker, defender) {
        log!("{0} takes {1} damage.", defender, damage);
        let status = defender.health.damage(damage);
        log!("{0} has {1} hit points remaining.", defender, defender.health.current());
        if let HealthStatus::Defeated = status {
            log!("{defender} is defeated!");
        }
    }
}