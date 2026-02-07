fn show_result(result: &Result<i32, String>) {
    // Result 型のマッチングで処理を分岐
    match result {
        Ok(x) => println!("結果: {}", x),
        Err(e) => println!("エラー: {}", e)
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("0で割ることはできません。"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let a = 10;
    let b = 2;
    let result = divide(a, b);
    show_result(&result);

    let a = 10;
    let b = 0;
    show_result(&divide(a, b));
}
