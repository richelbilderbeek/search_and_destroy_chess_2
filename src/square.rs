/// A square on a chess board
#[derive(Debug, PartialEq, Eq)]
pub struct Square {
    coordinat: String,
}

impl Square {
    /// Create a new square from a string, e.g. 'a1'
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
        Square {
            coordinat: String::from(coordinat_str),
        }
    }
    /// Get the coordinat of the Square, e.g. 'a1'
    pub fn get(&self) -> String {
        self.coordinat.clone()
    }
    /// Get the file of the Square, e.g. 'a'
    pub fn get_file(&self) -> String {
        let copy: String = self.coordinat.clone();
        copy.chars().next().unwrap().to_string()
    }
    /// Get the rank of the Square, e.g. '1'
    pub fn get_rank(&self) -> u32 {
        let copy: String = self.coordinat.clone();
        let slice = &copy[1..2];
        let first_str = String::from(slice);
        let rank: u32 = first_str.parse().unwrap();
        rank
    }
}

/// Get the nth file of the Square, e.g. '0' for the first file.
/// Or, the letter compared to 'a'
pub fn get_nth_file(square: &Square) -> u32 {
    let char_vec: Vec<char> = square.get_file().chars().collect();
    (char_vec[0] as i32 - 'a' as i32) as u32
}

/// Get the nth rank of the Square, e.g. '0' for the first rank
/// Or, the number compared to 1
pub fn get_nth_rank(square: &Square) -> u32 {
    square.get_rank() - 1_u32
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
    fn get_nth_file() {
        // the letter compared to 'a'
        assert_eq!(super::get_nth_file(&Square::new("a1")), 0);
        assert_eq!(super::get_nth_file(&Square::new("a2")), 0);
        assert_eq!(super::get_nth_file(&Square::new("b1")), 1);
    }

    #[test]
    fn get_nth_rank() {
        // the number minus 1
        assert_eq!(super::get_nth_rank(&Square::new("a1")), 0);
        assert_eq!(super::get_nth_rank(&Square::new("b1")), 0);
        assert_eq!(super::get_nth_rank(&Square::new("a2")), 1);
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
