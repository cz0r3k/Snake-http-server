use crate::direction_enum::*;
use crate::field_enum::*;
use array2d::Array2D;
use indexmap::IndexSet;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::VecDeque;

pub struct Game {
    width: usize,
    height: usize,
    pub direction: Direction,
    pub fields: Array2D<Field>,
    pub snake_position: VecDeque<(usize, usize)>,
    pub empty_fields: IndexSet<(usize, usize)>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let (x, y) = Game::get_default_snake_cord(width, height);

        let mut empty_fields = IndexSet::new();
        for i in 0..width {
            for j in 0..height {
                empty_fields.insert((i, j));
            }
        }
        empty_fields.remove(&(x, y));
        empty_fields.remove(&(x - 1, y));

        let mut fields = Array2D::filled_with(Field::Empty, height, width);
        fields.set(y, x, Field::Snake).expect("Adding snake");
        fields.set(y, x - 1, Field::Snake).expect("Adding snake");

        let mut snake_position = VecDeque::<(usize, usize)>::new();
        snake_position.push_back((x, y));
        snake_position.push_back((x - 1, y));

        Game {
            width,
            height,
            direction: Direction::Right,
            fields,
            snake_position,
            empty_fields,
        }
    }
    pub fn print_fields(&self) -> String {
        self.fields
            .rows_iter()
            .map(|row| row.map(char::from).collect::<String>() + "\n")
            .collect::<String>()
    }

    pub fn get_cords_after_move(&self, direction: &Direction) -> (usize, usize) {
        let snake_head = self.snake_position.front().expect("Get head");
        self.normalize_cord(&sum_tuples(
            &(snake_head.0 as i32, snake_head.1 as i32),
            &(<(i32, i32)>::from(direction)),
        ))
    }

    fn normalize_cord(&self, cord: &(i32, i32)) -> (usize, usize) {
        (
            (cord.0 + self.width as i32) as usize % self.width,
            (cord.1 + self.height as i32) as usize % self.height,
        )
    }

    pub fn restart_game(&mut self) {
        let (x, y) = Game::get_default_snake_cord(self.width, self.height);
        let mut fields = Array2D::filled_with(Field::Empty, self.height, self.width);
        fields.set(y, x, Field::Snake).expect("Adding snake");
        fields.set(y, x - 1, Field::Snake).expect("Adding snake");

        let mut snake_position = VecDeque::<(usize, usize)>::new();
        snake_position.push_back((x, y));
        snake_position.push_back((x - 1, y));

        let mut empty_fields = IndexSet::new();
        for i in 0..self.width {
            for j in 0..self.height {
                empty_fields.insert((i, j));
            }
        }
        empty_fields.remove(&(x, y));
        empty_fields.remove(&(x - 1, y));

        self.direction = Direction::Right;
        self.fields = fields;
        self.snake_position = snake_position;
        self.empty_fields = empty_fields;
    }

    fn get_default_snake_cord(width: usize, height: usize) -> (usize, usize) {
        (width / 2, height / 2)
    }

    fn get_random_empty_field(&self, rng: &mut ThreadRng) -> (usize, usize) {
        let empty_fields_size = self.empty_fields.len();
        *self
            .empty_fields
            .get_index(rng.gen_range(0..empty_fields_size))
            .expect("Empty fields get")
    }

    pub fn insert_fruit(&mut self, rng: &mut ThreadRng) {
        let (x, y) = self.get_random_empty_field(rng);
        self.fields
            .set(y, x, Field::Fruit)
            .expect("Fields add fruit");
        self.empty_fields.remove(&(x, y));
    }
}

fn sum_tuples(t1: &(i32, i32), t2: &(i32, i32)) -> (i32, i32) {
    (t1.0 + t2.0, t1.1 + t2.1)
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::game::sum_tuples;

    #[test]
    fn sum_tuples_test() {
        let t1 = (1, 1);
        let t2 = (-1, 1);
        let result = sum_tuples(&t1, &t2);
        assert_eq!(result, (0, 2));
    }
    #[test]
    fn get_default_snake_cord_test1() {
        let result = Game::get_default_snake_cord(4, 4);
        assert_eq!(result, (2, 2));
    }

    #[test]
    fn get_default_snake_cord_test2() {
        let result = Game::get_default_snake_cord(5, 5);
        assert_eq!(result, (2, 2));
    }
}
