use crate::direction_enum::*;
use crate::game::*;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use std::sync::{Arc, Mutex};

pub struct SharedData {
    pub directions: Arc<Mutex<Vec<Direction>>>,
    pub game: Arc<Mutex<Game>>,
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello from snake API!\n\n\
    Show snake\n\
    GET\t/snake\n\n\
    Add direction to direction list\n\
    POST\t/snake/<direction>\n\
    \tWHERE <direction> = top OR right OR bottom OR left"
}

#[get("/snake")]
pub fn get_snake(shared: &State<SharedData>) -> status::Accepted<String> {
    status::Accepted(Some(shared.game.lock().unwrap().print_fields()))
}

#[post("/snake/<direction>")]
pub fn add_direction(direction: &str, shared: &State<SharedData>) -> Status {
    let mut directions_guard = shared.directions.lock().unwrap();
    let direction = match map_direction(&direction.to_lowercase()) {
        Ok(direction) => direction,
        Err(_) => return Status::BadRequest,
    };
    directions_guard.push(direction);
    Status::Ok
}
