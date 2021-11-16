pub fn is_even(value: i32) -> bool {
    value % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_even_works() {
        assert_eq!(is_even(1), false);
        assert_eq!(is_even(2), true);
    }
}
