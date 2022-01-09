/// A chess game
pub struct Game {
    board: crate::board::Board,
    current_player: crate::color::Color,
    selector: crate::selector::Selector,
}

impl Game {

    /// Create a new Game
    /// 
    /// ```
    /// use search_and_destroy_chess_2::game::Game;
    /// use search_and_destroy_chess_2::color::Color;
    /// 
    /// let game = Game::new();
    /// let current_player = game.get_current_player();
    /// assert_eq!(current_player, Color::White);
    /// ```
    pub fn new() -> Game {

        Game {
            board: crate::board::Board::new(),
            current_player: crate::color::Color::White,
            selector: crate::selector::Selector::new(),
        }
    }

    /// ```
    /// use search_and_destroy_chess_2::game::Game;
    /// use search_and_destroy_chess_2::color::Color;
    /// 
    /// let game = Game::new();
    /// let current_player = game.get_current_player();
    /// assert_eq!(current_player, Color::White);
    /// ```
    pub fn get_current_player(&self) -> crate::color::Color { self.current_player.clone() }

    pub fn get_selector(&self) -> crate::selector::Selector { self.selector.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game() {
        let game = Game::new();
        assert_eq!(game.get_current_player(), crate::color::Color::White);
    }
}
