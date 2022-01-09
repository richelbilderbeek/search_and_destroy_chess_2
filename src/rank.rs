use crate::piece::Piece;
use crate::file_index::FileIndex;

/// A rank, a row on a chessboard
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rank {
    pieces: Vec<Option<Piece>>,
}

impl Rank {

    /// Create a rank (i.e. a row) of a chess board
    /// 
    /// ```
    /// use search_and_destroy_chess_2::file_index::FileIndex;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// use search_and_destroy_chess_2::rank::Rank;
    /// use search_and_destroy_chess_2::rank::create_pieces_from_rank_index;
    /// use search_and_destroy_chess_2::rank::get_piece;
    /// 
    /// let rank = Rank::new(create_pieces_from_rank_index(3));
    /// let piece = get_piece(&rank, &FileIndex::new(0));
    /// assert_eq!(piece, None);
    /// ```
    #[allow(dead_code)]
    pub fn new(pieces: Vec<Option<Piece>>) -> Rank {
        assert_eq!(pieces.len(), 8);

        Rank {
            pieces,
        }
    }

    /*
    /// Create a rank (i.e. a row) of a chess board
    /// 
    /// ```
    /// use search_and_destroy_chess_2::board::Rank;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// 
    /// let rank = Rank::new();
    /// let piece = board.get_piece_from_indices(0);
    /// assert_eq!(piece, None);
    /// ```
    #[allow(dead_code)]
    pub fn new_from_rank_index(rank_index: u8) -> Rank {
        assert!(rank_index >= 0);
        assert!(rank_index <= 7);
        Rank {
            pieces: create_pieces_from_rank_index(rank_index),
        }
    }
    */

    /// Get the pieces
    /// ```
    /// use search_and_destroy_chess_2::rank::Rank;
    /// use search_and_destroy_chess_2::rank::create_pieces_from_rank_index;
    /// use search_and_destroy_chess_2::rank::get_piece;
    /// use search_and_destroy_chess_2::file_index::FileIndex;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// 
    /// let rank = Rank::new(create_pieces_from_rank_index(3));
    /// let piece = get_piece(&rank, &FileIndex::new(0));
    /// assert_eq!(piece, None);
    /// ```
    pub fn get(&self) -> &Vec<Option<Piece>> { &self.pieces }
}

pub fn get_piece(rank: &Rank, file_index: &FileIndex) -> Option<Piece> {
    rank.get()[file_index.get()].clone()
}

pub fn create_pieces_from_rank_index(rank_index: u8) -> Vec<Option<Piece>> {
    assert!(rank_index <= 7);
    
    use crate::piece::*;
    match rank_index {
        0 => vec![
            Some(create_white_rook()),
            Some(create_white_knight()),
            Some(create_white_bishop()),
            Some(create_white_queen()),
            Some(create_white_king()),
            Some(create_white_bishop()),
            Some(create_white_knight()),
            Some(create_white_rook()),
        ],
        1 => vec![Some(create_white_pawn()); 8],
        6 => vec![Some(create_black_pawn()); 8],
        7 => vec![
            Some(create_black_rook()),
            Some(create_black_knight()),
            Some(create_black_bishop()),
            Some(create_black_queen()),
            Some(create_black_king()),
            Some(create_black_bishop()),
            Some(create_black_knight()),
            Some(create_black_rook()),
        ],
        _ => vec![None; 8],
    }
}

pub fn create_starting_ranks() -> Vec<Rank> {
    vec![
        Rank::new(create_pieces_from_rank_index(0)),
        Rank::new(create_pieces_from_rank_index(1)),
        Rank::new(create_pieces_from_rank_index(2)),
        Rank::new(create_pieces_from_rank_index(3)),
        Rank::new(create_pieces_from_rank_index(4)),
        Rank::new(create_pieces_from_rank_index(5)),
        Rank::new(create_pieces_from_rank_index(6)),
        Rank::new(create_pieces_from_rank_index(7)),
    ]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board() {
        let rank = Rank::new(vec![None; 8]);
        assert_eq!(rank.get().len(), 8);
    }
}