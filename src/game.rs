/// A chess game
#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub struct Game {
    board: crate::board::Board,
    current_player: crate::color::Color,
    selector: std::cell::RefCell<crate::selector::Selector>,
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
        let mut selector = crate::selector::Selector::new();
        selector.set_cursor(Some(crate::square::get_random_square()));

        Game {
            board: crate::board::Board::new(),
            current_player: crate::color::Color::White,
            selector: std::cell::RefCell::new(selector),
        }
    }

    pub fn get_board(&self) -> crate::board::Board { self.board.clone() }

    /// ```
    /// use search_and_destroy_chess_2::game::Game;
    /// use search_and_destroy_chess_2::color::Color;
    /// 
    /// let game = Game::new();
    /// let current_player = game.get_current_player();
    /// assert_eq!(current_player, Color::White);
    /// ```
    pub fn get_current_player(&self) -> crate::color::Color { self.current_player.clone() }

    pub fn get_selector(&self) -> std::cell::RefMut<crate::selector::Selector> { self.selector.borrow_mut() }
}

pub fn get_invisible_squares(game: &crate::game::Game, color: crate::color::Color) -> Vec<crate::square::Square> {
    crate::board::get_invisible_squares(&game.get_board(), color)
}

pub fn get_piece_from_indices(game: &crate::game::Game, file_index: &crate::file_index::FileIndex, rank_index: usize)  -> Option<crate::piece::Piece> {
    game.get_board().get_piece_from_indices(&file_index, rank_index)
}

pub fn move_cursor_up(game: &crate::game::Game) {
    game.get_selector().move_cursor_up();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game() {
        let game = Game::new();
        assert_eq!(game.get_current_player(), crate::color::Color::White);
    }
    #[test]
    fn move_cursor() {
        let game = Game::new();
        let cursor_before = game.get_selector().get_cursor();
        move_cursor_up(&game);
        let cursor_after = game.get_selector().get_cursor();
        assert_ne!(cursor_before, cursor_after);
    }
}
