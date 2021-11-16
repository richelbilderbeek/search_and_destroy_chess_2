fn is_even(value: i32) -> bool {
    value % 2 == 0
}

fn main() {
    println!("Hello, world!");
    println!("2: {}", is_even(2));
    println!("1: {}", is_even(1));
}
