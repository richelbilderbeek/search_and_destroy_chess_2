use crate::piece::Piece;
use crate::square::Square;

/// The position of pieces on a board.
/// Does not include any temporary information, such as
/// the possibility of en-passant or castling.
pub struct Board {
    pieces: Vec<Piece> ,
}

impl Board {
    pub fn new() -> Board {
        Board{ pieces: Vec::new() }
    }
}

pub fn is_pawn(_board: Board, _square: Square) -> bool {
  true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board() {
        let board = Board::new();
        assert_eq!(true, is_pawn(board, Square::new("a2")));
    }

}
