// Must be called 'lib.rs' to test

/// A square on a chess board
pub struct Square {
  coordinat: String
}

impl Square {
    pub fn new(coordinat_str: &str) -> Square {
        Square{
          coordinat: String::from(coordinat_str)
        }
    }
}

/// The position of pieces on a board.
/// Does not include any temporary information, such as
/// the possibility of en-passant or castling.
pub struct Board {

}

impl Board {
    pub fn new() -> Board {
        Board{}
    }
}

pub fn is_pawn(_board: Board, _square: Square) -> bool {
  true
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
    fn create_board() {
        let board = Board::new();
        assert_eq!(true, is_pawn(board, Square::new("A2")));
        assert_eq!(1 + 1, 2);
    }
}
