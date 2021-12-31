use crate::color::Color;
use crate::piece_type::PieceType;
use crate::square::Square;

/// A chess piece
pub struct Piece {
    color: Color,
    position: Square,
    r#type: PieceType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a2_pawn() {
        let piece = Piece{
            color: Color::White,
            position: Square::new("a2"),
            r#type: PieceType::Pawn
        };
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.position, Square::new("a2"));
        assert_eq!(piece.r#type, PieceType::Pawn);
    }

}
