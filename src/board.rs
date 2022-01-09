use crate::color::Color;
use crate::piece::Piece;

#[allow(unused_imports)]
use crate::file_index::FileIndex;
use crate::piece_type::PieceType;
use crate::square::get_nth_file;
use crate::square::get_nth_rank;
use crate::square::Square;

/// The position of pieces on a board.
/// Does not include any temporary information, such as
/// the possibility of en-passant or castling.
#[allow(dead_code)]
pub struct Board {
    ranks: Vec<crate::rank::Rank>,
}

impl Board {

    /// Create a new Board
    /// 
    /// ```
    /// use search_and_destroy_chess_2::board::Board;
    /// use search_and_destroy_chess_2::file_index::FileIndex;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// 
    /// let board = Board::new();
    /// let piece = board.get_piece_from_indices(&FileIndex::new(0), 0).unwrap();
    /// assert_eq!(piece.get_type(), PieceType::Rook);
    /// ```
    #[allow(dead_code)]
    pub fn new() -> Board {

        Board {
            ranks: crate::rank::create_starting_ranks(),
        }
    }

    /// Get a piece based on its indices
    /// 
    /// ```
    /// use search_and_destroy_chess_2::board::Board;
    /// use search_and_destroy_chess_2::file_index::FileIndex;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// 
    /// let board = Board::new();
    /// let piece = board.get_piece_from_indices(&FileIndex::new(0), 0).unwrap();
    /// assert_eq!(piece.get_type(), PieceType::Rook);
    /// ```
    #[allow(dead_code)]
    pub fn get_piece_from_indices(&self, file_index: &FileIndex, rank_index: usize) -> Option<Piece> {
        crate::rank::get_piece(&self.ranks[rank_index], &file_index)
    }
    /// Get the ranks of a chessboard
    /// 
    /// ```
    /// use search_and_destroy_chess_2::board::Board;
    /// 
    /// let board = Board::new();
    /// let ranks = board.get_ranks();
    /// assert_eq!(ranks.len(), 8);
    /// ```
    pub fn get_ranks(&self) -> &Vec<crate::rank::Rank> {
        &self.ranks
    }
}

pub fn get_invisible_squares(board: &Board, color: Color) -> Vec<Square> {
    let mut invisible_squares: Vec<Square> = vec![];
    for rank in board.get_ranks() {
        for file_index in crate::file_index::get_all_file_indices() {
            if let Some(piece) = crate::rank::get_piece(&rank, &file_index) {
                if piece.get_color() == color {
                invisible_squares.push(
                    crate::square::Square::new(
                        &crate::square::create_coordinat_from_indices(&file_index, 1 as u8)
                    )
                );
            }
        }
      }
  }
  invisible_squares
}

/// Get the color of a square
/// 
/// ```
/// use search_and_destroy_chess_2::board::get_square_color_from_square;
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let square = Square::new("d1");
/// let color = get_square_color_from_square(&square);
/// assert_eq!(color, Color::White);
/// ```
#[allow(dead_code)]
pub fn get_square_color_from_square(square: &Square) -> Color {
    let file_index = get_nth_file(square);
    let rank_index = get_nth_rank(square);
    get_square_color_from_indices(&file_index, rank_index)
}

/// Get the color of a square, based on its indices,
/// i.e. square a1 is `(0, 0)`, d1 is `(3, 0)`
/// 
/// ```
/// use search_and_destroy_chess_2::board::get_square_color_from_indices;
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::file_index::FileIndex;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let square = Square::new("d1");
/// let color = get_square_color_from_indices(&FileIndex::new(3), 0);
/// assert_eq!(color, Color::White);
/// ```
pub fn get_square_color_from_indices(file_index: &crate::file_index::FileIndex, rank_index: usize) -> Color {
    let file_bit = file_index.get() % 2;
    let rank_bit = rank_index % 2;
    let bit_int = (rank_bit + file_bit) % 2;
    assert!(bit_int == 0 || bit_int == 1);
    let bit = bit_int == 0;
    match bit {
        false => Color::White,
        true => Color::Black,
    }
}

/// Detect if a square on the board contains a pawn
/// 
/// ```
/// use search_and_destroy_chess_2::board::Board;
/// use search_and_destroy_chess_2::board::is_pawn;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let board = Board::new();
/// assert_eq!(true, is_pawn(&board, Square::new("a2")));
/// assert_eq!(false, is_pawn(&board, Square::new("a1")));
/// ```
#[allow(dead_code)]
pub fn is_pawn(board: &Board, square: Square) -> bool {
    let piece_option = board.get_piece_from_indices(&get_nth_file(&square), get_nth_rank(&square) as usize);
    match piece_option {
        Some(piece) => piece.get_type() == PieceType::Pawn,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board() {
        let board = Board::new();
        assert_eq!(true, is_pawn(&board, Square::new("a2")));
        assert_eq!(false, is_pawn(&board, Square::new("a1")));
    }
    #[test]
    fn test_get_square_color_from_square() {
        assert_eq!(
            get_square_color_from_square(&Square::new("a1")),
            Color::Black
        );
        assert_eq!(
            get_square_color_from_square(&Square::new("b1")),
            Color::White
        );
    }
    #[test]
    fn test_get_square_color_from_indices() {
        assert_eq!(get_square_color_from_indices(&FileIndex::new(0), 0), Color::Black);
        assert_eq!(get_square_color_from_indices(&FileIndex::new(0), 1), Color::White);
    }
    #[test]
    fn get_piece_from_indices() {
        let board = Board::new();
        assert_eq!(board.get_piece_from_indices(&FileIndex::new(0), 0).unwrap().get_color(), Color::White);
        assert_eq!(board.get_piece_from_indices(&FileIndex::new(0), 0).unwrap().get_type(), PieceType::Rook);
        assert_eq!(board.get_piece_from_indices(&FileIndex::new(1), 1).unwrap().get_color(), Color::White);
        assert_eq!(board.get_piece_from_indices(&FileIndex::new(1), 1).unwrap().get_type(), PieceType::Pawn);
    }
}
