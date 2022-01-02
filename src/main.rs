mod assets;
mod board;
mod color;
mod piece;
mod piece_type;
mod square;

extern crate find_folder;
extern crate piston_window;

use crate::board::get_square_color_from_indices;
use crate::assets::Assets;
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

    let assets = Assets::new();
    let mut glyphs = assets::get_font(&mut window);
    let white_queen = assets::get_white_queen(&mut window);
    let dark_square = assets.get_dark_square(&mut window);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for file_index in 0..8 {
                for rank_index in 0..8 {
                    let x = file_index as f64 * square_width;
                    let y = rank_index as f64 * square_height;
                    let color = get_square_color_from_indices(file_index, rank_index);
                    let screen_color = color_to_screen_color(color);

                    rectangle(
                        screen_color,
                        [x, y, square_width, square_height],
                        c.transform,
                        g,
                    );
                }
            }

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
                &dark_square,
                c.transform.trans(100.0, 200.0).scale(scale_x, scale_y),
                g,
            );
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
