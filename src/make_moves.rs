use crate::direction_enum::*;
use crate::field_enum::*;
use crate::game::*;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn make_moves(
    directions: Arc<Mutex<Vec<Direction>>>,
    game: Arc<Mutex<Game>>,
    duration_between_moves: u64,
    probability_of_appear_of_fruit: f64,
) {
    let mut rng = rand::thread_rng();
    loop {
        sleep(Duration::new(duration_between_moves, 0));

        let mut game_guard = game.lock().unwrap();
        let mut directions_guard = directions.lock().unwrap();
        let direction = match directions_guard
            .iter()
            .filter(|&direction| direction != &opposite_direction(&game_guard.direction))
            .choose(&mut rng)
        {
            Some(&dir) => dir,
            None => game_guard.direction,
        };
        game_guard.direction = direction;

        let (x, y) = game_guard.get_cords_after_move(&game_guard.direction);
        match game_guard.fields.get(y, x) {
            Some(Field::Empty) => {
                let (old_x, old_y) = game_guard.snake_position.pop_back().unwrap();
                game_guard.fields.set(old_y, old_x, Field::Empty).unwrap();
                game_guard.empty_fields.insert((old_x, old_y));

                game_guard.snake_position.push_front((x, y));
                game_guard.fields.set(y, x, Field::Snake).unwrap();
                game_guard.empty_fields.remove(&(x, y));

                directions_guard.clear();

                if rng.gen_bool(probability_of_appear_of_fruit) {
                    game_guard.insert_fruit(&mut rng);
                }
            }
            Some(Field::Fruit) => {
                game_guard.snake_position.push_front((x, y));
                game_guard.fields.set(y, x, Field::Snake).unwrap();

                directions_guard.clear();
            }
            Some(Field::Snake) => {
                let snake_tail = *game_guard.snake_position.back().unwrap();
                if snake_tail == (x, y) {
                    game_guard.snake_position.rotate_right(1);
                } else {
                    game_guard.restart_game();
                }
            }
            None => (),
        }
    }
}
