/// A selector
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Selector {
    cursor: Option<crate::square::Square>,
    from: Option<crate::square::Square>,
    to: Option<crate::square::Square>,
}

impl Selector {

    /// Create a new Selector
    /// 
    /// ```
    /// use search_and_destroy_chess_2::selector::Selector;
    /// use search_and_destroy_chess_2::color::Color;
    /// 
    /// let selector = Selector::new();
    /// assert_eq!(selector.get_cursor(), None);
    /// assert_eq!(selector.get_from(), None);
    /// assert_eq!(selector.get_to(), None);
    /// ```
    pub fn new() -> Selector {
        Selector {
            cursor: None,
            from: None,
            to: None,
        }
    }

    /// Get the cursor
    /// 
    /// ```
    /// use search_and_destroy_chess_2::selector::Selector;
    /// use search_and_destroy_chess_2::square::get_random_square;
    /// 
    /// let mut selector = Selector::new();
    /// assert_eq!(selector.get_cursor(), None);
    /// let random_square = get_random_square();
    /// selector.set_cursor(Some(random_square.clone()));
    /// assert_eq!(selector.get_cursor(), Some(random_square));
    /// selector.set_cursor(None);
    /// assert_eq!(selector.get_cursor(), None);
    /// ```
    pub fn get_cursor(&self) -> Option<crate::square::Square> { self.cursor.clone() }

    /// Get the selected 'from' square
    /// 
    /// ```
    /// use search_and_destroy_chess_2::selector::Selector;
    /// use search_and_destroy_chess_2::square::get_random_square;
    /// 
    /// let mut selector = Selector::new();
    /// assert_eq!(selector.get_from(), None);
    /// let random_square = get_random_square();
    /// selector.set_from(Some(random_square.clone()));
    /// assert_eq!(selector.get_from(), Some(random_square));
    /// selector.set_from(None);
    /// assert_eq!(selector.get_from(), None);
    /// ```
    pub fn get_from(&self) -> Option<crate::square::Square> { self.from.clone() }

    /// Get the selected 'to' square
    /// 
    /// ```
    /// use search_and_destroy_chess_2::selector::Selector;
    /// use search_and_destroy_chess_2::square::get_random_square;
    /// 
    /// let mut selector = Selector::new();
    /// assert_eq!(selector.get_to(), None);
    /// let random_square = get_random_square();
    /// selector.set_to(Some(random_square.clone()));
    /// assert_eq!(selector.get_to(), Some(random_square));
    /// selector.set_to(None);
    /// assert_eq!(selector.get_to(), None);
    /// ```
    pub fn get_to(&self) -> Option<crate::square::Square> { self.to.clone() }

    /// Moves the cursor up, if there is one.
    /// If the cursor leaves the top of the screen, the cursor will be put at the bottom
    pub fn move_cursor_up(&mut self) {
        if let Some(some_cursor) = &self.cursor {
            self.cursor = Some(crate::square::get_square_above(some_cursor.clone()));
        }
    }

    /// Set the cursor
    /// 
    /// ```
    /// use search_and_destroy_chess_2::selector::Selector;
    /// use search_and_destroy_chess_2::square::get_random_square;
    /// 
    /// let mut selector = Selector::new();
    /// assert_eq!(selector.get_cursor(), None);
    /// let random_square = get_random_square();
    /// selector.set_cursor(Some(random_square.clone()));
    /// assert_eq!(selector.get_cursor(), Some(random_square));
    /// selector.set_cursor(None);
    /// assert_eq!(selector.get_cursor(), None);
    /// ```
    pub fn set_cursor(&mut self, cursor: Option<crate::square::Square>) { self.cursor = cursor }

    /// Set the selected 'from' square
    /// 
    /// ```
    /// use search_and_destroy_chess_2::selector::Selector;
    /// use search_and_destroy_chess_2::square::get_random_square;
    /// 
    /// let mut selector = Selector::new();
    /// assert_eq!(selector.get_from(), None);
    /// let random_square = get_random_square();
    /// selector.set_from(Some(random_square.clone()));
    /// assert_eq!(selector.get_from(), Some(random_square));
    /// selector.set_from(None);
    /// assert_eq!(selector.get_from(), None);
    /// ```
    pub fn set_from(&mut self, from: Option<crate::square::Square>) { self.from = from }

    /// Set the selected 'to' square
    /// 
    /// ```
    /// use search_and_destroy_chess_2::selector::Selector;
    /// use search_and_destroy_chess_2::square::get_random_square;
    /// 
    /// let mut selector = Selector::new();
    /// assert_eq!(selector.get_to(), None);
    /// let random_square = get_random_square();
    /// selector.set_to(Some(random_square.clone()));
    /// assert_eq!(selector.get_to(), Some(random_square));
    /// selector.set_to(None);
    /// assert_eq!(selector.get_to(), None);
    /// ```
    pub fn set_to(&mut self, to: Option<crate::square::Square>) { self.to = to }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_selector() {
        let selector = Selector::new();
        assert_eq!(selector.get_cursor(), None);
        assert_eq!(selector.get_from(), None);
        assert_eq!(selector.get_to(), None);
    }
    #[test]
    fn change_cursor() {
        let mut selector = Selector::new();
        assert_eq!(selector.get_cursor(), None);
        let random_square = crate::square::get_random_square();
        selector.set_cursor(Some(random_square.clone()));
        assert_eq!(selector.get_cursor(), Some(random_square));
        selector.set_cursor(None);
        assert_eq!(selector.get_cursor(), None);
    }
    #[test]
    fn change_from() {
        let mut selector = Selector::new();
        assert_eq!(selector.get_from(), None);
        let random_square = crate::square::get_random_square();
        selector.set_from(Some(random_square.clone()));
        assert_eq!(selector.get_from(), Some(random_square));
        selector.set_from(None);
        assert_eq!(selector.get_from(), None);
    }
    #[test]
    fn change_to() {
        let mut selector = Selector::new();
        assert_eq!(selector.get_to(), None);
        let random_square = crate::square::get_random_square();
        selector.set_to(Some(random_square.clone()));
        assert_eq!(selector.get_to(), Some(random_square));
        selector.set_to(None);
        assert_eq!(selector.get_to(), None);
    }
    #[test]
    fn move_cursor() {
        let mut selector = Selector::new();
        let random_square = crate::square::get_random_square();
        selector.set_cursor(Some(random_square.clone()));
        let before = selector.get_cursor();
        selector.move_cursor_up();
        let after = selector.get_cursor();
        assert_ne!(before, after);
    }
}
