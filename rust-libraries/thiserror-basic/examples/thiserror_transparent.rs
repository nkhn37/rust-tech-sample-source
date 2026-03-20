use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    // 下位のエラーをそのまま伝搬させる
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// ファイルを読み込む関数
fn my_read_file(path: &str) -> Result<String, MyError> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn main() {
    match my_read_file("non_existent_file.txt") {
        Ok(content) => println!("ファイル内容: {content}"),
        Err(e) => {
            println!("エラー : {e}");
            println!("デバッグ情報: {e:?}");
        }
    }
}
