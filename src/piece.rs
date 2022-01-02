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
    pub fn clone(&self) -> Piece {
        Piece {
            color: self.color,
            position: self.position.clone(),
            r#type: self.r#type,
        }
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_position(&self) -> Square {
        Square::new(&self.position.get())
    }
    pub fn get_type(&self) -> PieceType {
        self.r#type
    }
}


pub fn create_black_pawn(position: &str) -> Piece {
    Piece {
        color: Color::Black,
        position: Square::new(position),
        r#type: PieceType::Pawn,
    }
}

pub fn create_black_rook(position: &str) -> Piece {
    Piece {
        color: Color::Black,
        position: Square::new(position),
        r#type: PieceType::Rook,
    }
}

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

pub fn create_white_pawn(position: &str) -> Piece {
    Piece {
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::Pawn,
    }
}

pub fn create_white_rook(position: &str) -> Piece {
    Piece {
        color: Color::White,
        position: Square::new(position),
        r#type: PieceType::Rook,
    }
}

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

pub fn create_starting_pieces() -> Vec<Piece> {
    let pieces: Vec<Piece> = vec![
        create_white_rook("a1"),
        create_white_rook("h1"),
        create_black_rook("a8"),
        create_black_rook("h8"),
    ];
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
    fn create_a3_pawn() {
        let piece = create_white_pawn("b2");
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.position, Square::new("b2"));
        assert_eq!(piece.r#type, PieceType::Pawn);
    }
    fn create_white_starting_pawns() {
        let pieces = super::create_white_starting_pawns();
        assert_eq!(pieces.len(), 8);
    }
    fn create_black_starting_pawns() {
        let pieces = super::create_black_starting_pawns();
        assert_eq!(pieces.len(), 8);
    }
    fn create_black_rook_a8() {
        let piece = super::create_black_rook("a8");
        assert_eq!(piece.color, Color::Black);
        assert_eq!(piece.position, Square::new("a8"));
        assert_eq!(piece.r#type, PieceType::Rook);
    }
    fn create_white_rook_a1() {
        let piece = super::create_white_rook("a1");
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.position, Square::new("a1"));
        assert_eq!(piece.r#type, PieceType::Rook);
    }

    #[test]
    fn starting_pieces() {
        let pieces = create_starting_pieces();
        assert_ne!(pieces.len(), 0);
    }
}
