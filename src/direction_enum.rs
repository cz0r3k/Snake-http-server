#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

pub fn map_direction(value: &str) -> Result<Direction, &'static str> {
    match value {
        "top" => Ok(Direction::Top),
        "right" => Ok(Direction::Right),
        "bottom" => Ok(Direction::Bottom),
        "left" => Ok(Direction::Left),
        _ => Err("Invalid direction"),
    }
}

pub fn opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Top => Direction::Bottom,
        Direction::Bottom => Direction::Top,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
    }
}

impl From<&Direction> for (i32, i32) {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Top => (0, -1),
            Direction::Bottom => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
}
