/// Detect if code is run on GitHub Actions
/// 
/// ```
/// search_and_destroy_chess_2::is_on_gha::is_on_gha();
/// ```
pub fn is_on_gha() -> bool {
    std::env::var("GITHUB_ACTIONS").is_ok()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on_gha() {
        assert!(is_on_gha() == true || is_on_gha() == false);
    }
}
