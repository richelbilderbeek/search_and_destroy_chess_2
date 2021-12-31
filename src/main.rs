mod board;
mod color;
mod piece;
mod piece_type;
mod square;

extern crate find_folder;
extern crate piston_window;

use crate::board::Board;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Search And Destroy Chess 2", [400, 400])
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            rectangle(
                [1.0, 0.0, 0.0, 1.0],     // red
                [0.0, 0.0, 100.0, 100.0], // rectangle
                c.transform,
                g,
            );

            let transform = c.transform.trans(10.0, 100.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                .draw("Hello world!", &mut glyphs, &c.draw_state, transform, g)
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}
