//! This library contains the meat of the code for the druid game. 
//! 
//! # Usage
//! Define the services laid out in [`ServiceContainer`], then execute the 
//! [`run`] function.

#![warn(missing_docs)]
pub mod combatant;
pub mod battle;
pub mod weapon;
pub mod render;
pub mod io;
pub mod input;
pub mod app;
pub mod combat;