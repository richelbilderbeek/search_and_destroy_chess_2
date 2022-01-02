mod assets;
mod board;
mod color;
mod piece;
mod piece_type;
mod square;

extern crate find_folder;
extern crate piston_window;

use crate::assets::Assets;
use crate::board::get_square_color_from_indices;
use crate::color::Color;
use piston_window::*;

pub fn color_to_screen_color(color: Color) -> [f32; 4] {
    match color {
        Color::Black => [0.0, 0.0, 0.0, 1.0],
        Color::White => [1.0, 1.0, 1.0, 1.0],
    }
}

fn main() {
    let window_width = 400;
    let window_height = 400;
    let window_size = [window_width, window_height];
    let square_width = window_width as f64 / 8.0;
    let square_height = window_height as f64 / 8.0;
    let image_width = 128; // pixels
    let image_height = 128; // pixels
    let scale_x = square_width / image_width as f64;
    let scale_y = square_height / image_height as f64;
    let mut window: PistonWindow = WindowSettings::new("Search And Destroy Chess 2", window_size)
        .build()
        .unwrap();

    let mut assets = Assets::new(&mut window);
    let mut glyphs = assets.get_font();
    let white_queen = assets.get_white_queen();
    let dark_square = assets.get_square(Color::Black);
    let light_square = assets.get_square(Color::White);
    let board = crate::board::Board::new();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            //Draw the squares
            for file_index in 0..8 {
                for rank_index in 0..8 {
                    let x = file_index as f64 * square_width;
                    let y = rank_index as f64 * square_height;
                    let color = get_square_color_from_indices(file_index, rank_index);
                    image(
                        match color {
                            crate::color::Color::Black => &dark_square,
                            crate::color::Color::White => &light_square,
                        },
                        c.transform.trans(x, y).scale(scale_x, scale_y),
                        g,
                    );
                }
            }

            //Draw the pieces
            /*
            for file_index in 0..8 {
                for rank_index in 0..8 {
                    let x = file_index as f64 * square_width;
                    let y = rank_index as f64 * square_height;
                    let piece = board.get_piece_from_indices(file_index, rank_index);
                    image(
                        match piece.get_color() {
                            crate::color::Color::White => match piece.get_type() {
                                crate::piece_type::PieceType::Queen => &white_queen,
                            },
                            crate::color::Color::White => match piece.get_type() {

                            },
                        },
                        c.transform.trans(x, y).scale(scale_x, scale_y),
                        g,
                    );
                }
            }
            */

            let transform = c.transform.trans(10.0, 100.0);
            let color = [1.0, 0.0, 0.0, 1.0];
            text::Text::new_color(color, 32)
                .draw("Hello world!", &mut glyphs, &c.draw_state, transform, g)
                .unwrap();

            text::Text::new_color(color, 32)
                .draw(
                    "Again",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 200.0),
                    g,
                )
                .unwrap();
            image(
                &white_queen,
                c.transform.trans(100.0, 200.0).scale(scale_x, scale_y),
                g,
            );

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}
