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
#[derive(Debug, PartialEq, Eq, Clone)]
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
    for rank_index in 1..8 {
        let rank = &board.get_ranks()[rank_index];
        for file_index in crate::file_index::get_all_file_indices() {
            if let Some(piece) = crate::rank::get_piece(&rank, &file_index) {
                if piece.get_color() == color {
                invisible_squares.push(
                    crate::square::Square::new(
                        &crate::square::create_coordinat_from_indices(&file_index, rank_index as u8)
                    )
                );
            }
        }
      }
  }
  invisible_squares
}

pub fn get_piece_at_square(board: &Board, square: &crate::square::Square)  -> Option<crate::piece::Piece> {
    board.get_piece_from_indices(&crate::square::get_nth_file(square), crate::square::get_nth_rank(square))
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

/// Get the squares when going bottom-left, excluding the target square
pub fn get_squares_going_bottom_left(square: &crate::square::Square) -> Vec<crate::square::Square> {
    let mut squares = Vec::new();
    /*
    let mut there = square.clone();
    if get_nth_file(&there).get() == 0 { return squares; }
    if get_nth_rank(&there) == 7 { return squares; }
    loop {
          there = crate::square::get_square_above_left(there);
          squares.push(there);
          if get_nth_file(&there).get() == 0 { return squares; }
          if get_nth_rank(&there) == 7 { return squares; }
    }
    */
    squares
}

/// Get the squares when going bottom-right, excluding the target square
pub fn get_squares_going_bottom_right(square: &crate::square::Square) -> Vec<crate::square::Square> {
    let mut squares = Vec::new();
    /*
    let mut there = square.clone();
    if get_nth_file(&there).get() == 0 { return squares; }
    if get_nth_rank(&there) == 7 { return squares; }
    loop {
          there = crate::square::get_square_above_left(there);
          squares.push(there);
          if get_nth_file(&there).get() == 0 { return squares; }
          if get_nth_rank(&there) == 7 { return squares; }
    }
    */
    squares
}

/// Get the squares when going top-left, excluding the target square
/// ```
/// using search_and_destroy_chess::board::get_squares_going_top_left;
/// using search_and_destroy_chess::square::Square;
/// 
/// // From the top-left square, one cannot go towards the top-left
/// let squares = get_squares_going_top_left(Square::new("a8"));
/// assert_eq!(squares.len(), 0)
/// 
/// // From a top square, one cannot go towards the top-left
/// let squares = get_squares_going_top_left(Square::new("d8"));
/// assert_eq!(squares.len(), 0)
/// 
/// // From a left square, one cannot go towards the top-left
/// let squares = get_squares_going_top_left(Square::new("a4"));
/// assert_eq!(squares.len(), 0)
/// 
/// // b7 -> a8
/// let squares = get_squares_going_top_left(Square::new("b7"));
/// assert_eq!(squares.len(), 1)
/// 
/// // c5 -> b6 -> a7
/// let squares = get_squares_going_top_left(Square::new("c5"));
/// assert_eq!(squares.len(), 2)
/// 
/// // d3 -> c4 -> b5 -> a4
/// let squares = get_squares_going_top_left(Square::new("d3"));
/// assert_eq!(squares.len(), 4)
/// 
/// // h1 -> g2 -> f3 -> e4 -> d5 -> c6 -> b7 -> a8
/// let squares = get_squares_going_top_left(Square::new("h1"));
/// assert_eq!(squares.len(), 8)
/// ```
pub fn get_squares_going_top_left(square: &crate::square::Square) -> Vec<crate::square::Square> {
  let mut there = square.clone();
  let mut squares = Vec::new();
  if get_nth_file(&there).get() == 0 { return squares; }
  if get_nth_rank(&there) == 7 { return squares; }
  loop {
        there = crate::square::get_square_above_left(there);
        squares.push(there);
        if get_nth_file(&there).get() == 0 { return squares; }
        if get_nth_rank(&there) == 7 { return squares; }
    }
}

/// Get the squares when going top-right, excluding the target square
pub fn get_squares_going_top_right(square: &crate::square::Square) -> Vec<crate::square::Square> {
    let mut squares = Vec::new();
    /*
    let mut there = square.clone();
    if get_nth_file(&there).get() == 0 { return squares; }
    if get_nth_rank(&there) == 7 { return squares; }
    loop {
          there = crate::square::get_square_above_left(there);
          squares.push(there);
          if get_nth_file(&there).get() == 0 { return squares; }
          if get_nth_rank(&there) == 7 { return squares; }
    }
    */
    squares
}
  

/// Get the target squares for the piece at the square, using only the knowledge of the current board,
/// i.e. this excludes en passent (requires knowledge of the previous move) and castling (requires
/// knowledge of the king and rook's past movements)
/// 
/// Assumes there is a piece at the square
pub fn get_target_squares(board: &Board, square: crate::square::Square) -> Vec<crate::square::Square> {

    let piece_option = get_piece_at_square(&board, &square);
    if piece_option == None { panic!("There must be a piece at the square") }
    let piece = piece_option.unwrap();

    // Basic movements that are possible on a board
    let squares = match piece.get_type() {
        crate::piece_type::PieceType::Bishop => get_target_squares_for_bishop(&board, square),
        crate::piece_type::PieceType::King => get_target_squares_for_king(&board, &square),
        crate::piece_type::PieceType::Knight => get_target_squares_for_knight(&board, &square),
        crate::piece_type::PieceType::Pawn => get_target_squares_for_pawn(&board, &square),
        crate::piece_type::PieceType::Queen => get_target_squares_for_queen(&board, &square),
        crate::piece_type::PieceType::Rook => get_target_squares_for_rook(&board, &square),
    };

    squares
}

/// Get the target squares for the piece at the square, using only the knowledge of the current board,
/// i.e. this excludes en passent (requires knowledge of the previous move) and castling (requires
/// knowledge of the king and rook's past movements)
/// 
/// Assumes there is a piece at the square
pub fn get_target_squares_for_bishop(board: &Board, square: crate::square::Square) -> Vec<crate::square::Square> {

    let piece_option = get_piece_at_square(&board, &square);
    assert_ne!(piece_option, None);
    let piece = piece_option.unwrap();
    assert_eq!(piece.get_type(), crate::piece_type::PieceType::Bishop);
    let color = piece.get_color();
    let mut squares = Vec::new();
    let mut squares_top_left = get_squares_going_top_left(&square);
    let mut squares_top_right = get_squares_going_top_right(&square);
    let mut squares_bottom_left = get_squares_going_bottom_left(&square);
    let mut squares_bottom_right = get_squares_going_bottom_right(&square);
    squares.append(&mut squares_top_left);
    squares.append(&mut squares_top_right);
    squares.append(&mut squares_bottom_left);
    squares.append(&mut squares_bottom_right);
    squares
}

pub fn get_target_squares_for_king(board: &Board, square: crate::square::Square) -> Vec<crate::square::Square> {
    Vec::new()
}

pub fn get_target_squares_for_knight(board: &Board, square: crate::square::Square) -> Vec<crate::square::Square> {
    Vec::new()
}

pub fn get_target_squares_for_pawn(board: &Board, square: crate::square::Square) -> Vec<crate::square::Square> {
    Vec::new()
}

pub fn get_target_squares_for_queen(board: &Board, square: crate::square::Square) -> Vec<crate::square::Square> {
    Vec::new()
}

pub fn get_target_squares_for_rook(board: &Board, square: crate::square::Square) -> Vec<crate::square::Square> {
    Vec::new()
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
