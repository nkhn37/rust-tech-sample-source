use serde::Serialize;

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

    // 読みやすいように改行・インデント付きの JSON 文字列に変換する
    let json = serde_json::to_string_pretty(&user)?;
    println!("{json}");

    Ok(())
}
