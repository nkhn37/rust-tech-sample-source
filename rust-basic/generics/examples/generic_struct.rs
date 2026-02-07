// ジェネリックな構造体
struct SomeNum<T, U> {
    c: char,
    num1: T,
    num2: U,
    num3: U,
}

fn main() {
    let some_number = SomeNum{
        c:'a', num1: 10, num2: 20.5, num3: 30.5
    };
    println!("{}", some_number.c);
    println!("{}", some_number.num1);
    println!("{}", some_number.num2);
    println!("{}", some_number.num3);

    let some_number1 = SomeNum{
        c: 'b', num1: 10.5, num2: 20, num3: 30
    };
    println!("{}", some_number1.c);
    println!("{}", some_number1.num1);
    println!("{}", some_number1.num2);
    println!("{}", some_number1.num3);
}