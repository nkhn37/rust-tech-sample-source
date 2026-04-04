fn main() {
    let numbers = [1, 2, 3, 4, 5];

    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();

    println!("sum: {}", sum);
    println!("product: {}", product);
}
