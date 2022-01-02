use crate::color::Color;
use crate::piece::create_starting_pieces;
use crate::piece::Piece;
use crate::piece_type::PieceType;
use crate::square::get_nth_file;
use crate::square::get_nth_rank;
use crate::square::Square;

/// The position of pieces on a board.
/// Does not include any temporary information, such as
/// the possibility of en-passant or castling.
pub struct Board {
    pieces: Vec<Piece>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: create_starting_pieces(),
        }
    }
    pub fn get_piece_from_indices(&self, rank: u8, file: u8) -> Option<Piece> {
        Some(self.pieces[0].clone())
    }
}

pub fn get_square_color_from_square(square: &Square) -> Color {
    let file_index = get_nth_file(square);
    let rank_index = get_nth_rank(square);
    get_square_color_from_indices(file_index, rank_index)
}

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
    }
}
