#[derive(Debug, Clone)]
pub enum Field {
    Snake,
    Fruit,
    Empty,
}

impl From<&Field> for char {
    fn from(field: &Field) -> Self {
        match field {
            Field::Snake => '#',
            Field::Fruit => '*',
            Field::Empty => '_',
        }
    }
}
