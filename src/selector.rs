/// A selector
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Selector {
    cursor: crate::square::Square,
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
    /// assert_eq!(selector.get_from(), None);
    /// assert_eq!(selector.get_to(), None);
    /// ```
    pub fn new() -> Selector {
        Selector {
            cursor: crate::square::get_random_square(),
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
    /// let random_square = get_random_square();
    /// selector.set_cursor(random_square.clone());
    /// assert_eq!(selector.get_cursor(), random_square);
    /// ```
    pub fn get_cursor(&self) -> crate::square::Square { self.cursor.clone() }

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
    pub fn move_cursor(&mut self, direction: crate::direction::Direction) {
        self.cursor = crate::square::get_square_at(self.cursor.clone(), direction);
    }

    /// Set the cursor
    /// 
    /// ```
    /// use search_and_destroy_chess_2::selector::Selector;
    /// use search_and_destroy_chess_2::square::get_random_square;
    /// 
    /// let mut selector = Selector::new();
    /// let random_square = get_random_square();
    /// selector.set_cursor(random_square.clone());
    /// assert_eq!(selector.get_cursor(), random_square);
    /// ```
    pub fn set_cursor(&mut self, cursor: crate::square::Square) { self.cursor = cursor }

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

pub fn set_cursor(selector: &mut crate::selector::Selector, square: &crate::square::Square) {
    selector.set_cursor(square.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_selector() {
        let selector = Selector::new();
        assert_eq!(selector.get_from(), None);
        assert_eq!(selector.get_to(), None);
    }
    #[test]
    fn change_cursor() {
        let mut selector = Selector::new();
        let random_square = crate::square::get_random_square();
        selector.set_cursor(random_square.clone());
        assert_eq!(selector.get_cursor(), random_square);
        let random_square = crate::square::get_random_square();
        selector.set_cursor(random_square.clone());
        assert_eq!(selector.get_cursor(), random_square);
    }
    #[test]
    fn change_from() {
        let mut selector = Selector::new();
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
        selector.set_cursor(random_square.clone());
        let before = selector.get_cursor();
        selector.move_cursor(crate::direction::Direction::Up);
        let after = selector.get_cursor();
        assert_ne!(before, after);
    }
    #[test]
    fn test_set_cursor() {
        let mut selector = Selector::new();
        let random_square = crate::square::get_random_square();
        set_cursor(&mut selector, &random_square);
        assert_eq!(selector.get_cursor(), random_square);
        let random_square = crate::square::get_random_square();
        set_cursor(&mut selector, &random_square);
        assert_eq!(selector.get_cursor(), random_square);
    }
}
