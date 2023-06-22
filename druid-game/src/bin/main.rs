use std::{error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = druid_game::combat_example() {
        eprintln!("Application eror: {e}");
        process::exit(1);
    }

    Ok(())
}