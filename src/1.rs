  use std::vec;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.into_iter().sum();
    println!("Sum of the numbers: {}", sum);
}