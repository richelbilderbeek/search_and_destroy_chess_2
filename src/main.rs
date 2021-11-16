mod is_even;
mod EvenNumber;

fn main() {
    println!("Hello, world!");
    println!("2: {}", is_even::is_even(2));
    println!("1: {}", is_even::is_even(1));
    let even_number = EvenNumber::EvenNumber{value: 42};
    println!("Even Number: {}", even_number.get());
}
