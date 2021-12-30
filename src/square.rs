pub mod square {

/// A square on a chess board
pub struct Square {
  coordinat: String
}

impl Square {
    pub fn new(coordinat_str: &str) -> Square {
        if coordinat_str.len() != 2 {
            panic!("A coordinat has two characters")
        }
        Square{
          coordinat: String::from(coordinat_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_square() {
        let str = "A2";
        let square = Square::new(str);
        assert_eq!(str, square.coordinat);
    }

    #[test]
    #[should_panic(expected = "A coordinat has two characters")]
    fn invalid_square() {
        Square::new("X");
    }
}


} //~ pub mod square
