use serde::Serialize;

// 構造体を定義し、Serialize トレイトを derive することで
// 構造体を JSON にシリアライズできるようにする
#[derive(Serialize, Debug)]
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

    Ok(())
}
