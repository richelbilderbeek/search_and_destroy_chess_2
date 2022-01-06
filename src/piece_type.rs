/// The type of a chesspiece
/// ```
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::piece_type::to_str;
/// 
/// assert_eq!(to_str(PieceType::Bishop), "Bishop");
/// assert_eq!(to_str(PieceType::King), "King");
/// assert_eq!(to_str(PieceType::Knight), "Knight");
/// assert_eq!(to_str(PieceType::Pawn), "Pawn");
/// assert_eq!(to_str(PieceType::Queen), "Queen");
/// assert_eq!(to_str(PieceType::Rook), "Rook");
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}


/// Convert a PieceType to a String
/// 
/// ```
/// use search_and_destroy_chess_2::piece_type::PieceType;
/// use search_and_destroy_chess_2::piece_type::to_str;
/// 
/// assert_eq!(to_str(PieceType::Bishop), "Bishop");
/// assert_eq!(to_str(PieceType::King), "King");
/// assert_eq!(to_str(PieceType::Knight), "Knight");
/// assert_eq!(to_str(PieceType::Pawn), "Pawn");
/// assert_eq!(to_str(PieceType::Queen), "Queen");
/// assert_eq!(to_str(PieceType::Rook), "Rook");
/// ```
#[allow(dead_code)]
pub fn to_str(piece_type: PieceType) -> String {
    match piece_type {
        PieceType::Bishop => "Bishop".to_string(),
        PieceType::King => "King".to_string(),
        PieceType::Knight => "Knight".to_string(),
        PieceType::Pawn => "Pawn".to_string(),
        PieceType::Queen => "Queen".to_string(),
        PieceType::Rook => "Rook".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy() {
        let piece_type = PieceType::Bishop;
        let piece_type_copy = piece_type;
        assert_eq!(piece_type_copy, PieceType::Bishop);

    }
    #[test]
    fn clone() {
        let piece_type = PieceType::Bishop;
        let piece_type_clone = piece_type.clone();
        assert_eq!(piece_type, piece_type_clone);

    }

    #[test]
    fn bishop_to_str() {
        let str = to_str(PieceType::Bishop);
        assert_eq!(str, String::from("Bishop"));
    }
    #[test]
    fn king_to_str() {
        let str = to_str(PieceType::King);
        assert_eq!(str, String::from("King"));
    }
    #[test]
    fn knight_to_str() {
        let str = to_str(PieceType::Knight);
        assert_eq!(str, String::from("Knight"));
    }
    #[test]
    fn pawn_to_str() {
        let str = to_str(PieceType::Pawn);
        assert_eq!(str, String::from("Pawn"));
    }
    #[test]
    fn queen_to_str() {
        let str = to_str(PieceType::Queen);
        assert_eq!(str, String::from("Queen"));
    }
    #[test]
    fn rook_to_str() {
        let str = to_str(PieceType::Rook);
        assert_eq!(str, String::from("Rook"));
    }
}
