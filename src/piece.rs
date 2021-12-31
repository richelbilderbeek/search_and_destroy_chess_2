use crate::color::Color;
use crate::piece_type::PieceType;
use crate::square::Square;

/// A chess piece
pub struct Piece {
    color: Color,
    position: Square,
    r#type: PieceType,
}

pub fn create_white_pawn(position: &str) -> Piece {
    Piece{
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::Pawn
    }
}

pub fn create_starting_pieces() -> Vec<Piece> {
    let pieces: Vec<Piece> = vec![
        create_white_pawn("a2"),
        create_white_pawn("b2"),
    ];
    pieces
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
    fn create_a3_pawn() {
        let piece = create_white_pawn("b2");
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.position, Square::new("b2"));
        assert_eq!(piece.r#type, PieceType::Pawn);
    }

    #[test]
    fn starting_pieces() {
        let pieces = create_starting_pieces();
        assert_ne!(pieces.len(), 0);
    }

}
