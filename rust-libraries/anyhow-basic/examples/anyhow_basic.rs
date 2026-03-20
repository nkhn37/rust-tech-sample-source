use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let path = "input_file.txt";

    // エラーを anyhow::Error に変換して返す
    let content = fs::read_to_string(path)?;

    println!("{content}");

    Ok(())
}
