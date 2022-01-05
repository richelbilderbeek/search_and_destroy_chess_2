mod board;
mod color;
mod piece;
mod piece_type;
mod square;
//mod sound_asset;

extern crate rand;
extern crate sfml;

use rand::{thread_rng, Rng};
use sfml::{
    audio::{Sound, SoundBuffer, SoundSource},
    graphics::{
        CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
        Transformable,
    },
    system::{Clock, Time, Vector2f},
    window::{ContextSettings, Event, Key, Style},
};

#[allow(unused_imports)]
use std::{env, f32::consts::PI};

#[allow(dead_code)]
struct GameView {
    game_width: u32,
    game_height: u32,
    window: std::cell::RefCell<RenderWindow>,
    paddle_size: Vector2f,
    ball_radius: f32,
    board: board::Board,
    //assets: crate::assets::Assets<'a>,
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
            paddle_size: Vector2f::new(25., 100.),
            ball_radius: 10.,
            board: board::Board::new(),
        }
    }
    pub fn run(&self) {
        let mut rng = thread_rng();
        self.window.borrow_mut().set_vertical_sync_enabled(true);

        // Load the sounds used in the game
        let ball_soundbuffer: sfml::SfBox<SoundBuffer> = SoundBuffer::from_file("assets/examples_resources_ball.wav").unwrap();
        let mut ball_sound: sfml::audio::Sound = Sound::with_buffer(&ball_soundbuffer);

        // Create the left paddle
        let mut left_paddle = RectangleShape::new();
        left_paddle.set_size(self.paddle_size - 3.);
        left_paddle.set_outline_thickness(3.);
        left_paddle.set_outline_color(Color::BLACK);
        left_paddle.set_fill_color(Color::rgb(100, 100, 200));
        left_paddle.set_origin(self.paddle_size / 2.);

        // Create the right paddle
        let mut right_paddle = RectangleShape::new();
        right_paddle.set_size(self.paddle_size - 3.);
        right_paddle.set_outline_thickness(3.);
        right_paddle.set_outline_color(Color::BLACK);
        right_paddle.set_fill_color(Color::rgb(200, 100, 100));
        right_paddle.set_origin(self.paddle_size / 2.);

        // Create the ball
        let mut ball = CircleShape::default();
        ball.set_radius(self.ball_radius - 3.);
        ball.set_outline_thickness(3.);
        ball.set_outline_color(Color::BLACK);
        ball.set_fill_color(Color::WHITE);
        ball.set_origin((self.ball_radius / 2., self.ball_radius / 2.));

        // Load the text font
        let font = Font::from_file("assets/sansation.ttf").unwrap();

        // Initialize the pause message
        let mut pause_message = Text::default();
        pause_message.set_font(&font);
        pause_message.set_character_size(40);
        pause_message.set_position((170., 150.));
        pause_message.set_fill_color(Color::WHITE);
        pause_message.set_string("Welcome to SFML pong!\nPress space to start the game");

        // Define the paddles properties
        let mut ai_timer = Clock::start();
        let ai_time = Time::seconds(0.0333);
        let paddle_speed = 400.;
        let mut right_paddle_speed = 0.;
        let mut ball_speed = 400.;
        let mut ball_angle: f32 = 0.;

        let mut clock = Clock::start();
        let mut is_playing = false;
        let mut up = false;
        let mut down = false;

        loop {
            while let Some(event) = &self.window.borrow_mut().poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::ESCAPE, ..
                    } => return,
                    Event::KeyPressed {
                        code: Key::SPACE, ..
                    } if !is_playing => {
                        // (re)start the game
                        is_playing = true;
                        ball_speed = 400.0;
                        ball_sound.set_pitch(1.0);
                        clock.restart();
                        // Reset the position of the paddles and ball
                        left_paddle.set_position((10. + self.paddle_size.x / 2., self.game_height as f32 / 2.));
                        right_paddle.set_position((
                            self.game_width as f32 - 10. - self.paddle_size.x / 2.,
                            self.game_height as f32 / 2.,
                        ));
                        ball.set_position((self.game_width as f32 / 2., self.game_height as f32 / 2.));
                        // Reset the ball angle
                        loop {
                            // Make sure the ball initial angle is not too much vertical
                            ball_angle = rng.gen_range(0.0..360.) * 2. * PI / 360.;

                            if ball_angle.cos().abs() >= 0.7 {
                                break;
                            }
                        }
                    }
                    Event::KeyPressed { code: Key::UP, .. } => up = true,
                    Event::KeyReleased { code: Key::UP, .. } => up = false,
                    Event::KeyPressed {
                        code: Key::DOWN, ..
                    } => down = true,
                    Event::KeyReleased {
                        code: Key::DOWN, ..
                    } => down = false,
                    _ => {}
                }
            }
            if is_playing {
                let delta_time = clock.restart().as_seconds();

                // Move the player's paddle
                if up && (left_paddle.position().y - self.paddle_size.y / 2. > 5.) {
                    left_paddle.move_((0., -paddle_speed * delta_time));
                }
                if down && (left_paddle.position().y + self.paddle_size.y / 2. < self.game_height as f32 - 5.) {
                    left_paddle.move_((0., paddle_speed * delta_time));
                }

                // Move the computer's paddle
                if ((right_paddle_speed < 0.) && (right_paddle.position().y - self.paddle_size.y / 2. > 5.))
                    || ((right_paddle_speed > 0.)
                        && (right_paddle.position().y + self.paddle_size.y / 2. < self.game_height as f32 - 5.))
                {
                    right_paddle.move_((0., right_paddle_speed * delta_time));
                }

                // Update the computer's paddle direction according to the ball position
                if ai_timer.elapsed_time() > ai_time {
                    ai_timer.restart();
                    if ball.position().y + self.ball_radius > right_paddle.position().y + self.paddle_size.y / 2.
                    {
                        right_paddle_speed = paddle_speed;
                    } else if ball.position().y - self.ball_radius
                        < right_paddle.position().y - self.paddle_size.y / 2.
                    {
                        right_paddle_speed = -paddle_speed;
                    } else {
                        right_paddle_speed = 0.;
                    }
                }

                // Move the ball
                let factor = ball_speed * delta_time;
                ball.move_((ball_angle.cos() * factor, ball_angle.sin() * factor));

                // Check collisions between the ball and the screen
                if ball.position().x - self.ball_radius < 0. {
                    is_playing = false;
                    pause_message.set_string("You lost !\nPress space to restart or\nescape to exit");
                }
                if ball.position().x + self.ball_radius > self.game_width as f32 {
                    is_playing = false;
                    pause_message.set_string("You won !\nPress space to restart or\nescape to exit");
                }
                if ball.position().y - self.ball_radius < 0. {
                    on_bounce(&mut ball_sound, &mut ball_speed);
                    ball_angle = -ball_angle;
                    let p = ball.position().x;
                    ball.set_position((p, self.ball_radius + 0.1));
                }
                if ball.position().y + self.ball_radius > self.game_height as f32 {
                    on_bounce(&mut ball_sound, &mut ball_speed);
                    ball_angle = -ball_angle;
                    let p = ball.position().x;
                    ball.set_position((p, self.game_height as f32 - self.ball_radius - 0.1));
                }

                // Check the collisions between the ball and the paddles
                // Left Paddle
                let (ball_pos, paddle_pos) = (ball.position(), left_paddle.position());
                if ball_pos.x - self.ball_radius < paddle_pos.x + self.paddle_size.x / 2.
                    && ball_pos.y + self.ball_radius >= paddle_pos.y - self.paddle_size.y / 2.
                    && ball_pos.y - self.ball_radius <= paddle_pos.y + self.paddle_size.y / 2.
                {
                    if ball_pos.y > paddle_pos.y {
                        ball_angle = PI - ball_angle + rng.gen_range(0.0..20.) * PI / 180.;
                    } else {
                        ball_angle = PI - ball_angle - rng.gen_range(0.0..20.) * PI / 180.;
                    }

                    on_bounce(&mut ball_sound, &mut ball_speed);
                    ball.set_position((
                        paddle_pos.x + self.ball_radius + self.paddle_size.x / 2. + 0.1,
                        ball_pos.y,
                    ));
                }

                // Right Paddle
                let (ball_pos, paddle_pos) = (ball.position(), right_paddle.position());
                if ball_pos.x + self.ball_radius > paddle_pos.x - self.paddle_size.x / 2.
                    && ball_pos.y + self.ball_radius >= paddle_pos.y - self.paddle_size.y / 2.
                    && ball_pos.y - self.ball_radius <= paddle_pos.y + self.paddle_size.y / 2.
                {
                    if ball_pos.y > paddle_pos.y {
                        ball_angle = PI - ball_angle + rng.gen_range(0.0..20.) * PI / 180.;
                    } else {
                        ball_angle = PI - ball_angle - rng.gen_range(0.0..20.) * PI / 180.;
                    }

                    on_bounce(&mut ball_sound, &mut ball_speed);
                    ball.set_position((
                        paddle_pos.x - self.ball_radius - self.paddle_size.x / 2. - 0.1,
                        ball_pos.y,
                    ));
                }
            }
            // Clear the window
            self.window.borrow_mut().clear(Color::rgb(50, 200, 50));

            if is_playing {
                // Draw the paddles and the ball
                self.window.borrow_mut().draw(&left_paddle);
                self.window.borrow_mut().draw(&right_paddle);
                self.window.borrow_mut().draw(&ball);
            } else {
                // Draw the pause message
                self.window.borrow_mut().draw(&pause_message);
            }

            // Display things on screen
            self.window.borrow_mut().display()
        }
    }
}

fn main() {
    let game_width = 800;
    let game_height = 600;
    let game_view = GameView::new(game_width, game_height);
    game_view.run()
}

fn on_bounce(ball_sound: &mut sfml::audio::Sound, ball_speed: &mut f32) {
    ball_sound.play();
    ball_sound.set_pitch(ball_sound.pitch() + 0.004);
    *ball_speed += 16.0;
}

/*
mod assets;
mod board;
mod color;
mod piece;
mod piece_type;
mod square;

extern crate find_folder;

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
