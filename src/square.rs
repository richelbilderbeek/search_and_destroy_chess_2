/// A square on a chess board
pub struct Square {
  coordinat: String
}

impl Square {
    pub fn new(coordinat_str: &str) -> Square {
        if coordinat_str.len() != 2 {
            panic!("A coordinat has two characters");
        }
        let file_regex = regex::Regex::new(r"^[a-h].$").unwrap();
        if !file_regex.is_match(coordinat_str) {
            panic!("The file must be 'a..h'");
        }
        let rank_regex = regex::Regex::new(r"^.[1-8]$").unwrap();
        if !rank_regex.is_match(coordinat_str) {
            panic!("The rank must be '1..8'");
        }
        Square{
          coordinat: String::from(coordinat_str)
        }
    }
    pub fn get(&self) -> String {
      String::from(self.coordinat.clone())
    }
    pub fn get_file(&self) -> String {
        let copy: String = String::from(self.coordinat.clone());
        copy.chars().nth(0).unwrap().to_string()
    }
    pub fn get_rank(&self) -> u32 {
        let copy: String = String::from(self.coordinat.clone());
        let slice = &copy[1..2];
        let first_str = String::from(slice);
        let rank: u32 = first_str.parse().unwrap();
        rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_square() {
        let str = "a2";
        let square = Square::new(str);
        assert_eq!(str, square.coordinat);
    }

    #[test]
    fn get() {
        let str = "h3";
        let square = Square::new(str);
        assert_eq!(str, square.get());
    }

    #[test]
    fn get_file() {
        let str = "b8";
        let square = Square::new(str);
        assert_eq!("b", square.get_file());
    }

    #[test]
    fn get_rank() {
        let str = "c4";
        let square = Square::new(str);
        assert_eq!(4, square.get_rank());
    }

    #[test]
    #[should_panic(expected = "A coordinat has two characters")]
    fn invalid_square() {
        Square::new("X");
    }
    #[test]
    #[should_panic(expected = "The file must be 'a..h'")]
    fn invalid_file() {
        Square::new("X1");
    }
    #[test]
    #[should_panic(expected = "The rank must be '1..8'")]
    fn invalid_rank() {
        Square::new("a9");
    }
}
