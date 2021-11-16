mod even_number;

pub struct EvenNumber {
    value: i64
}

impl EvenNumber {
    pub fn get(&self) -> i64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get() {
        let x = 1;
        let even_number = EvenNumber{value: x};
        assert_eq!(even_number.get(), x);
    }
}
