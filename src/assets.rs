#[allow(dead_code)]
pub struct Assets {
    pub dark_square_texture: sfml::SfBox<sfml::graphics::Texture>,
    pub light_square_texture: sfml::SfBox<sfml::graphics::Texture>,
}

impl Assets {
    ///
    /// ```
    /// let assets = search_and_destroy_chess_2::assets::Assets::new();
    /// assert!(!assets.light_square_texture.is_smooth());
    /// ```

    pub fn new() -> Assets {
        Assets{
            dark_square_texture: sfml::graphics::Texture::from_file("assets/d.png").unwrap(),
            light_square_texture: sfml::graphics::Texture::from_file("assets/l.png").unwrap(),
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