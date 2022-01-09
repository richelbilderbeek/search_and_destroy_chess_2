// A rank index is in usize for now, instead of a full class

pub fn create_random_rank_index() -> usize {
    4
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_rank() {
        let rank_index = create_random_rank_index();
        assert!(rank_index <= 7);
    }
}