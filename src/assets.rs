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
    /// 
    /// let assets = Assets::new();
    /// assert!(!assets.light_square_texture.is_smooth());
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
            white_bishop: sfml::graphics::Texture::from_file("assets/bw.png").unwrap(),
            white_king: sfml::graphics::Texture::from_file("assets/kw.png").unwrap(),
            white_knight: sfml::graphics::Texture::from_file("assets/nw.png").unwrap(),
            white_pawn: sfml::graphics::Texture::from_file("assets/pw.png").unwrap(),
            white_queen: sfml::graphics::Texture::from_file("assets/qw.png").unwrap(),
            white_rook: sfml::graphics::Texture::from_file("assets/rw.png").unwrap(),
        }
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

/*
    pub fn get_black_bishop(&mut self) -> piston_window::G2dTexture {
        self.get_texture("bb.png")
    }
    pub fn get_black_king(&mut self) -> piston_window::G2dTexture {
        self.get_texture("kb.png")
    }
    pub fn get_black_knight(&mut self) -> piston_window::G2dTexture {
        self.get_texture("nb.png")
    }
    pub fn get_black_pawn(&mut self) -> piston_window::G2dTexture {
        self.get_texture("pb.png")
    }
    pub fn get_black_queen(&mut self) -> piston_window::G2dTexture {
        self.get_texture("qb.png")
    }
    pub fn get_black_rook(&mut self) -> piston_window::G2dTexture {
        self.get_texture("rb.png")
    }
    pub fn get_dark_square(&mut self) -> piston_window::G2dTexture {
        self.get_texture("d.png")
    }
    pub fn get_font(&mut self) -> piston_window::Glyphs {
        let glyphs = self
            .window
            .load_font(self.assets_folder.join("FiraSans-Regular.ttf"))
            .unwrap();
        glyphs
    }
    pub fn get_light_square(&mut self) -> piston_window::G2dTexture {
        self.get_texture("l.png")
    }
    pub fn get_square(&mut self, color: crate::color::Color) -> piston_window::G2dTexture {
        match color {
            crate::color::Color::White => self.get_light_square(),
            crate::color::Color::Black => self.get_dark_square(),
        }
    }
    pub fn get_texture(&mut self, filename: &str) -> piston_window::G2dTexture {
        let texture_path = self.assets_folder.join(filename);
        let texture: piston_window::G2dTexture = piston_window::Texture::from_path(
            &mut self.window.create_texture_context(),
            &texture_path,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new(),
        )
        .unwrap();
        texture
    }
    pub fn get_white_bishop(&mut self) -> piston_window::G2dTexture {
        self.get_texture("bw.png")
    }
    pub fn get_white_king(&mut self) -> piston_window::G2dTexture {
        self.get_texture("kw.png")
    }
    pub fn get_white_knight(&mut self) -> piston_window::G2dTexture {
        self.get_texture("nw.png")
    }
    pub fn get_white_pawn(&mut self) -> piston_window::G2dTexture {
        self.get_texture("pw.png")
    }
    pub fn get_white_queen(&mut self) -> piston_window::G2dTexture {
        self.get_texture("qw.png")
    }
    pub fn get_white_rook(&mut self) -> piston_window::G2dTexture {
        self.get_texture("rw.png")
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let assets = Assets::new();
        assert!(!assets.light_square_texture.is_smooth());
        assert_eq!(1 + 1, 2);
    }
}