//! This module specifies the [`Combatant`] type, as well as [`CombatStats`] 
//! for use by it. 

use std::fmt::Display;
use crate::weapon::Weapon;

/// A representation of a character that might participate in combat. 
pub struct Combatant {
    /// The combatant's name, used to refer to them in text.
    pub name: String,
    /// Combat statistics
    pub stats: CombatStats,
    /// How much damage they can take before being defeated. 
    pub health: Health,
    current_weapon: Option<Weapon>,
}
impl Display for Combatant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Combatant {
    /// Initializes a combatant with health of 10 and default stats of all 0. 
    /// 
    /// ## Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use druid_game::combatant::Combatant;
    /// 
    /// let hero = Combatant::new("Hero of the Week".to_string());
    pub fn new(name: String) -> Combatant {
        Combatant { 
            name,
            stats: CombatStats::new(), 
            health: Health::new(10), 
            current_weapon: None, 
        }
    }

    /// Borrows a reference to the combatant's current weapon.
    /// 
    /// ## Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use druid_game::combatant::Combatant;
    /// use druid_game::weapon::Weapon;
    /// 
    /// let weapon = Weapon::new("Longsword".to_string(), 80, 10);
    /// 
    /// let mut wielder = Combatant::new("Hero of the Week".to_string());
    /// wielder.give_weapon(weapon);
    /// 
    /// let current_weapon = wielder.current_weapon();
    /// ```
    pub fn current_weapon(&self) -> &Option<Weapon> {
        &self.current_weapon
    }

    /// The combatant takes ownership of the given weapon and equips it as 
    /// their current weapon.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use druid_game::combatant::Combatant;
    /// use druid_game::weapon::Weapon;
    /// 
    /// let weapon = Weapon::new("Blessed Longsword".to_string(), 90, 12);
    /// 
    /// let mut wielder = Combatant::new("Mysterious Figure".to_string());
    /// wielder.give_weapon(weapon);
    /// ```
    pub fn give_weapon(&mut self, weapon: Weapon) {
        self.current_weapon = Some(weapon);
    }
}

/// A set of stats used in calculating combat values.
#[derive(Default)]
pub struct CombatStats {
    /// Affects how likely they are to direct-hit with an attack.
    pub accuracy: i32,
    /// Affects how likely they are to avoid suffering a direct hit.
    pub evasion: i32,
    /// Affects how much damage their attacks deal.
    pub strength: i32,
    /// Affects how much they can reduce the damage they take.
    pub defense: i32,
}

impl CombatStats {
    /// Initialize a set of combat stats with all `0` values.
    pub fn new() -> CombatStats {
        CombatStats {
            accuracy: 0,
            evasion: 0,
            strength: 0,
            defense: 0,
        }
    }
}

/// Enum specifying general health states.
#[derive(PartialEq, Debug)]
pub enum HealthStatus {
    /// The subject has its maximum health.
    Healthy,
    /// The subject has less than its maximum health.
    Hurt,
    /// The subject has 0 health.
    Defeated,
}

/// A creature's vitality, as represented by an integer. 
/// 
/// Health is bound between `0` and a maximum value, which can be manipulated. 
/// Most functions which alter health also return a [`HealthStatus`] to gauge 
/// current health relative to the maxiumum. 
pub struct Health {
    current: i32,
    max: i32,
}

impl Health {
    /// Construct a new `Health` object, with a maximum and current value of 
    /// the given value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use druid_game::combatant::Health;
    /// 
    /// let health = Health::new(10);
    /// ``` 
    pub fn new(max: i32) -> Self {
        Health {
            current: max,
            max
        }
    }

    /// Returns the maximum health.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use druid_game::combatant::Health;
    /// 
    /// let mut health = Health::new(10);
    /// 
    /// health.damage(5);
    /// assert_eq!(5, health.current());
    /// ```
    pub fn current(&self) -> i32 {
        self.current
    }

    /// Returns the maximum health.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use druid_game::combatant::Health;
    /// 
    /// let health = Health::new(10);
    /// 
    /// assert_eq!(10, health.max());
    /// ```
    pub fn max(&self) -> i32 {
        self.max
    }
    
    /// Reduces the current health by the given damage, then returns the 
    /// current health status.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use druid_game::combatant::Health;
    /// use druid_game::combatant::HealthStatus;
    /// 
    /// let mut health = Health::new(10);
    /// 
    /// let new_status = health.damage(7);
    /// assert_eq!(HealthStatus::Hurt, new_status);
    /// assert_eq!(3, health.current());
    /// 
    /// let new_status = health.damage(7);
    /// assert_eq!(HealthStatus::Defeated, new_status);
    /// assert_eq!(0, health.current());
    /// ```
    pub fn damage(&mut self, damage: i32) -> HealthStatus {
        self.current -= damage;
        self.clamp();
        self.check_status()
    }

    /// Clamps current health to the range of `0..max` inclusive. 
    /// 
    /// Must call every time current health is changed.
    fn clamp(&mut self) {
        self.current = self.current.clamp(0, self.max);
    }

    /// Return a HealthStatus based on the current health compared to 
    /// the maximum.
    /// 
    /// The status is Healthy when the current health is at its maximum value, 
    /// Hurt when it is between 0 and the maximum, and Defeated if it's at 0.
    /// 
    /// Most functions that alter hit points also check the status.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use druid_game::combatant::Health;
    /// use druid_game::combatant::HealthStatus;
    /// 
    /// let health = Health::new(10);
    /// 
    /// let status = health.check_status();
    /// assert_eq!(HealthStatus::Healthy, status);
    /// ```
    pub fn check_status(&self) -> HealthStatus {
        if self.current >= self.max {
            return HealthStatus::Healthy
        }
        else if self.current <= 0 {
            return HealthStatus::Defeated
        } 
        HealthStatus::Hurt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_healthy_status() {
        let health = Health::new(10);

        let actual = health.check_status();
        assert_eq!(HealthStatus::Healthy, actual,
            "Health must initialize to healthy status.");
    }

    #[test]
    fn test_hurt_status() {
        let mut health = Health::new(10);
        health.damage(5);

        let actual = health.check_status();
        assert_eq!(HealthStatus::Hurt, actual,
            "Health status must be hurt after taking damage.");
    }
    
    #[test]
    fn test_defeated_status() {
        let mut health = Health::new(10);
        health.damage(10);

        let actual = health.check_status();
        assert_eq!(HealthStatus::Defeated, actual,
            "Health status must be defeated after reducing health to 0.");
    }
}