use std::fmt::Display;

enum MyResult<S, F> {
    Success(S),    // Result
    Failure(F, String),  // FailureCode と エラー文字列
    Uncertainty,
}

// トレイト境界を指定したチェック関数
fn check_result<S, F> (result: &MyResult<S, F>) 
where S: Display, F: Display
{
    match result {
        MyResult::Success(code) => println!("Success: {}", code),
        MyResult::Failure(code, error) => println!("Failure: {} detail: {}", code, error),
        MyResult::Uncertainty => println!("Uncertain Result")
    }
}

fn main() {
    let mut result = MyResult::Success::<i32, u16>(10);
    check_result(&result);

    result = MyResult::Failure(1, "Error1".to_string());
    check_result(&result);
    
    result = MyResult::Uncertainty;
    check_result(&result);
}
