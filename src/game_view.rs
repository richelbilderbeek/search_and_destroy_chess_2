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
    board: crate::board::Board,
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
            board: crate::board::Board::new(),
            assets: crate::assets::Assets::new(),
        }
    }

    /// Draw the screen
    pub fn draw(&self) {
        // Clear the window
        //self.window.borrow_mut().clear(sfml::graphics::Color::rgb(50, 200, 50));


        self.draw_squares();
        self.draw_square_coordinats();
        self.draw_pieces();
        self.draw_fog_of_war();

        // Display things on screen
        self.window.borrow_mut().display();
    }
    fn draw_fog_of_war(&self) {
        use crate::board::get_invisible_squares;
        use crate::square::get_nth_file;
        use crate::square::get_nth_rank;
        use crate::color::Color;

        let square_width = self.game_width as f32 / 8.0;
        let square_height = self.game_height as f32 / 8.0;
        let scale_x = get_scale_x(self);
        let scale_y = get_scale_y(self);
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
    fn draw_pieces(&self) {

        let square_width = self.game_width as f32 / 8.0;
        let square_height = self.game_height as f32 / 8.0;
        let scale_x = get_scale_x(self);
        let scale_y = get_scale_y(self);
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
    fn draw_square_coordinats(&self) {
        let scale_x = get_scale_x(self);
        let scale_y = get_scale_y(self);
        let square_width = self.game_width as f32 / 8.0;
        let square_height = self.game_height as f32 / 8.0;
        let squares = crate::square::get_all_squares();
        for square in squares {
            let file_index = crate::square::get_nth_file(&square);
            let rank_index = crate::square::get_nth_rank(&square);
            let x = file_index as f32 * square_width;
            // files go up, 'file_index + 1' as tiles are draw from top
            let y = self.game_height as f32 - ((rank_index + 1) as f32 * square_height);
            let coordinat = square.get();
            let mut text = sfml::graphics::Text::new(&coordinat, self.assets.get_font(), 32);
            text.set_position(sfml::system::Vector2f::new(x, y));
            text.set_scale(sfml::system::Vector2f::new(scale_x, scale_y));
            self.window.borrow_mut().draw(&text);

        }
    }
    fn draw_squares(&self) {

        let square_width = self.game_width as f32 / 8.0;
        let square_height = self.game_height as f32 / 8.0;
        let scale_x = get_scale_x(self);
        let scale_y = get_scale_y(self);
        for file_index in 0..8 {
            for rank_index in 0..8 {
                let x = file_index as f32 * square_width;
                // files go up, 'file_index + 1' as tiles are draw from top
                let y = self.game_height as f32 - ((rank_index + 1) as f32 * square_height);
                let color = crate::board::get_square_color_from_indices(file_index, rank_index);
                let mut sprite = sfml::graphics::Sprite::with_texture(&self.assets.get_square(color));
                sprite.set_position(sfml::system::Vector2f::new(x, y));
                sprite.set_scale(sfml::system::Vector2f::new(scale_x, scale_y));
                self.window.borrow_mut().draw(&sprite);
            }
        }
    }

    /// Get the assets
    /// 
    /// ```
    /// let game_width = 800;
    /// let game_height = 600;
    /// let game_view = crate::game_view::GameView::new(game_width, game_height);
    /// let assets = game_view.get_assets();
    /// ```
    pub fn get_assets(&self) -> &crate::assets::Assets {
        &self.assets
    }

    /// Get the height of the view
    /// 
    /// ```
    /// let game_width = 800;
    /// let game_height = 600;
    /// let game_view = crate::game_view::GameView::new(game_width, game_height);
    /// assert_eq!(game_view.get_height(), game_height)
    /// assert_eq!(game_view.get_width(), game_width)
    /// ```
    pub fn get_height(&self) -> u32 {
        self.game_height
    }

    /// Get the width of the view
    /// 
    /// ```
    /// let game_width = 800;
    /// let game_height = 600;
    /// let game_view = crate::game_view::GameView::new(game_width, game_height);
    /// assert_eq!(game_view.get_width(), game_width)
    /// assert_eq!(game_view.get_height(), game_height)
    /// ```
    pub fn get_width(&self) -> u32 {
        self.game_width
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

            assert!(up == true || up == false);
            assert!(down == true || down == false);
            self.draw();
        }
    }

}

pub fn get_scale_x(game_view: &GameView) -> f32{
    let image_width = game_view.get_assets().get_image_width(); // pixels
    let square_width = game_view.get_width() as f32 / 8.0;
    let scale_x = square_width / image_width as f32;
    scale_x
}

pub fn get_scale_y(game_view: &GameView) -> f32{
    let image_height = game_view.get_assets().get_image_height(); // pixels
    let square_height = game_view.get_height() as f32 / 8.0;
    let scale_y = square_height / image_height as f32;
    scale_y
}
