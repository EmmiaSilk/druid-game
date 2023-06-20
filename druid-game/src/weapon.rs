//! This module specifies the [`Weapon`] type.

use std::fmt::Display;

/// A representation of a weapon used in combat.  
pub struct Weapon {
    /// The name used to refer to the weapon in text.
    pub name: String,
    /// The base frequency with which this weapon hits, compared to a roll 
    /// from 1 through 100.
    pub hit_rate: i32,
    /// The base amount of damage this weapon deals on a direct hit.
    pub damage: i32,
}
impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Weapon {
    /// Constructs a weapon with the given parameters.
    /// 
    /// # Examples
    /// 
    /// Basic usage
    /// 
    /// ```
    /// use druid_game::weapon::Weapon;
    /// 
    /// Weapon::new("Blessed Longsword".to_string(), 90, 12);
    /// ``` 
    pub fn new(name: String, hit_rate: i32, damage: i32) -> Weapon {
        Weapon { name, hit_rate, damage }
    }
}