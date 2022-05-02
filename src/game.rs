use board::get_piece_at_square;

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

/// Get the square that is selected to move a piece from, if any
pub fn get_cursor_from(game: &crate::game::Game) -> Option<crate::square::Square> {
    game.get_selector().get_from()
}

/// Get the square where the cursor is now, if anywhere
pub fn get_cursor_square(game: &crate::game::Game) -> Option<crate::square::Square> {
    game.get_selector().get_cursor()
}

/// Get the square that is selected to move a piece to, if any
pub fn get_cursor_to(game: &crate::game::Game) -> Option<crate::square::Square> {
    game.get_selector().get_to()
}

/// Get the square that is selected to move a piece to, if any
pub fn get_piece_at_cursor(game: &crate::game::Game) -> Option<crate::piece::Piece> {
    let square: Option<crate::square::Square> = get_cursor_square(game);
    if let None = square {
        return None
    }
    get_piece_at_square(&game.get_board(), &square.unwrap())
}

/// Select a square, e.g. by pressing space
/// * if there is no piece at that square: 
///   * if no 'from' is selected: nothing happens
///   * if a 'from' is selected: the 'from' square is unselected
/// * if there is a piece at that square:
///   * if no 'from' is selected: that square is selected as 'from'
///   * if a 'from' is selected: the new square is selected as 'from'
pub fn do_select(game: &crate::game::Game) {
    let cursor_square: Option<crate::square::Square> = game.get_selector().get_cursor();
    if let None = cursor_square {
        return()
    }
    let piece: Option<crate::piece::Piece> = get_piece_at_cursor(&game);
    if let None = piece {
        game.set_from(None);
        return()
    }
    game.get_selector().set_from(cursor_square);
}

pub fn move_cursor(game: &crate::game::Game, direction: crate::direction::Direction) {
    game.get_selector().move_cursor(direction);
}


pub fn move_cursor_up(game: &crate::game::Game) {
    game.get_selector().move_cursor_up();
}

pub fn set_cursor_at(game: &crate::game::Game, square: crate::square::Square) {
    game.get_selector().set_cursor(Some(square));
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
    #[test]
    fn move_to_a_square() {
        let game = Game::new();
        set_cursor_at(&game, crate::square::Square::new("e2"));
        assert_eq!(get_cursor_square(&game).unwrap(), crate::square::Square::new("e2"));
    }
    #[test]
    fn select_nothing_at_e4() {
        let game = Game::new();
        set_cursor_at(&game, crate::square::Square::new("e4"));
        assert_eq!(get_cursor_square(&game).unwrap(), crate::square::Square::new("e4"));
        assert_eq!(get_cursor_from(&game), None);
        assert_eq!(get_cursor_to(&game), None);
        do_select(&game);
        assert_eq!(get_cursor_square(&game).unwrap(), crate::square::Square::new("e4"));
        assert_eq!(get_cursor_from(&game), None);
        assert_eq!(get_cursor_to(&game), None);
    }
    #[test]
    fn select_from_piece_at_e2() {
        let game = Game::new();
        set_cursor_at(&game, crate::square::Square::new("e2"));
        assert_eq!(get_cursor_square(&game).unwrap(), crate::square::Square::new("e2"));
        assert_eq!(get_cursor_from(&game), None);
        assert_eq!(get_cursor_to(&game), None);
        do_select(&game);
        assert_eq!(get_cursor_square(&game).unwrap(), crate::square::Square::new("e2"));
        assert_eq!(get_cursor_from(&game).unwrap(), crate::square::Square::new("e2"));
        assert_eq!(get_cursor_to(&game), None);
    }
    #[test]
    fn select_from_piece_then_select_nothing() {
        let game = Game::new();
        set_cursor_at(&game, crate::square::Square::new("e2"));
        do_select(&game);
        set_cursor_at(&game, crate::square::Square::new("e4"));
        do_select(&game);
        assert_eq!(get_cursor_square(&game).unwrap(), crate::square::Square::new("e4"));
        assert_eq!(get_cursor_from(&game), None);
        assert_eq!(get_cursor_to(&game), None);
    }
}
