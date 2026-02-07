use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    let path = "input_file.txt";

    // コンテキスト情報を追加してエラーを返す
    let content =
        fs::read_to_string(path).context("[main] テキストファイルの読み込みに失敗しました。")?;

    println!("{content}");

    Ok(())
}
