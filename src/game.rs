/// A chess game
#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
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
        let mut selector = crate::selector::Selector::new();
        selector.set_cursor(Some(crate::square::get_random_square()));

        Game {
            board: crate::board::Board::new(),
            current_player: crate::color::Color::White,
            selector,
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

    pub fn get_selector(&self) -> &crate::selector::Selector { &self.selector }
}

pub fn get_invisible_squares(game: &crate::game::Game, color: crate::color::Color) -> Vec<crate::square::Square> {
    crate::board::get_invisible_squares(&game.get_board(), color)
}

pub fn get_piece_from_indices(game: &crate::game::Game, file_index: &crate::file_index::FileIndex, rank_index: usize)  -> Option<crate::piece::Piece> {
    game.get_board().get_piece_from_indices(&file_index, rank_index)
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
