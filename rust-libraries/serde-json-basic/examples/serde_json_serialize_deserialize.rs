use serde::{Deserialize, Serialize};

// Serialize と Deserialize の両方を derive することで
// シリアライズとデシリアライズの両方に対応できる
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() -> Result<(), serde_json::Error> {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

    // 構造体を JSON 文字列に変換（シリアライズ）
    let json = serde_json::to_string(&user)?;
    println!("{json}");

    // JSON 文字列を構造体に変換（デシリアライズ）
    let deserialized_user: User = serde_json::from_str(&json)?;
    println!("{deserialized_user:?}");

    Ok(())
}
