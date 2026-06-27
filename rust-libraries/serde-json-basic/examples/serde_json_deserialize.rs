use serde::Deserialize;

// 構造体を定義し、Deserialize トレイトを derive することで
// JSON 文字列を構造体にデシリアライズできるようにする
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct User {
    name: String,
    age: u32,
}

fn main() -> Result<(), serde_json::Error> {
    let json = r#"{"name":"Alice","age":30}"#;

    // JSON 文字列を構造体に変換（デシリアライズ）
    let user: User = serde_json::from_str(json)?;
    println!("{user:?}");

    Ok(())
}
