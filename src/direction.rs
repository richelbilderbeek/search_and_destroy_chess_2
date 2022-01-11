/// A square on a chess board
/// ```
/// use search_and_destroy_chess_2::direction::Direction;
/// use search_and_destroy_chess_2::direction::to_str;
/// 
/// assert_eq!(to_str(Direction::Up), "Up");
/// assert_eq!(to_str(Direction::Right), "Right");
/// assert_eq!(to_str(Direction::Down), "Down");
/// assert_eq!(to_str(Direction::Left), "Left");
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

/// Convert a Direction to a String
/// 
/// ```
/// use search_and_destroy_chess_2::direction::Direction;
/// use search_and_destroy_chess_2::direction::to_str;
/// 
/// assert_eq!(to_str(Direction::Up), "Up");
/// assert_eq!(to_str(Direction::Right), "Right");
/// assert_eq!(to_str(Direction::Down), "Down");
/// assert_eq!(to_str(Direction::Left), "Left");
/// ```
#[allow(dead_code)]
pub fn to_str(direction: Direction) -> String {
    match direction {
        Direction::Up => "Up".to_string(),
        Direction::Right => "Right".to_string(),
        Direction::Down => "Down".to_string(),
        Direction::Left => "Left".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_str() {
        assert_eq!(to_str(Direction::Up), String::from("Up"));
        assert_eq!(to_str(Direction::Right), String::from("Right"));
        assert_eq!(to_str(Direction::Down), String::from("Down"));
        assert_eq!(to_str(Direction::Left), String::from("Left"));
    }

}
