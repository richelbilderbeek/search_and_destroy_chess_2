pub struct Assets<'a> {
    assets_folder: std::path::PathBuf,
    window: &'a mut piston_window::PistonWindow,
}

impl<'a> Assets<'a> {
    pub fn new(window: &mut piston_window::PistonWindow) -> Assets {
        Assets {
            assets_folder: find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap(),
            window,
        }
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
    pub fn get_white_queen(&mut self) -> piston_window::G2dTexture {
        self.get_texture("qw.png")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn constructor() {
        //let assets = Assets::new();
        assert_eq!(1 + 1, 2);
    }
}
