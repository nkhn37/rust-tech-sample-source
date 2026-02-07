use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    // std::io::Error から自動的に変換されるエラー
    #[error("My I/Oエラーが発生: {0}")]
    Io(#[from] std::io::Error),
}

// ファイルを読み込む関数
fn my_read_file(path: &str) -> Result<String, MyError> {
    // fs::read_to_string は std::io::Error を返却するが、
    // #[from] により MyError に自動変換される
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
