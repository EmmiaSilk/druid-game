//! This module contains a set of functions for determining the outcome 
//! of battle.

use std::ops::Mul;

use crate::combatant::Combatant;

/// A list specifiying possible results of an attempted attack.
// TODO: How do you get an attack result?
#[derive(PartialEq, Debug)]
pub enum AttackResult {
    /// The attack will deal maximum damage.
    DirectHit,
    /// The attack will deal half damage.
    GlancingBlow,
    /// The attacker missed and dealt no damage. 
    Miss,
    /// The attacker doesn't have a weapon to attack with. 
    NoWeapon,
}

/// Revolves the result of an attack based on a dice roll and the stats of an 
/// attacker and defender. The provided die roll is compared with a hit rate 
/// provided by [`calculate_hit_rate`].
/// 
/// # Die Rolls
///  
/// The dice roll is usually assumed to be between 1 and 100 inclusive.
/// A low roll on the die is more likely to provide a direct hit, whereas a 
/// high roll is likely to result in a glancing blow.
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
/// 
/// let hit_rate = 50;
/// 
/// let mut attacker = Combatant::new("Attacker".to_string());
/// attacker.give_weapon(Weapon::new("Dummy Weapon".to_string(), hit_rate, 10));
/// let defender = Combatant::new("Defender".to_string()); 
/// 
/// // This attack will be a direct hit, because the dice rolled below the hit rate.
/// let dice_roll = 40;
/// let attack_result = battle::resolve_attack(dice_roll, &attacker, &defender);
/// assert_eq!(battle::AttackResult::DirectHit, attack_result);
/// 
/// // This attack will be a glancing blow, because the dice rolled above the hit rate.
/// let dice_roll = 60;
/// let attack_result = battle::resolve_attack(dice_roll, &attacker, &defender);
/// assert_eq!(battle::AttackResult::GlancingBlow, attack_result);
/// 
/// // This attack will be direct hit, because the dice roll and hit rate are
/// // the same. "If you meet it, you beat it."
/// let dice_roll = 50;
/// let attack_result = battle::resolve_attack(dice_roll, &attacker, &defender);
/// assert_eq!(battle::AttackResult::DirectHit, attack_result);
/// ```
/// 
/// # Special Case: No Weapon
/// 
/// If the given attacker is not wielding a weapon, this function will return 
/// a special outcome, [`AttackResult::NoWeapon`]. This is meant to indicate 
/// that the attack will not have an effect. 
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// 
/// let dice_roll = 50;
/// 
/// // This attacker has not yet been given a weapon!
/// let mut attacker = Combatant::new("Attacker".to_string());
/// let mut defender = Combatant::new("Defender".to_string());
/// 
/// let attack_result = battle::resolve_attack(dice_roll, &attacker, &defender);
/// assert_eq!(battle::AttackResult::NoWeapon, attack_result);
/// ```
pub fn resolve_attack(dice_roll: i32, attacker: &Combatant, defender: &Combatant) -> AttackResult {
    if attacker.current_weapon().is_none() {
        return AttackResult::NoWeapon;
    }

    let hit_rate = match calculate_hit_rate(attacker, defender) {
        None => return AttackResult::Miss, // Automatic miss
        Some(hit_rate) => hit_rate,
    };

    if dice_roll <= hit_rate {
        AttackResult::DirectHit
    }
    else {
        AttackResult::GlancingBlow
    }
}


/// Calculates the chance of the attacker hitting the defender with an attack 
/// using their currently-wielded Weapon.
/// 
/// # Examples
/// 
/// Basic usage:
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
/// 
/// let mut attacker = Combatant::new("Attacker".to_string());
/// let weapon = Weapon::new("Dummy Weapon".to_string(), 50, 10);
/// attacker.give_weapon(weapon);
/// 
/// let defender = Combatant::new("Defender".to_string());
/// 
/// let hit_rate = battle::calculate_hit_rate(&attacker, &defender);
/// assert_eq!(Some(50), hit_rate);
/// ```
/// 
/// Factors that can affect the hit rate are detailed below.
/// 
/// # Weapon Hit Rates
/// 
/// Increasing the hit rate of the attacker's weapon will positively affect the 
/// likelihood that the attack will hit. 
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
///
/// let mut attacker = Combatant::new("Attacker".to_string());
/// let defender = Combatant::new("Defender".to_string());
///
/// // Low hit rate
/// attacker.give_weapon(Weapon::new("Dummy Weapon".to_string(), 40, 5));
/// let hit_rate = battle::calculate_hit_rate(&attacker, &defender);
/// assert_eq!(Some(40), hit_rate);
///
/// // High hit rate
/// attacker.give_weapon(Weapon::new("Dummy Weapon".to_string(), 60, 5));
/// let hit_rate = battle::calculate_hit_rate(&attacker, &defender);
/// assert_eq!(Some(60), hit_rate);
/// ``` 
/// 
/// # Attacker Accuracy
/// 
/// Increasing the attacker's accuracy score will positively affect the 
/// likelihood that the attack will hit. 
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
///
/// let mut attacker = Combatant::new("Attacker".to_string());
/// attacker.give_weapon(Weapon::new("Dummy Weapon".to_string(), 50, 5));
/// let defender = Combatant::new("Defender".to_string());
///
/// // Low hit rate
/// attacker.stats.accuracy = -10;
/// let hit_rate = battle::calculate_hit_rate(&attacker, &defender);
/// assert_eq!(Some(40), hit_rate);
///
/// // High hit rate
/// attacker.stats.accuracy = 10;
/// let hit_rate = battle::calculate_hit_rate(&attacker, &defender);
/// assert_eq!(Some(60), hit_rate);
/// ``` 
/// 
/// # Defender Evasion
/// 
/// Increasing the defender's evasion score will negatively affect the 
/// likelihood that the attack will hit.
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
///
/// let mut attacker = Combatant::new("Attacker".to_string());
/// attacker.give_weapon(Weapon::new("Dummy Weapon".to_string(), 50, 5));
/// let mut defender = Combatant::new("Defender".to_string());
///
/// // Low hit rate
/// defender.stats.evasion = -10;
/// let hit_rate = battle::calculate_hit_rate(&attacker, &defender);
/// assert_eq!(Some(60), hit_rate);
///
/// // High hit rate
/// defender.stats.evasion = 10;
/// let hit_rate = battle::calculate_hit_rate(&attacker, &defender);
/// assert_eq!(Some(40), hit_rate);
/// ```
pub fn calculate_hit_rate(attacker: &Combatant, defender: &Combatant) -> Option<i32> {
    let mut hit_rate = match attacker.current_weapon() {
        None => return None,
        Some(weapon) => weapon.hit_rate,
    };

    // Attacker accuracy
    hit_rate += attacker.stats.accuracy;

    // Defender
    hit_rate -= defender.stats.evasion;

    Some(hit_rate)
}

/// Calculates the damage of an attack based on attack result and the stats of 
/// the attacker and defender. 
/// 
/// # Effects of Attack Results
/// 
/// The `attack_result` parameter has a variety of impacts on the damage dealt.
/// 
/// If the `attack_result` is [`AttackResult::DirectHit`], the calculation 
/// will result in full damage, whereas [`AttackResult::GlancingBlow`] 
/// results in half damage.
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
/// 
/// let mut attacker = Combatant::new("Attacker".to_string());
/// attacker.give_weapon(Weapon::new("Dummy Sword".to_string(), 50, 10));
/// let defender = Combatant::new("Defender".to_string());
/// 
/// let attack_result = battle::AttackResult::DirectHit;
/// let damage = battle::calculate_damage(&attack_result, &attacker, &defender);
/// assert_eq!(Some(10), damage);
/// 
/// let attack_result = battle::AttackResult::GlancingBlow;
/// let damage = battle::calculate_damage(&attack_result, &attacker, &defender);
/// assert_eq!(Some(5), damage);
/// ```
/// 
/// If the `attack_result` is [`AttackResult::Miss`] or 
/// [`AttackResult::NoWeapon`], the calculation instead results in 
/// [`Option::None`]. 
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
/// 
/// let mut attacker = Combatant::new("Attacker".to_string());
/// attacker.give_weapon(Weapon::new("Dummy Sword".to_string(), 50, 10));
/// let defender = Combatant::new("Defender".to_string());
/// 
/// let attack_result = battle::AttackResult::Miss;
/// let damage = battle::calculate_damage(&attack_result, &attacker, &defender);
/// assert_eq!(None, damage);
/// 
/// let attack_result = battle::AttackResult::NoWeapon;
/// let damage = battle::calculate_damage(&attack_result, &attacker, &defender);
/// assert_eq!(None, damage);
/// ```
/// 
/// # Stats
/// 
/// Increasing the damage of the attacker's weapon will increase the resulting 
/// damage.
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
/// 
/// let attack_result = battle::AttackResult::DirectHit;
/// let mut attacker = Combatant::new("Attacker".to_string());
/// let defender = Combatant::new("Defender".to_string());
/// 
/// attacker.give_weapon(Weapon::new("Big Sword".to_string(), 50, 15));
/// let damage = battle::calculate_damage(&attack_result, &attacker, &defender);
/// assert_eq!(Some(15), damage);
/// ```
/// 
/// Increasing the attacker's strength will also increase the resuling damage, 
/// whereas increasing the defender's defense will decrease it.
/// 
/// ```
/// use druid_game::battle;
/// use druid_game::combatant::Combatant;
/// use druid_game::weapon::Weapon;
/// 
/// let attack_result = battle::AttackResult::DirectHit;
///
/// let mut attacker = Combatant::new("Attacker".to_string());
/// attacker.give_weapon(Weapon::new("Dummy Sword".to_string(), 50, 10));
/// let mut defender = Combatant::new("Defender".to_string());
/// 
/// attacker.stats.strength = 5;
/// let damage = battle::calculate_damage(&attack_result, &attacker, &defender);
/// assert_eq!(Some(15), damage);
///
/// attacker.stats.strength = 0; 
/// defender.stats.defense = 5;
/// let damage = battle::calculate_damage(&attack_result, &attacker, &defender);
/// assert_eq!(Some(5), damage);
/// ```
/// 
pub fn calculate_damage(attack_result: &AttackResult, attacker: &Combatant, defender: &Combatant) -> Option<i32> {
    // Attack effectiveness multiplier
    let multiplier = match attack_result {
        AttackResult::Miss => return None,
        AttackResult::NoWeapon => return None,
        AttackResult::DirectHit => 1.0,
        AttackResult::GlancingBlow => 0.5, 
    };

    // Calculate base damage
    let mut damage = match attacker.current_weapon() {
        None => return None,
        Some(weapon) => weapon.damage,
    };
    damage += attacker.stats.strength;
    damage -= defender.stats.defense;

    // Multiplier
    let damage = (damage as f64).mul(multiplier);

    Some(damage as i32)
}