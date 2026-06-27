use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    city: String,
    zip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    addresses: Vec<Address>,
}

fn main() -> Result<(), serde_json::Error> {
    let user = User {
        name: String::from("Alice"),
        addresses: vec![
            Address {
                city: String::from("Tokyo"),
                zip: String::from("100-0001"),
            },
            Address {
                city: String::from("Osaka"),
                zip: String::from("530-0001"),
            },
        ],
    };

    // 構造体を JSON 文字列に変換（シリアライズ）
    let json = serde_json::to_string_pretty(&user)?;
    println!("{json}");

    // JSON 文字列を構造体に変換（デシリアライズ）
    let deserialized_user: User = serde_json::from_str(&json)?;
    println!("{deserialized_user:?}");

    Ok(())
}
