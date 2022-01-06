/// A square on a chess board
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::color::to_str;
/// 
/// assert_eq!(to_str(Color::Black), "Black");
/// assert_eq!(to_str(Color::White), "White");
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

/// Convert a Color to a String
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::color::to_str;
/// 
/// assert_eq!(to_str(Color::Black), "Black");
/// assert_eq!(to_str(Color::White), "White");
/// ```
#[allow(dead_code)]
pub fn to_str(color: Color) -> String {
    match color {
        Color::Black => "Black".to_string(),
        Color::White => "White".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_to_str() {
        let str = to_str(Color::Black);
        assert_eq!(str, String::from("Black"));
    }

    #[test]
    fn white_to_str() {
        let str = to_str(Color::White);
        assert_eq!(str, String::from("White"));
    }
}
