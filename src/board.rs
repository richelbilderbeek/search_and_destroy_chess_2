use crate::color::Color;
use crate::piece::create_starting_pieces;
use crate::piece::Piece;

#[allow(unused_imports)]
use crate::piece_type::PieceType;
use crate::square::get_nth_file;
use crate::square::get_nth_rank;
use crate::square::Square;

/// The position of pieces on a board.
/// Does not include any temporary information, such as
/// the possibility of en-passant or castling.
#[allow(dead_code)]
pub struct Board {
    pieces: Vec<Piece>,
}

impl Board {

    /// Create a new Board
    /// 
    /// ```
    /// use search_and_destroy_chess_2::board::Board;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// 
    /// let board = Board::new();
    /// let piece = board.get_piece_from_indices(0, 0).unwrap();
    /// assert_eq!(piece.get_type(), PieceType::Rook);
    /// ```
    #[allow(dead_code)]
    pub fn new() -> Board {
        Board {
            pieces: create_starting_pieces(),
        }
    }

    /// Get a piece based on its indices
    /// 
    /// ```
    /// use search_and_destroy_chess_2::board::Board;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// 
    /// let board = Board::new();
    /// let piece = board.get_piece_from_indices(0, 0).unwrap();
    /// assert_eq!(piece.get_type(), PieceType::Rook);
    /// ```
    #[allow(dead_code)]
    pub fn get_piece_from_indices(&self, rank: u8, file: u8) -> Option<Piece> {
        for piece in &self.pieces {
            if get_nth_rank(&piece.get_position()) == rank.into()
              && get_nth_file(&piece.get_position()) == file.into() {
                return Some(piece.clone())
            }
        }
        None
    }
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
    get_square_color_from_indices(file_index, rank_index)
}

/// Get the color of a square, based on its indices,
/// i.e. square a1 is `(0, 0)`, d1 is `(3, 0)`
/// 
/// ```
/// use search_and_destroy_chess_2::board::get_square_color_from_indices;
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let square = Square::new("d1");
/// let color = get_square_color_from_indices(3, 0);
/// assert_eq!(color, Color::White);
/// ```
pub fn get_square_color_from_indices(file_index: u32, rank_index: u32) -> Color {
    let file_bit = file_index % 2;
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
    let piece = board.get_piece_from_indices(get_nth_rank(&square) as u8, get_nth_file(&square) as u8).unwrap();
    piece.get_type() == PieceType::Pawn
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
    fn get_square_color_from_square() {
        assert_eq!(
            super::get_square_color_from_square(&Square::new("a1")),
            Color::Black
        );
        assert_eq!(
            super::get_square_color_from_square(&Square::new("b1")),
            Color::White
        );
    }
    #[test]
    fn get_square_color_from_indices() {
        assert_eq!(super::get_square_color_from_indices(0, 0), Color::Black);
        assert_eq!(super::get_square_color_from_indices(0, 1), Color::White);
    }
    #[test]
    fn get_piece_from_indices() {
        let board = Board::new();
        assert_eq!(board.get_piece_from_indices(0, 0).unwrap().get_color(), Color::White);
        assert_eq!(board.get_piece_from_indices(0, 0).unwrap().get_type(), PieceType::Rook);
        assert_eq!(board.get_piece_from_indices(1, 1).unwrap().get_color(), Color::White);
        assert_eq!(board.get_piece_from_indices(1, 1).unwrap().get_type(), PieceType::Pawn);
    }
}
