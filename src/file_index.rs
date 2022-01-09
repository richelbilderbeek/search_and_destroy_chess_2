/// The index of a file, that is the column
#[derive(Debug)]
pub struct FileIndex {
    index: usize
}

impl FileIndex {

    /// Create a FileIndex, i.e. the index of a file (i.e. a column)
    /// 
    /// ```
    /// use search_and_destroy_chess_2::file_index::FileIndex;
    /// 
    /// let a_file_index = FileIndex::new(0); // i.e. the 'a' file
    /// let h_file_index = FileIndex::new(7); // i.e. the 'h' file
    /// ```
    #[allow(dead_code)]
    pub fn new(index: usize) -> FileIndex {
        assert!(index <= 7);

        FileIndex {
            index,
        }
    }

    pub fn get(&self) -> usize {
        self.index.clone()
    }

}

/// Get all the file indices, i.e. 0..8
/// 
///```
/// use search_and_destroy_chess_2::file_index::get_all_file_indices;
/// 
/// let file_indices = get_all_file_indices();
/// assert_eq!(file_indices.len(), 8);
///```
pub fn get_all_file_indices() -> Vec<FileIndex> {
    vec![
        FileIndex::new(0),
        FileIndex::new(1),
        FileIndex::new(2),
        FileIndex::new(3),
        FileIndex::new(4),
        FileIndex::new(5),
        FileIndex::new(6),
        FileIndex::new(7),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_file() {
        let _file_index = FileIndex::new(3);
    }
    #[test]
    fn compare_equal() {
        let file_index_a = FileIndex::new(3);
        let file_index_b = FileIndex::new(3);
        assert_eq!(file_index_a.get(), file_index_b.get());
    }
    #[test]
    fn get() {
        let value = 3_usize;
        let d_file_index = FileIndex::new(value);
        assert_eq!(d_file_index.get(), value);
    }
    #[test]
    fn get_all_file_indices_fn() {
        let file_indices = get_all_file_indices();
        assert_eq!(file_indices.len(), 8);
    }
}