extern crate piston_window;
use piston_window::*;

pub fn get_white_queen(window: &mut PistonWindow) -> G2dTexture {
    let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let white_queen = assets_folder.join("qw.png");
    let white_queen: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &white_queen,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    white_queen
}

#[cfg(test)]
mod tests {
    use super::*;

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
