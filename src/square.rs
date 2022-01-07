/// A square on a chess board
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Square {
    coordinat: String,
}

impl Square {
    /// ```
    /// use search_and_destroy_chess_2::square::Square;
    /// 
    /// let square = Square::new("d1");
    /// let square_again = square.clone();
    /// assert_eq!(square, square_again);
    /// ```
    pub fn clone(&self) -> Square {
        Square {
            coordinat: String::from(self.coordinat.clone()),
        }
    }
    /// Create a new square from a string, e.g. 'a1'
    /// ```
    /// use search_and_destroy_chess_2::square::Square;
    /// 
    /// let square = Square::new("d1");
    /// let square_again = Square::new("d1");
    /// assert_eq!(square, square_again);
    /// ```
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
    /// Create a new square from a string, e.g. 'a1'
    /// ```
    /// use search_and_destroy_chess_2::square::Square;
    /// 
    /// let square = Square::new("d1");
    /// assert_eq!(square.get(), "d1");
    /// ```
    pub fn get(&self) -> String {
        self.coordinat.clone()
    }
    /// Get the file of the Square, e.g. 'a'
    /// ```
    /// use search_and_destroy_chess_2::square::Square;
    /// 
    /// let square = Square::new("e1");
    /// assert_eq!(square.get_file(), "e");
    /// ```
    pub fn get_file(&self) -> String {
        let copy: String = self.coordinat.clone();
        copy.chars().next().unwrap().to_string()
    }
    /// Get the rank of the Square, e.g. '1'
    /// ```
    /// use search_and_destroy_chess_2::square::Square;
    /// 
    /// let square = Square::new("f7");
    /// assert_eq!(square.get_rank(), 7);
    /// ```
    pub fn get_rank(&self) -> u32 {
        let copy: String = self.coordinat.clone();
        let slice = &copy[1..2];
        let first_str = String::from(slice);
        let rank: u32 = first_str.parse().unwrap();
        rank
    }
}

/// Create a coordinat string from two indices
/// ```
/// assert_eq!(create_coordinat_from_indices(0, 0), String::from("a1"));
/// assert_eq!(create_coordinat_from_indices(7, 0), String::from("h1"));
/// assert_eq!(create_coordinat_from_indices(7, 4), String::from("h4"));
/// assert_eq!(create_coordinat_from_indices(7, 7), String::from("h7"));
/// ```
pub fn create_coordinat_from_indices(file_index: u8, rank_index: u8) -> String {
    let file_str = match file_index {
        0 => String::from("a"),
        1 => String::from("b"),
        2 => String::from("c"),
        3 => String::from("d"),
        4 => String::from("e"),
        5 => String::from("f"),
        6 => String::from("g"),
        7 => String::from("h"),
        _ => panic!("Use a file_index from [0..8]")
    };
    let rank_str = match rank_index {
        0 => String::from("1"),
        1 => String::from("2"),
        2 => String::from("3"),
        3 => String::from("4"),
        4 => String::from("5"),
        5 => String::from("6"),
        6 => String::from("7"),
        7 => String::from("8"),
        _ => panic!("Use a rank_index from [0..8]")
    };
    let str = file_str + &rank_str;
    str
}

/// Get all the 64 coordinats on a chessboard
/// ```
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let coordinats = get_all_coordinats()
/// assert_eq!(coordinats.len(), 64);
/// ```
pub fn get_all_coordinats() -> Vec<String> {
    let mut squares: Vec<String> = Vec::new();
    for file_index in 0..8 {
        for rank_index in 0..8 {
            squares.push(create_coordinat_from_indices(file_index, rank_index));
        }
    }
    squares
}

/// Get all the 64 squares on a chessboard
/// ```
/// use search_and_destroy_chess_2::square::Square;
/// use search_and_destroy_chess_2::square::get_nth_rank;
/// 
/// let squares = get_all_squares()
/// assert_eq!(squares.len(), 64);
/// ```
pub fn get_all_squares() -> Vec<Square> {
    let coordinats = get_all_coordinats();
    let mut squares: Vec<Square> = Vec::new();
    for coordinat in coordinats {
        squares.push(Square::new(&coordinat));
    }
    squares
}

/// Get the nth file of the Square, e.g. '0' for the first file.
/// Or, the letter compared to 'a'
/// 
/// A file is a column of the chessboard. 
/// 
/// ```
/// use search_and_destroy_chess_2::square::Square;
/// use search_and_destroy_chess_2::square::get_nth_file;
/// 
/// let square = Square::new("a1");
/// assert_eq!(get_nth_file(&square), 0);
/// let square = Square::new("h1");
/// assert_eq!(get_nth_file(&square), 7);
/// ```
pub fn get_nth_file(square: &Square) -> u32 {
let char_vec: Vec<char> = square.get_file().chars().collect();
(char_vec[0] as i32 - 'a' as i32) as u32
}

/// Get the nth rank of the Square, e.g. '0' for the first rank
/// Or, the number compared to 1
/// ```
/// use search_and_destroy_chess_2::square::Square;
/// use search_and_destroy_chess_2::square::get_nth_rank;
/// 
/// let square = Square::new("a1");
/// assert_eq!(get_nth_rank(&square), 0);
/// let square = Square::new("a8");
/// assert_eq!(get_nth_rank(&square), 7);
/// ```
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
    #[test]
    fn get_all_squares() {
        assert_eq!(super::get_all_squares().len(), 64);
    }
}
