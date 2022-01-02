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
            window: window,
        }
    }
    pub fn get_dark_square(&mut self) -> piston_window::G2dTexture {
        let texture_path = self.assets_folder.join("d.png");
        let texture: piston_window::G2dTexture = piston_window::Texture::from_path(
            &mut self.window.create_texture_context(),
            &texture_path,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new(),
        )
        .unwrap();

        texture
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
        let white_queen = self.assets_folder.join("qw.png");
        let white_queen: piston_window::G2dTexture = piston_window::Texture::from_path(
            &mut self.window.create_texture_context(),
            &white_queen,
            piston_window::Flip::None,
            &piston_window::TextureSettings::new(),
        )
        .unwrap();

        white_queen
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
