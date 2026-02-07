use thiserror::Error;

// 独自エラー型の定義
#[derive(Error, Debug)]
enum MyError {
    #[error("不正な入力です。")]
    InvalidInput,

    #[error("権限がありません。")]
    PermissionDenied,
}

fn main() {
    // 不正な入力エラー
    let err = MyError::InvalidInput;
    println!("{err}");
    println!("{err:?}");

    // 権限拒否エラー
    let err = MyError::PermissionDenied;
    println!("{err}");
    println!("{err:?}");
}
