/// Direction enum
/// 
/// # Variants
/// 
/// * `Up` - Up direction
/// * `Down` - Down direction
/// * `Left` - Left direction
/// * `Right` - Right direction
/// 
/// # Methods
/// * `coordinate_offset` - Returns the coordinate offset for the direction
/// * `change_direction` - Returns the next direction (90 degree turn clockwise)

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn coordinate_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn change_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}