use config_file::FromConfigFile;
use serde::Deserialize;

const DEFAULT_WIDTH: usize = 10;
const DEFAULT_HEIGHT: usize = 10;
const DEFAULT_DURATION_BETWEEN_MOVES: u64 = 5;
const DEFAULT_PROBABILITY_OF_APPEAR_OF_FRUIT: f64 = 0.1;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_width")]
    pub width: usize,
    #[serde(default = "default_height")]
    pub height: usize,
    #[serde(default = "default_duration_between_moves")]
    pub duration_between_moves: u64,
    #[serde(default = "default_probability_of_appear_of_fruit")]
    pub probability_of_appear_of_fruit: f64,
}

pub fn read_config(config_path: &str) -> Config {
    let mut config = match Config::from_config_file(config_path) {
        Ok(config) => config,
        Err(_) => Config {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            duration_between_moves: DEFAULT_DURATION_BETWEEN_MOVES,
            probability_of_appear_of_fruit: DEFAULT_PROBABILITY_OF_APPEAR_OF_FRUIT,
        },
    };
    if config.width < 2 {
        config.width = DEFAULT_WIDTH;
    }
    if config.height < 2 {
        config.height = DEFAULT_HEIGHT;
    }
    if config.probability_of_appear_of_fruit > 1_f64 {
        config.probability_of_appear_of_fruit = DEFAULT_PROBABILITY_OF_APPEAR_OF_FRUIT;
    }
    config
}

fn default_width() -> usize {
    DEFAULT_WIDTH
}

fn default_height() -> usize {
    DEFAULT_HEIGHT
}

fn default_duration_between_moves() -> u64 {
    DEFAULT_DURATION_BETWEEN_MOVES
}

fn default_probability_of_appear_of_fruit() -> f64 {
    DEFAULT_PROBABILITY_OF_APPEAR_OF_FRUIT
}
