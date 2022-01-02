pub fn get_font(window: &mut piston_window::PistonWindow) -> piston_window::Glyphs {

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();
    glyphs
}

pub fn get_white_queen(window: &mut piston_window::PistonWindow) -> piston_window::G2dTexture {
    let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let white_queen = assets_folder.join("qw.png");
    let white_queen: piston_window::G2dTexture = piston_window::Texture::from_path(
        &mut window.create_texture_context(),
        &white_queen,
        piston_window::Flip::None,
        &piston_window::TextureSettings::new(),
    ).unwrap();

    white_queen
}

#[cfg(test)]
mod tests {


    #[test]
    fn constructor() {
        //let mut window: PistonWindow = WindowSettings::new("Test", [640, 400])
        //    .build()
        //    .unwrap();
        //let white_queen = get_white_queen(&mut window);

        // Cannot test, as creating a window creates
        // another EventLoop
        assert_eq!(1 + 1, 2);
    }
}
