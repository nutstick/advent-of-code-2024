#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn checked_add(self, idx: (usize, usize)) -> Option<(usize, usize)> {
        let pair: (isize, isize) = self.into();
        match (
            idx.0.checked_add_signed(pair.0),
            idx.1.checked_add_signed(pair.1),
        ) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }
}
