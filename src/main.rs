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
    graphics::{
        RenderTarget,
        Transformable,
    },
};

use crate::board::get_square_color_from_indices;

#[allow(unused_imports)]
use std::{env, f32::consts::PI};

#[allow(dead_code)]
struct GameView {
    game_width: u32,
    game_height: u32,
    window: std::cell::RefCell<sfml::graphics::RenderWindow>,
    board: board::Board,
    font: sfml::SfBox<sfml::graphics::Font>,
    assets: crate::assets::Assets,
}

impl GameView {
    pub fn new(game_width: u32, game_height: u32) -> GameView {

        // Create the window of the application
        let window = std::cell::RefCell::new(
            sfml::graphics::RenderWindow::new(
                (game_width, game_height),
                "Search And Destroy Chess 2",
                sfml::window::Style::CLOSE,
                &sfml::window::ContextSettings::default(),
            )
        );
        GameView {
            game_width,
            game_height,
            window,
            board: board::Board::new(),
            font: sfml::graphics::Font::from_file("assets/sansation.ttf").unwrap(),
            assets: crate::assets::Assets::new(),
        }
    }
    pub fn run(&self) {
        self.window.borrow_mut().set_vertical_sync_enabled(true);

        let ball_soundbuffer: sfml::SfBox<sfml::audio::SoundBuffer> = sfml::audio::SoundBuffer::from_file("assets/examples_resources_ball.wav").unwrap();
        let mut ball_sound: sfml::audio::Sound = sfml::audio::Sound::with_buffer(&ball_soundbuffer);

        let mut up = false;
        let mut down = false;

        loop {
            while let Some(event) = &self.window.borrow_mut().poll_event() {
                match event {
                    sfml::window::Event::Closed
                    | sfml::window::Event::KeyPressed {
                        code: sfml::window::Key::ESCAPE, ..
                    } => return,
                    sfml::window::Event::KeyPressed { code: sfml::window::Key::UP, .. } => {
                        ball_sound.play();
                        up = true
                    },
                    sfml::window::Event::KeyReleased { code: sfml::window::Key::UP, .. } => up = false,
                    sfml::window::Event::KeyPressed { code: sfml::window::Key::DOWN, .. } => down = true,
                    sfml::window::Event::KeyReleased { code: sfml::window::Key::DOWN, .. } => down = false,
                    _ => {}
                }
            }

            // Clear the window
            self.window.borrow_mut().clear(sfml::graphics::Color::rgb(50, 200, 50));

            assert!(up == true || up == false);
            assert!(down == true || down == false);

            self.draw_squares();
            self.draw_pieces();
            self.draw_fog_of_war();

            // Display things on screen
            self.window.borrow_mut().display();
        }
    }
    pub fn draw_fog_of_war(&self) {
        use crate::board::get_invisible_squares;
        use crate::square::get_nth_file;
        use crate::square::get_nth_rank;
        use crate::color::Color;

        let image_width = 128; // pixels
        let image_height = 128; // pixels
        let square_width = self.game_width as f32 / 8.0;
        let square_height = self.game_height as f32 / 8.0;
        let scale_x = square_width / image_width as f32;
        let scale_y = square_height / image_height as f32;
        let squares = get_invisible_squares(&self.board, Color::Black);
        for square in squares {
            let file_index = get_nth_file(&square);
            let rank_index = get_nth_rank(&square);
            let x = file_index as f32 * square_width;
            // files go up, 'file_index + 1' as tiles are draw from top
            let y = self.game_height as f32 - ((rank_index + 1) as f32 * square_height);
            let mut sprite = sfml::graphics::Sprite::with_texture(&self.assets.get_question_mark());
            sprite.set_position(sfml::system::Vector2f::new(x, y));
            sprite.set_scale(sfml::system::Vector2f::new(scale_x, scale_y));
            self.window.borrow_mut().draw(&sprite);
        }
    }
    pub fn draw_pieces(&self) {

        let image_width = 128; // pixels
        let image_height = 128; // pixels
        let square_width = self.game_width as f32 / 8.0;
        let square_height = self.game_height as f32 / 8.0;
        let scale_x = square_width / image_width as f32;
        let scale_y = square_height / image_height as f32;
        for file_index in 0..8 {
            for rank_index in 0..8 {
                let x = file_index as f32 * square_width;
                // files go up, 'file_index + 1' as tiles are draw from top
                let y = self.game_height as f32 - ((rank_index + 1) as f32 * square_height);
                let piece_option = self.board.get_piece_from_indices(file_index, rank_index);
                if let Some(piece) = piece_option {
                        let mut sprite = sfml::graphics::Sprite::with_texture(&self.assets.get_piece(piece));
                        sprite.set_position(sfml::system::Vector2f::new(x, y));
                        sprite.set_scale(sfml::system::Vector2f::new(scale_x, scale_y));
                        self.window.borrow_mut().draw(&sprite);
                }
            }
        }
    }
    pub fn draw_squares(&self) {

        let image_width = 128; // pixels
        let image_height = 128; // pixels
        let square_width = self.game_width as f32 / 8.0;
        let square_height = self.game_height as f32 / 8.0;
        let scale_x = square_width / image_width as f32;
        let scale_y = square_height / image_height as f32;
        for file_index in 0..8 {
            for rank_index in 0..8 {
                let x = file_index as f32 * square_width;
                // files go up, 'file_index + 1' as tiles are draw from top
                let y = self.game_height as f32 - ((rank_index + 1) as f32 * square_height);
                let color = get_square_color_from_indices(file_index, rank_index);
                let mut sprite = sfml::graphics::Sprite::with_texture(&self.assets.get_square(color));
                sprite.set_position(sfml::system::Vector2f::new(x, y));
                sprite.set_scale(sfml::system::Vector2f::new(scale_x, scale_y));
                self.window.borrow_mut().draw(&sprite);
            }
        }
    }
}

fn main() {
    let game_width = 800;
    let game_height = 600;
    let game_view = GameView::new(game_width, game_height);
    game_view.run()
}