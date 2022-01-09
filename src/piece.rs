use crate::color::Color;
use crate::piece_type::PieceType;

/// A chess piece
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Piece {
    color: Color,
    r#type: PieceType,
}

impl Piece {
    /// Clone a chesspiece
    /// ```
    /// use search_and_destroy_chess_2::piece::Piece;
    /// use search_and_destroy_chess_2::piece::create_black_bishop;
    /// 
    /// let piece = create_black_bishop();
    /// let piece_again = piece.clone();
    /// 
    /// assert_eq!(piece, piece_again);
    /// ```
    pub fn clone(&self) -> Piece {
        Piece {
            color: self.color,
            r#type: self.r#type,
        }
    }
    /// ```
    /// use search_and_destroy_chess_2::color::Color;
    /// use search_and_destroy_chess_2::piece::Piece;
    /// use search_and_destroy_chess_2::piece::create_black_bishop;
    /// 
    /// let piece = create_black_bishop();
    /// assert_eq!(piece.get_color(), Color::Black);
    /// ```
    #[allow(dead_code)]
    pub fn get_color(&self) -> Color {
        self.color
    }
    /// ```
    /// use search_and_destroy_chess_2::piece::Piece;
    /// use search_and_destroy_chess_2::piece::create_black_bishop;
    /// use search_and_destroy_chess_2::piece_type::PieceType;
    /// 
    /// let piece = create_black_bishop();
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
/// let piece = create_black_bishop();
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Bishop);
/// ```
pub fn create_black_bishop() -> Piece {
    Piece {
        color: Color::Black,
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
/// let piece = create_black_king();
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::King);
/// ```
pub fn create_black_king() -> Piece {
    Piece {
        color: Color::Black,
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
/// let piece = create_black_knight();
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Knight);
/// ```
pub fn create_black_knight() -> Piece {
    Piece {
        color: Color::Black,
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
/// let piece = create_black_pawn();
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Pawn);
/// ```
pub fn create_black_pawn() -> Piece {
    Piece {
        color: Color::Black,
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
/// let piece = create_black_queen();
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Queen);
/// ```
pub fn create_black_queen() -> Piece {
    Piece {
        color: Color::Black,
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
/// let piece = create_black_rook();
/// assert_eq!(piece.get_color(), Color::Black);
/// assert_eq!(piece.get_type(), PieceType::Rook);
/// ```
pub fn create_black_rook() -> Piece {
    Piece {
        color: Color::Black,
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
    vec![create_black_pawn(); 8]
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
/// let piece = create_white_bishop();
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Bishop);
/// ```
pub fn create_white_bishop() -> Piece {
    Piece {
        color: Color::White,
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
/// let piece = create_white_king();
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::King);
/// ```
pub fn create_white_king() -> Piece {
    Piece {
        color: Color::White,
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
/// let piece = create_white_knight();
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Knight);
/// ```
pub fn create_white_knight() -> Piece {
    Piece {
        color: Color::White,
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
/// let piece = create_white_pawn();
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Pawn);
/// ```
pub fn create_white_pawn() -> Piece {
    Piece {
        color: Color::White,
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
/// let piece = create_white_queen();
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Queen);
/// ```
pub fn create_white_queen() -> Piece {
    Piece {
        color: Color::White,
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
/// 
/// let piece = create_white_rook();
/// assert_eq!(piece.get_color(), Color::White);
/// assert_eq!(piece.get_type(), PieceType::Rook);
/// ```
pub fn create_white_rook() -> Piece {
    Piece {
        color: Color::White,
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
    vec![create_white_pawn(); 8]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a2_pawn() {
        let piece = Piece {
            color: Color::White,
            r#type: PieceType::Pawn,
        };
        assert_eq!(piece.color, Color::White);
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
    fn create_black_rook() {
        let piece = super::create_black_rook();
        assert_eq!(piece.color, Color::Black);
        assert_eq!(piece.r#type, PieceType::Rook);
    }
    #[test]
    fn create_white_rook() {
        let piece = super::create_white_rook();
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.r#type, PieceType::Rook);
    }
}
