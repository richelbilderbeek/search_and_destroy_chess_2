// Must be called 'lib.rs' to test
pub struct Board {

}

impl Board {
    pub fn new() -> Board {
        Board{}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board() {
        let board = Board::new();
        assert_eq!(1 + 1, 2)
    }
}
