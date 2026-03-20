use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    let path = "input_file.txt";

    // クロージャーを使って動的にコンテキスト情報を追加してエラーを返す
    let content = fs::read_to_string(path)
        .with_context(|| format!("[main] テキストファイルの読み込みに失敗しました。path={path}"))?;

    println!("{content}");

    Ok(())
}
