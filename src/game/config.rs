use std::error::Error;

use super::cli_utils;
use super::field::{Difficulty,Field};


/// handles setup for the game
pub struct Config {
    pub field:Field,
}
impl Config {
    /// creates and returns a new Config from user input
    pub fn new() -> Result<Config, Box<dyn Error>> {
        //DATA
        let difficulty: Difficulty;
        let mut config: Config = Config {
            field:Field::new(),
        };

        //get data from user input
        //input loop
        loop { match cli_utils::get_string_from_user_input("DIFFICULTY (B = Beginner, I = Intermediate, A = Advanced): ") {
            Ok (s) => {
                match s.to_ascii_lowercase().chars().nth(0) {
                    Some('b') => difficulty = Difficulty::BEGINNER,
                    Some('i') => difficulty = Difficulty::INTERMEDIATE,
                    Some('a') => difficulty = Difficulty::ADVANCED,
                    _ => {eprintln!("invalid difficulty"); continue;},
                }
                break;
            },
            Err(e) => eprintln!("{}",e),
        }}

        //populate field
        config.field.populate(difficulty);

        //return new config
        return Ok(config);
    }
}
