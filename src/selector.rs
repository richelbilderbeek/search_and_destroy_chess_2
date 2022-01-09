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

    pub fn get_cursor(&self) -> Option<crate::square::Square> { self.cursor.clone() }
    pub fn get_from(&self) -> Option<crate::square::Square> { self.from.clone() }
    pub fn get_to(&self) -> Option<crate::square::Square> { self.to.clone() }
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
}
