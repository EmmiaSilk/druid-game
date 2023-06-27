//! This library contains the meat of the code for the druid game. 
//! 
//! # Usage
//! Define the services laid out in [`service::ServiceContainer`], then execute the 
//! [`app::run`] function.

#![warn(missing_docs)]
pub mod app;
pub mod battle;
pub mod combat;
pub mod combatant;
pub mod render;
pub mod service;
pub mod weapon;
