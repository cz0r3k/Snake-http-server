#![feature(iter_intersperse)]
mod config;
mod direction_enum;
mod endpoints;
mod field_enum;
mod game;
mod make_moves;

use crate::config::*;
use crate::direction_enum::*;
use crate::endpoints::*;
use crate::game::*;
use crate::make_moves::*;

#[macro_use]
extern crate rocket;
use std::sync::{Arc, Mutex};
use std::thread;

#[rocket::main]
async fn main() {
    let config = read_config("config.toml");

    let directions1 = Arc::new(Mutex::new(Vec::<Direction>::new()));
    let directions2 = Arc::clone(&directions1);

    let game1 = Arc::new(Mutex::new(Game::new(config.width, config.height)));
    let game2 = Arc::clone(&game1);

    thread::spawn(move || {
        make_moves(
            directions1,
            game1,
            config.duration_between_moves,
            config.probability_of_appear_of_fruit,
        );
    });

    let _ = rocket::build()
        .mount("/", routes![index, get_snake, add_direction])
        .manage(SharedData {
            directions: directions2,
            game: game2,
        })
        .launch()
        .await
        .expect("Server error");
}
