#[test]
fn can_get() {
    let x = 1;
    let even_number = EvenNumber{value: x};
    assert_eq!(even_number.get(), x);
}
#[test]
fn can_get_2() {
    let x = 1;
    let even_number = EvenNumber{value: x};
    assert_eq!(even_number.get(), x);
}
