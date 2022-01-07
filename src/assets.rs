#[allow(dead_code)]
pub struct Assets {
    pub black_bishop: sfml::SfBox<sfml::graphics::Texture>,
    pub black_king: sfml::SfBox<sfml::graphics::Texture>,
    pub black_knight: sfml::SfBox<sfml::graphics::Texture>,
    pub black_pawn: sfml::SfBox<sfml::graphics::Texture>,
    pub black_queen: sfml::SfBox<sfml::graphics::Texture>,
    pub black_rook: sfml::SfBox<sfml::graphics::Texture>,
    pub dark_square: sfml::SfBox<sfml::graphics::Texture>,
    pub light_square: sfml::SfBox<sfml::graphics::Texture>,
    question_mark: sfml::SfBox<sfml::graphics::Texture>,
    pub white_bishop: sfml::SfBox<sfml::graphics::Texture>,
    pub white_king: sfml::SfBox<sfml::graphics::Texture>,
    pub white_knight: sfml::SfBox<sfml::graphics::Texture>,
    pub white_pawn: sfml::SfBox<sfml::graphics::Texture>,
    pub white_queen: sfml::SfBox<sfml::graphics::Texture>,
    pub white_rook: sfml::SfBox<sfml::graphics::Texture>,
}

impl Assets {
    ///Create a new collection of assets
    /// 
    /// ```
    /// use search_and_destroy_chess_2::assets::Assets;
    /// use search_and_destroy_chess_2::color::Color;
    /// use search_and_destroy_chess_2::piece::*;
    /// 
    /// let assets = Assets::new();
    /// let dark_square = assets.get_square(Color::Black);
    /// let black_bishop = assets.get_piece(create_black_bishop("a1")); // Note that the position is irrelevant
    /// ```
    pub fn new() -> Assets {
        Assets{
            black_bishop: sfml::graphics::Texture::from_file("assets/bb.png").unwrap(),
            black_king: sfml::graphics::Texture::from_file("assets/kb.png").unwrap(),
            black_knight: sfml::graphics::Texture::from_file("assets/nb.png").unwrap(),
            black_pawn: sfml::graphics::Texture::from_file("assets/pb.png").unwrap(),
            black_queen: sfml::graphics::Texture::from_file("assets/qb.png").unwrap(),
            black_rook: sfml::graphics::Texture::from_file("assets/rb.png").unwrap(),
            dark_square: sfml::graphics::Texture::from_file("assets/d.png").unwrap(),
            light_square: sfml::graphics::Texture::from_file("assets/l.png").unwrap(),
            question_mark: sfml::graphics::Texture::from_file("assets/qm.png").unwrap(),
            white_bishop: sfml::graphics::Texture::from_file("assets/bw.png").unwrap(),
            white_king: sfml::graphics::Texture::from_file("assets/kw.png").unwrap(),
            white_knight: sfml::graphics::Texture::from_file("assets/nw.png").unwrap(),
            white_pawn: sfml::graphics::Texture::from_file("assets/pw.png").unwrap(),
            white_queen: sfml::graphics::Texture::from_file("assets/qw.png").unwrap(),
            white_rook: sfml::graphics::Texture::from_file("assets/rw.png").unwrap(),
        }
    }

    /// Get the height of the images used
    /// ```
    /// use search_and_destroy_chess_2::assets::Assets;
    /// 
    /// let assets = Assets::new();
    /// assert_eq!(assets.get_image_width(), 128);
    /// assert_eq!(assets.get_image_height(), 128);
    /// ```
    pub fn get_image_height(&self) -> i32 { 128 }

    /// Get the width of the images used
    /// ```
    /// use search_and_destroy_chess_2::assets::Assets;
    /// 
    /// let assets = Assets::new();
    /// assert_eq!(assets.get_image_width(), 128);
    /// assert_eq!(assets.get_image_height(), 128);
    /// ```
    pub fn get_image_width(&self) -> i32 { 128 }

    /// Get the texture of a piece
    ///
    /// ```
    /// use search_and_destroy_chess_2::assets::Assets;
    /// use search_and_destroy_chess_2::piece::*;
    /// 
    /// let assets = Assets::new();
    /// let black_bishop = assets.get_piece(create_black_bishop("c8"));
    /// ```
    pub fn get_piece(&self, piece: crate::piece::Piece) ->  &sfml::SfBox<sfml::graphics::Texture> {
        match piece.get_color() {
            crate::color::Color::Black => match piece.get_type() {
                crate::piece_type::PieceType::Bishop => &self.black_bishop,
                crate::piece_type::PieceType::Knight => &self.black_knight,
                crate::piece_type::PieceType::King => &self.black_king,
                crate::piece_type::PieceType::Pawn => &self.black_pawn,
                crate::piece_type::PieceType::Queen => &self.black_queen,
                crate::piece_type::PieceType::Rook => &self.black_rook,
            },
            crate::color::Color::White => match piece.get_type() {
                crate::piece_type::PieceType::Bishop => &self.white_bishop,
                crate::piece_type::PieceType::Knight => &self.white_knight,
                crate::piece_type::PieceType::King => &self.white_king,
                crate::piece_type::PieceType::Pawn => &self.white_pawn,
                crate::piece_type::PieceType::Queen => &self.white_queen,
                crate::piece_type::PieceType::Rook => &self.white_rook,
            },
        }
    }

    /// Get the texture of a question mark
    ///
    /// ```
    /// use search_and_destroy_chess_2::assets::Assets;
    /// 
    /// let assets = Assets::new();
    /// let question_mark = assets.get_question_mark();
    /// ```
    pub fn get_question_mark(&self) ->  &sfml::SfBox<sfml::graphics::Texture> {
        &self.question_mark
    }

    /// Get the texture of a square
    ///
    /// ```
    /// use search_and_destroy_chess_2::assets::Assets;
    /// use search_and_destroy_chess_2::color::Color;
    /// 
    /// let assets = Assets::new();
    /// let dark_square = assets.get_square(Color::Black);
    /// let light_square = assets.get_square(Color::White);
    /// ```
    pub fn get_square(&self, color: crate::color::Color) ->  &sfml::SfBox<sfml::graphics::Texture> {
        use crate::color::Color;

        match color {
            Color::Black => &self.dark_square,
            Color::White => &self.light_square,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let assets = Assets::new();
        assert!(!assets.get_question_mark().is_smooth());
    }
}