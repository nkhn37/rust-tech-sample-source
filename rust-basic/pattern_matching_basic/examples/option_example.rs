fn show_maybe_number(maybe_number: &Option<i32>) {
    // Option 型のマッチングで処理を分岐
    match maybe_number {
        Some(x) => println!("数値: {}", x),
        None => println!("値がありません。"),
    }
}

fn main() {
    let maybe_number = Some(10);
    show_maybe_number(&maybe_number);

    let maybe_number = None;
    show_maybe_number(&maybe_number);
}
