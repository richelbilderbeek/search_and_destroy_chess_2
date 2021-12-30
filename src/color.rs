/// A square on a chess board
pub enum Color {
  Black,
  White
}

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
