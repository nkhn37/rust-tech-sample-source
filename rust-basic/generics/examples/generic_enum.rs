// ジェネリックな列挙体
enum MyResult<S, F> {
    Success(S),    // 成功時の値
    Failure(F, String),  // 失敗時の値とエラー文字列
    Uncertainty,
}

fn main() {
    let mut result = MyResult::Success::<i32, u16>(10);
    // let mut result: MyResult<i32, u16> = MyResult::Success(10);
    if let MyResult::Success(code) = result {
        println!("Success {}", code);
    };

    result = MyResult::Failure(1, "Error1".to_string());
    if let MyResult::Failure(code, error) = result {
        println!("Failure {} {}", code, error);
    }

    result = MyResult::Uncertainty;
    if let MyResult::Uncertainty = result {
        println!("Uncertain");
    }

    // result の S は i32型で使用しているので以下はエラーとなる
    // result = MyResult::Success(10.0);

    // 異なる型での利用
    let result = MyResult::Success::<f64, u8>(10.5);
    if let MyResult::Success(code) = result {
        println!("Success {}", code);
    };
}
