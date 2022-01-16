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
        selector.set_cursor(crate::square::get_random_square());

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

/// Determine if the player can select a square
/// This is true if the current player is on a square with one of his/her pieces.
/// It is is irrelevant if another piece is selected: that other piece gets deselected
pub fn can_select(game: &crate::game::Game) -> bool {
    let cursor_square = game.get_selector().get_cursor();
    let piece_option = get_piece_at_square(&game, &cursor_square);
    if piece_option == None { 
        return false
    }
    let piece = piece_option.unwrap();
    return { 
        piece.get_color() == game.get_current_player()
    }
}

/// Determine if the player can select a square
/// This is true if the current player is on a square with one of his/her pieces.
/// It is is irrelevant if another piece is selected: that other piece gets deselected
pub fn do_select(game: &crate::game::Game) {
    assert!(can_select(&game));
    let mut selector = game.get_selector();
    let from = Some(selector.get_cursor());
    selector.set_from(from)
}

pub fn get_invisible_squares(game: &crate::game::Game, color: crate::color::Color) -> Vec<crate::square::Square> {
    crate::board::get_invisible_squares(&game.get_board(), color)
}

pub fn get_piece_from_indices(game: &crate::game::Game, file_index: &crate::file_index::FileIndex, rank_index: usize)  -> Option<crate::piece::Piece> {
    game.get_board().get_piece_from_indices(&file_index, rank_index)
}

pub fn get_piece_at_square(game: &crate::game::Game, square: &crate::square::Square)  -> Option<crate::piece::Piece> {
    get_piece_from_indices(&game, &crate::square::get_nth_file(square), crate::square::get_nth_rank(square))
}

/// Get the square that holds a selected piece, or a None of no piece is selected
pub fn get_from(game: &crate::game::Game) -> Option<crate::square::Square> {
    game.get_selector().get_from()
}

/// Get the target squares for the piece at the square
pub fn get_target_squares(game: &Game, square: crate::square::Square) -> Vec<crate::square::Square> {
    // Basic movements that are possible on a board
    let squares = crate::board::get_target_squares(&game.get_board(), square);

    // En passant

    // Castling, dependent on fog of war

    // Move into chess, dependent on fog of war
    squares
}

pub fn move_cursor(game: &crate::game::Game, direction: crate::direction::Direction) {
    game.get_selector().move_cursor(direction);
}

pub fn set_cursor(game: &mut crate::game::Game, square: &crate::square::Square) {
    crate::selector::set_cursor(&mut game.get_selector(), square);
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
    fn test_move_cursor() {
        let game = Game::new();
        let cursor_before = game.get_selector().get_cursor();
        move_cursor(&game, crate::direction::Direction::Up);
        let cursor_after = game.get_selector().get_cursor();
        assert_ne!(cursor_before, cursor_after);
    }
    #[test]
    fn test_set_cursor() {
        let mut game = Game::new();
        let square = crate::square::Square::new("d2");
        set_cursor(&mut game, &square);
        assert_eq!(game.get_selector().get_from(), None);
        do_select(&mut game);
        assert_eq!(game.get_selector().get_from().unwrap(), square);
    }
    #[test]
    fn test_get_target_squares() {
        let game = Game::new();
        let squares = get_target_squares(&game, crate::square::Square::new("a1"));
        assert_eq!(squares.len(), 0);
    }
}
