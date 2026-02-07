use thiserror::Error;

// 独自エラー型の定義 引数付き
#[derive(Error, Debug)]
enum MyError {
    #[error("不正な入力です: {0}")]
    InvalidInput(String),

    #[error("権限がありません: ID {user_id}")]
    PermissionDenied { user_id: u32 },
}

fn main() {
    // 不正な入力エラー
    let err = MyError::InvalidInput("空の文字列".to_string());
    println!("{err}");
    println!("{err:?}");

    // 権限拒否エラー
    let err = MyError::PermissionDenied { user_id: 42 };
    println!("{err}");
    println!("{err:?}");
}
