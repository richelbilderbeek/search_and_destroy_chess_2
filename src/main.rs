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
    let black_bishop = assets.get_black_bishop();
    let black_king = assets.get_black_king();
    let black_knight = assets.get_black_knight();
    let black_pawn = assets.get_black_pawn();
    let black_queen = assets.get_black_queen();
    let black_rook = assets.get_black_rook();
    let white_bishop = assets.get_white_bishop();
    let white_king = assets.get_white_king();
    let white_knight = assets.get_white_knight();
    let white_pawn = assets.get_white_pawn();
    let white_queen = assets.get_white_queen();
    let white_rook = assets.get_white_rook();

    let dark_square = assets.get_square(Color::Black);
    let light_square = assets.get_square(Color::White);
    let board = crate::board::Board::new();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            //Draw the squares
            for file_index in 0..8 {
                for rank_index in 0..8 {
                    let x = rank_index as f64 * square_height;
                    let y = file_index as f64 * square_width;
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
            for file_index in 0..8 {
                for rank_index in 0..8 {
                    let x = rank_index as f64 * square_height;
                    let y = file_index as f64 * square_width;
                    let piece = board.get_piece_from_indices(file_index, rank_index);
                    if let Some(piece) = piece {
                        image(
                            match piece.get_color() {
                                crate::color::Color::Black => match piece.get_type() {
                                    crate::piece_type::PieceType::Bishop => &black_bishop,
                                    crate::piece_type::PieceType::Knight => &black_knight,
                                    crate::piece_type::PieceType::King => &black_king,
                                    crate::piece_type::PieceType::Pawn => &black_pawn,
                                    crate::piece_type::PieceType::Queen => &black_queen,
                                    crate::piece_type::PieceType::Rook => &black_rook,
                                },
                                crate::color::Color::White => match piece.get_type() {
                                    crate::piece_type::PieceType::Bishop => &white_bishop,
                                    crate::piece_type::PieceType::Knight => &white_knight,
                                    crate::piece_type::PieceType::King => &white_king,
                                    crate::piece_type::PieceType::Pawn => &white_pawn,
                                    crate::piece_type::PieceType::Queen => &white_queen,
                                    crate::piece_type::PieceType::Rook => &white_rook,
                                },
                            },
                            c.transform.trans(x, y).scale(scale_x, scale_y),
                            g,
                        );
                    }
                }
            }

            /*
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

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
            */
        });
    }
}
