use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: Option<String>,
}

fn main() -> Result<(), serde_json::Error> {
    let user1 = User {
        name: String::from("Alice"),
        email: Some(String::from("alice@example.com")),
    };
    let user2 = User {
        name: String::from("Bob"),
        email: None,
    };

    // 構造体を JSON 文字列に変換（シリアライズ）
    let json1 = serde_json::to_string(&user1)?;
    let json2 = serde_json::to_string(&user2)?;
    println!("{json1}");
    println!("{json2}");

    // JSON 文字列を構造体に変換（デシリアライズ）
    let deserialized_user1: User = serde_json::from_str(&json1)?;
    let deserialized_user2: User = serde_json::from_str(&json2)?;
    println!("{deserialized_user1:?}");
    println!("{deserialized_user2:?}");

    Ok(())
}
