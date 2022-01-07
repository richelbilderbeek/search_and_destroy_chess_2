pub mod assets;
pub mod board;
pub mod color;
pub mod piece;
pub mod piece_type;
pub mod square;
pub mod scribble;

extern crate rand;
extern crate sfml;

use sfml::{
    audio::{Sound, SoundBuffer},
    graphics::{
        Color, Font, RenderTarget, RenderWindow,
        Transformable,
    },
    system::{Vector2f},
    window::{ContextSettings, Event, Key, Style},
};

#[allow(unused_imports)]
use std::{env, f32::consts::PI};

#[allow(dead_code)]
struct GameView {
    game_width: u32,
    game_height: u32,
    window: std::cell::RefCell<RenderWindow>,
    board: board::Board,
    font: sfml::SfBox<sfml::graphics::Font>,
    assets: crate::assets::Assets,
}

impl GameView {
    pub fn new(game_width: u32, game_height: u32) -> GameView {

        // Create the window of the application
        let window = std::cell::RefCell::new(
            RenderWindow::new(
                (game_width, game_height),
                "Search And Destroy Chess 2",
                Style::CLOSE,
                &ContextSettings::default(),
            )
        );
        GameView {
            game_width,
            game_height,
            window,
            board: board::Board::new(),
            font: Font::from_file("assets/sansation.ttf").unwrap(),
            assets: crate::assets::Assets::new(),
        }
    }
    pub fn run(&self) {
        self.window.borrow_mut().set_vertical_sync_enabled(true);

        // Load the sounds used in the game
        let ball_soundbuffer: sfml::SfBox<SoundBuffer> = SoundBuffer::from_file("assets/examples_resources_ball.wav").unwrap();
        let mut ball_sound: sfml::audio::Sound = Sound::with_buffer(&ball_soundbuffer);

        let mut dark_square_sprite = sfml::graphics::Sprite::with_texture(&self.assets.dark_square_texture);
        dark_square_sprite.set_origin(Vector2f::new(28_f32, 28_f32));
        dark_square_sprite.set_position(Vector2f::new(0_f32, 0_f32));

        let mut light_square_sprite = sfml::graphics::Sprite::with_texture(&self.assets.light_square_texture);
        light_square_sprite.set_origin(Vector2f::new(28_f32, 28_f32));
        light_square_sprite.set_position(Vector2f::new(320_f32, 320_f32));


        let mut up = false;
        let mut down = false;

        loop {
            while let Some(event) = &self.window.borrow_mut().poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::ESCAPE, ..
                    } => return,
                    Event::KeyPressed { code: Key::UP, .. } => {
                        ball_sound.play();
                        up = true
                    },
                    Event::KeyReleased { code: Key::UP, .. } => up = false,
                    Event::KeyPressed { code: Key::DOWN, .. } => down = true,
                    Event::KeyReleased { code: Key::DOWN, .. } => down = false,
                    _ => {}
                }
            }

            // Clear the window
            self.window.borrow_mut().clear(Color::rgb(50, 200, 50));

            assert!(up == true || up == false);
            assert!(down == true || down == false);

            self.window.borrow_mut().draw(&dark_square_sprite);
            self.window.borrow_mut().draw(&light_square_sprite);

            // Display things on screen
            self.window.borrow_mut().display();
        }
    }
}

fn main() {
    let game_width = 800;
    let game_height = 600;
    let game_view = GameView::new(game_width, game_height);
    game_view.run()
}


/*
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

        });
    }
}
*/
