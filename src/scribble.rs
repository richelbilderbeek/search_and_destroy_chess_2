/*
use std::cell::RefCell;

pub struct Scribble<'a>  {
    buffer: String,
    text: Option<RefCell<&'a String>>,
}

impl <'a> Scribble<'a> {
    pub fn new(text: &str) -> Scribble<'a> {
        let mut scribble = Scribble{
            buffer: String::from(text),
            text: None,
        };
        scribble.text = Some(RefCell::new(&scribble.buffer));
        scribble
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sscribble() {
        let scribble = Scribble::new("hello");
        assert_eq!(scribble.buffer.as_ref(), scribble.text.unwrap().get_mut());
        assert_eq!(1, 2)
    }
}
*/