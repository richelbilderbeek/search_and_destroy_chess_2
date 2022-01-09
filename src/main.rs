pub mod assets;
pub mod board;
pub mod color;
pub mod file_index;
pub mod game_view;
pub mod is_on_gha;
pub mod piece;
pub mod piece_type;
pub mod rank;
pub mod square;
pub mod scribble;

extern crate rand;
extern crate sfml;

fn main() {
    if crate::is_on_gha::is_on_gha() {
        println!("On GHA");

    } else {
        use crate::game_view::GameView;
        let game_width = 800;
        let game_height = 600;
        let game_view = GameView::new(game_width, game_height);
        game_view.run()
    }
}