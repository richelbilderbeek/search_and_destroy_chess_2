use sfml::{
    graphics::{
        RenderTarget,
        Transformable,
    },
};

pub struct GameView {
    game_width: u32,
    game_height: u32,
    window: std::cell::RefCell<sfml::graphics::RenderWindow>,
    game: crate::game::Game,
    assets: crate::assets::Assets,
}

impl GameView {
    pub fn new(game_width: u32, game_height: u32) -> GameView {

        // Cannot run on GitHub Actions
        assert!(!crate::is_on_gha::is_on_gha());

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
            game: crate::game::Game::new(),
            assets: crate::assets::Assets::new(),
        }
    }

    /// Draw the screen and displating it when done
    pub fn draw(&self) {
        // Clear the window: not needed now, but will be in the future
        self.window.borrow_mut().clear(sfml::graphics::Color::rgb(50, 200, 50));

        self.draw_squares();
        self.draw_square_coordinats();
        self.draw_pieces();
        self.draw_fog_of_war();
        self.draw_possible_target_squares();
        self.draw_selector();

        // Display things on screen
        self.window.borrow_mut().display();
    }
    /// Draw the question marks ruthlessly obscuring the squares of the board
    fn draw_fog_of_war(&self) {
        use crate::color::Color;
        use crate::game::get_invisible_squares;
        use crate::square::get_nth_file;
        use crate::square::get_nth_rank;

        let squares = get_invisible_squares(&self.game, Color::Black);
        for square in squares {
            let file_index = get_nth_file(&square);
            let rank_index = get_nth_rank(&square);
            let x = file_index.get() as f32 * get_square_width(&self) as f32;
            // files go up, 'file_index + 1' as tiles are draw from top
            let y = self.game_height as f32 - ((rank_index + 1) as f32 * get_square_height(&self) as f32);
            let mut sprite = sfml::graphics::Sprite::with_texture(&self.assets.get_question_mark());
            sprite.set_position(sfml::system::Vector2f::new(x, y));
            sprite.set_scale(sfml::system::Vector2f::new(get_scale_x(self), get_scale_y(self)));
            self.window.borrow_mut().draw(&sprite);
        }
    }
    /// Draw the chess pieces
    fn draw_pieces(&self) {
        for file_index in crate::file_index::get_all_file_indices() {
            for rank_index in 0..8 {
                let x = file_index.get() as f32 * get_square_width(&self) as f32;
                // files go up, 'file_index + 1' as tiles are draw from top
                let y = self.game_height as f32 - ((rank_index + 1) as f32 * get_square_height(&self) as f32);
                let piece_option = crate::game::get_piece_from_indices(&self.game, &file_index, rank_index);
                if let Some(piece) = piece_option {
                        let mut sprite = sfml::graphics::Sprite::with_texture(&self.assets.get_piece(piece));
                        sprite.set_position(sfml::system::Vector2f::new(x, y));
                        sprite.set_scale(sfml::system::Vector2f::new(get_scale_x(self), get_scale_y(self)));
                        self.window.borrow_mut().draw(&sprite);
                }
            }
        }
    }
    /// Draw the possible target square, if a 'from' piece is selected
    fn draw_possible_target_squares(&self) {
        let from_option = crate::game::get_from(&self.game);
        if from_option == None  { return; }
        let from = from_option.unwrap();
        let squares = crate::game::get_target_squares(&self.game, from);
        for square in squares {
            let file_index = crate::square::get_nth_file(&square);
            let rank_index = crate::square::get_nth_rank(&square);
            let x = file_index.get() as f32 * get_square_width(&self) as f32;
            // files go up, 'file_index + 1' as tiles are draw from top
            let y = self.game_height as f32 - ((rank_index + 1) as f32 * get_square_height(&self) as f32);
            let mut sprite = sfml::graphics::Sprite::with_texture(&self.assets.get_question_mark());
            sprite.set_position(sfml::system::Vector2f::new(x, y));
            sprite.set_scale(sfml::system::Vector2f::new(get_scale_x(self), get_scale_y(self)));
            self.window.borrow_mut().draw(&sprite);
        }

    }
    /// Draw the selector: cursor, selected 'from' square, selected 'to' square
    fn draw_selector(&self) {
        self.draw_square(Some(self.game.get_selector().get_cursor()), sfml::graphics::Color::RED);
        self.draw_square(self.game.get_selector().get_from(), sfml::graphics::Color::BLUE);
    }
    /// Draw the selector: cursor, selected 'from' square, selected 'to' square
    /// A red color is used for the cursor
    /// A blue color is used for the selected square
    fn draw_square(&self, square_option: Option<crate::square::Square>, color: sfml::graphics::Color) {
        if let Some(square) = square_option {
            let file_index = crate::square::get_nth_file(&square);
            let rank_index = crate::square::get_nth_rank(&square);
            // The 'from' square is smaller than the cursor
            let delta = match color {
                sfml::graphics::Color::RED => 0_f32,
                _ => -8_f32,
            };
            let x = file_index.get() as f32 * get_square_width(&self) as f32;
            // files go up, 'file_index + 1' as tiles are draw from top
            let y = self.game_height as f32 - ((rank_index + 1) as f32 * get_square_height(&self) as f32);
            let mut rectangle = sfml::graphics::RectangleShape::new();
            use sfml::graphics::Shape;
            rectangle.set_fill_color(sfml::graphics::Color::rgba(
                sfml::graphics::Color::red(&color),
                sfml::graphics::Color::green(&color),
                sfml::graphics::Color::blue(&color),
                64 as u8)
            );
            rectangle.set_origin(sfml::system::Vector2f::new(0 as f32, 0 as f32));
            rectangle.set_outline_thickness(5.0);
            rectangle.set_outline_color(color);
            rectangle.set_size(
                sfml::system::Vector2f::new(get_square_width(&self) as f32 + delta, 
                get_square_height(&self) as f32 + delta)
            );
            rectangle.set_position(sfml::system::Vector2f::new(x - (delta / 2.0), y - (delta / 2.0)));
            self.window.borrow_mut().draw(&rectangle);
        }
    }
    /// Draw the coordinats on the square
    fn draw_square_coordinats(&self) {
        let squares = crate::square::get_all_squares();
        for square in squares {
            let file_index = crate::square::get_nth_file(&square);
            let rank_index = crate::square::get_nth_rank(&square);
            let x = file_index.get() as f32 * get_square_width(&self) as f32;
            // files go up, 'file_index + 1' as tiles are draw from top
            let y = self.game_height as f32 - ((rank_index + 1) as f32 * get_square_height(&self) as f32);
            let coordinat = square.get();
            let mut text = sfml::graphics::Text::new(&coordinat, self.assets.get_font(), 32);
            text.set_position(sfml::system::Vector2f::new(x, y));
            text.set_scale(sfml::system::Vector2f::new(get_scale_x(self), get_scale_y(self)));
            self.window.borrow_mut().draw(&text);

        }
    }
    fn draw_squares(&self) {
        for file_index in crate::file_index::get_all_file_indices() {
            for rank_index in 0..8 {
                let x = file_index.get() as f32 * get_square_width(&self) as f32;
                // files go up, 'file_index + 1' as tiles are draw from top
                let y = self.game_height as f32 - ((rank_index + 1) as f32 * get_square_height(&self) as f32);
                let color = crate::board::get_square_color_from_indices(&file_index, rank_index);
                let mut sprite = sfml::graphics::Sprite::with_texture(&self.assets.get_square(color));
                sprite.set_position(sfml::system::Vector2f::new(x, y));
                sprite.set_scale(sfml::system::Vector2f::new(get_scale_x(self), get_scale_y(self)));
                self.window.borrow_mut().draw(&sprite);
            }
        }
    }

    /// Get the assets
    /// 
    /// ```
    /// if !search_and_destroy_chess_2::is_on_gha::is_on_gha() {
    ///     let game_width = 800;
    ///     let game_height = 600;
    ///     let game_view = crate::game_view::GameView::new(game_width, game_height);
    ///     let assets = game_view.get_assets();
    /// }
    /// ```
    pub fn get_assets(&self) -> &crate::assets::Assets {
        &self.assets
    }

    /// Get the height of the view
    /// 
    /// ```
    /// if !search_and_destroy_chess_2::is_on_gha::is_on_gha() {
    ///     let game_width = 800;
    ///     let game_height = 600;
    ///     let game_view = crate::game_view::GameView::new(game_width, game_height);
    ///     assert_eq!(game_view.get_height(), game_height)
    ///     assert_eq!(game_view.get_width(), game_width)
    /// }
    /// ```
    pub fn get_height(&self) -> u32 {
        self.game_height
    }

    /// Get the width of the view
    /// 
    /// ```
    /// if !search_and_destroy_chess_2::is_on_gha::is_on_gha() {
    ///     let game_width = 800;
    ///     let game_height = 600;
    ///     let game_view = crate::game_view::GameView::new(game_width, game_height);
    ///     assert_eq!(game_view.get_width(), game_width)
    ///     assert_eq!(game_view.get_height(), game_height)
    /// }
    /// ```
    pub fn get_width(&self) -> u32 {
        self.game_width
    }

    pub fn run(&mut self) {
        self.window.borrow_mut().set_vertical_sync_enabled(true);

        let mut ball_sound: sfml::audio::Sound = sfml::audio::Sound::with_buffer(&self.assets.get_bounce_sound_buffer());

        loop {
            while let Some(event) = &self.window.borrow_mut().poll_event() {
                match event {
                    sfml::window::Event::Closed
                    | sfml::window::Event::KeyPressed {
                        code: sfml::window::Key::ESCAPE, ..
                    } => return,
                    sfml::window::Event::KeyReleased { code: sfml::window::Key::SPACE, .. } => {
                        if crate::game::can_select(&self.game) {
                            crate::game::do_select(&self.game);
                            ball_sound.play();
                        }
                    }
                    sfml::window::Event::KeyPressed { code: sfml::window::Key::UP, .. } => {
                        crate::game::move_cursor(&self.game, crate::direction::Direction::Up);
                    },
                    sfml::window::Event::KeyPressed { code: sfml::window::Key::RIGHT, .. } => {
                        crate::game::move_cursor(&self.game, crate::direction::Direction::Right);
                    },
                    sfml::window::Event::KeyPressed { code: sfml::window::Key::DOWN, .. } => {
                        crate::game::move_cursor(&self.game, crate::direction::Direction::Down);
                    },
                    sfml::window::Event::KeyPressed { code: sfml::window::Key::LEFT, .. } => {
                        crate::game::move_cursor(&self.game, crate::direction::Direction::Left);
                    },
                    _ => {}
                }
            }

            self.draw();
        }
    }

}

/// Get the scale factor in the horizontal direction
pub fn get_scale_x(game_view: &GameView) -> f32{
    let image_width = game_view.get_assets().get_image_width(); // pixels
    let square_width = game_view.get_width() as f32 / 8.0;
    let scale_x = square_width / image_width as f32;
    scale_x
}

/// Get the scale factor in the vertical direction
pub fn get_scale_y(game_view: &GameView) -> f32{
    let image_height = game_view.get_assets().get_image_height(); // pixels
    let square_height = game_view.get_height() as f32 / 8.0;
    let scale_y = square_height / image_height as f32;
    scale_y
}

/// Get the height of a square.
/// This equals the height of the screen didived by the number of squares the board is high
pub fn get_square_height(game_view: &GameView) -> u32 {
    
    game_view.get_height() / 8
}

/// Get the width of a square.
/// This equals the width of the screen didived by the number of squares the board is wide
pub fn get_square_width(game_view: &GameView) -> u32 {
    game_view.get_width() / 8
}
