use crate::color::Color;
use crate::piece_type::PieceType;
use crate::square::Square;

/// A chess piece
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Piece {
    color: Color,
    position: Square,
    r#type: PieceType,
}

impl Piece {
    /// Clone a chesspiece
    /// ```
    /// use search_and_destroy_chess_2::piece::Piece;
    /// use search_and_destroy_chess_2::piece::create_black_bishop;
    /// 
    /// let piece = create_black_bishop("c8");
    /// let piece_again = piece.clone();
    /// 
    /// assert_eq!(piece, piece_again);
    /// ```
    pub fn clone(&self) -> Piece {
        Piece {
            color: self.color,
            position: self.position.clone(),
            r#type: self.r#type,
        }
    }
    /// ```
    /// use search_and_destroy_chess_2::color::Color;
    /// use search_and_destroy_chess_2::piece::Piece;
    /// use search_and_destroy_chess_2::piece::create_black_bishop;
    /// 
    /// let piece = create_black_bishop("c8");
    /// assert_eq!(piece.get_color(), Color::Black);
    /// ```
    #[allow(dead_code)]
    pub fn get_color(&self) -> Color {
        self.color
    }
    /// ```
    /// use search_and_destroy_chess_2::piece::Piece;
    /// use search_and_destroy_chess_2::piece::create_black_bishop;
    /// use search_and_destroy_chess_2::square::Square;
    /// 
    /// let piece = create_black_bishop("f8");
    /// assert_eq!(piece.get_position(), Square::new("f8"));
    /// ```
    pub fn get_position(&self) -> Square {
        Square::new(&self.position.get())
    }
    /// ```
    /// use search_and_destroy_chess_2::piece::Piece;
    /// use search_and_destroy_chess_2::piece::create_black_bishop;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// 
    /// let piece = create_black_bishop("f8");
    /// assert_eq!(piece.get_type(), PieceType::Bishop);
    /// ```
    #[allow(dead_code)]
    pub fn get_type(&self) -> PieceType {
        self.r#type
    }
}

/// Create a black bishop
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_black_bishop;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_black_bishop("c8");
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Bishop);
/// assert_eq!(piece.get_position(), Square::new("c8"));
/// ```
pub fn create_black_bishop(position: &str) -> Piece {
    Piece {
        color: Color::Black,
        position: Square::new(position),
        r#type: PieceType::Bishop,
    }
}

/// Create a black king
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_black_king;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_black_king("e8");
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::King);
/// assert_eq!(piece.get_position(), Square::new("e8"));
/// ```
pub fn create_black_king(position: &str) -> Piece {
    Piece {
        color: Color::Black,
        position: Square::new(position),
        r#type: PieceType::King,
    }
}

/// Create a black knight
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_black_knight;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_black_knight("b8");
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Knight);
/// assert_eq!(piece.get_position(), Square::new("b8"));
/// ```
pub fn create_black_knight(position: &str) -> Piece {
    Piece {
        color: Color::Black,
        position: Square::new(position),
        r#type: PieceType::Knight,
    }
}


/// Create a black pawn
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_black_pawn;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_black_pawn("a7");
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Pawn);
/// assert_eq!(piece.get_position(), Square::new("a7"));
/// ```
pub fn create_black_pawn(position: &str) -> Piece {
    Piece {
        color: Color::Black,
        position: Square::new(position),
        r#type: PieceType::Pawn,
    }
}

/// Create a black queen
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_black_queen;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_black_queen("d8");
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Queen);
/// assert_eq!(piece.get_position(), Square::new("d8"));
/// ```
pub fn create_black_queen(position: &str) -> Piece {
    Piece {
        color: Color::Black,
        position: Square::new(position),
        r#type: PieceType::Queen,
    }
}

/// Create a black rook
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_black_rook;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_black_rook("a8");
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Rook);
/// assert_eq!(piece.get_position(), Square::new("a8"));
/// ```
pub fn create_black_rook(position: &str) -> Piece {
    Piece {
        color: Color::Black,
        position: Square::new(position),
        r#type: PieceType::Rook,
    }
}

/// Create the pawns of black in their starting positions
/// 
/// ```
/// use search_and_destroy_chess_2::piece::create_black_starting_pawns;
/// let pieces = create_black_starting_pawns();
/// assert_eq!(8, pieces.len());
/// ```
pub fn create_black_starting_pawns() -> Vec<Piece> {
    vec![
        create_black_pawn("a7"),
        create_black_pawn("b7"),
        create_black_pawn("c7"),
        create_black_pawn("d7"),
        create_black_pawn("e7"),
        create_black_pawn("f7"),
        create_black_pawn("g7"),
        create_black_pawn("h7"),
    ]
}

/// Create a white bishop
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_white_bishop;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_white_bishop("c1");
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Bishop);
/// assert_eq!(piece.get_position(), Square::new("c1"));
/// ```
pub fn create_white_bishop(position: &str) -> Piece {
    Piece {
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::Bishop,
    }
}

/// Create a white king
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_white_king;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_white_king("e1");
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::King);
/// assert_eq!(piece.get_position(), Square::new("e1"));
/// ```
pub fn create_white_king(position: &str) -> Piece {
    Piece {
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::King,
    }
}

/// Create a white knight
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_white_knight;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_white_knight("b1");
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Knight);
/// assert_eq!(piece.get_position(), Square::new("b1"));
/// ```
pub fn create_white_knight(position: &str) -> Piece {
    Piece {
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::Knight,
    }
}

/// Create a white pawn
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_white_pawn;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_white_pawn("a2");
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Pawn);
/// assert_eq!(piece.get_position(), Square::new("a2"));
/// ```
pub fn create_white_pawn(position: &str) -> Piece {
    Piece {
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::Pawn,
    }
}

/// Create a white queen
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_white_queen;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_white_queen("d1");
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Queen);
/// assert_eq!(piece.get_position(), Square::new("d1"));
/// ```
pub fn create_white_queen(position: &str) -> Piece {
    Piece {
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::Queen,
    }
}

/// Create a white rook
/// 
/// ```
/// use search_and_destroy_chess_2::color::Color;
/// use search_and_destroy_chess_2::piece::Piece;
/// use search_and_destroy_chess_2::piece::create_white_rook;
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::square::Square;
/// 
/// let piece = create_white_rook("a1");
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Rook);
/// assert_eq!(piece.get_position(), Square::new("a1"));
/// ```
pub fn create_white_rook(position: &str) -> Piece {
    Piece {
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::Rook,
    }
}

/// Create the pawns of white in their starting positions
/// 
/// ```
/// use search_and_destroy_chess_2::piece::create_white_starting_pawns;
/// let pieces = create_white_starting_pawns();
/// assert_eq!(8, pieces.len());
/// ```
pub fn create_white_starting_pawns() -> Vec<Piece> {
    vec![
        create_white_pawn("a2"),
        create_white_pawn("b2"),
        create_white_pawn("c2"),
        create_white_pawn("d2"),
        create_white_pawn("e2"),
        create_white_pawn("f2"),
        create_white_pawn("g2"),
        create_white_pawn("h2"),
    ]
}

/// Create the starting pieces in their standard positions
/// 
/// ```
/// use search_and_destroy_chess_2::piece::create_starting_pieces;
/// let pieces = create_starting_pieces();
/// assert_eq!(32, pieces.len());
/// ```
pub fn create_starting_pieces() -> Vec<Piece> {
    let mut pieces: Vec<Piece> = vec![
        create_white_rook("a1"),
        create_white_knight("b1"),
        create_white_bishop("c1"),
        create_white_queen("d1"),
        create_white_king("e1"),
        create_white_bishop("f1"),
        create_white_knight("g1"),
        create_white_rook("h1"),
        create_black_rook("a8"),
        create_black_knight("b8"),
        create_black_bishop("c8"),
        create_black_queen("d8"),
        create_black_king("e8"),
        create_black_bishop("f8"),
        create_black_knight("g8"),
        create_black_rook("h8"),
    ];
    for pawn in create_white_starting_pawns() {
        pieces.push(pawn)
    }
    for pawn in create_black_starting_pawns() {
        pieces.push(pawn)
    }
    pieces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a2_pawn() {
        let piece = Piece {
            color: Color::White,
            position: Square::new("a2"),
            r#type: PieceType::Pawn,
        };
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.position, Square::new("a2"));
        assert_eq!(piece.r#type, PieceType::Pawn);
    }
    #[test]
    fn create_a3_pawn() {
        let piece = create_white_pawn("b2");
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.position, Square::new("b2"));
        assert_eq!(piece.r#type, PieceType::Pawn);
    }
    #[test]
    fn create_white_starting_pawns() {
        let pieces = super::create_white_starting_pawns();
        assert_eq!(pieces.len(), 8);
    }
    #[test]
    fn create_black_starting_pawns() {
        let pieces = super::create_black_starting_pawns();
        assert_eq!(pieces.len(), 8);
    }
    #[test]
    fn create_black_rook_a8() {
        let piece = super::create_black_rook("a8");
        assert_eq!(piece.color, Color::Black);
        assert_eq!(piece.position, Square::new("a8"));
        assert_eq!(piece.r#type, PieceType::Rook);
    }
    #[test]
    fn create_white_rook_a1() {
        let piece = super::create_white_rook("a1");
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.position, Square::new("a1"));
        assert_eq!(piece.r#type, PieceType::Rook);
    }

    #[test]
    fn starting_pieces() {
        let pieces = create_starting_pieces();
        assert_eq!(pieces.len(), 32);
    }
}
