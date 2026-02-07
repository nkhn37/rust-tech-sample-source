use anyhow::{Result, bail};

fn main() -> Result<()> {
    let path = "";

    if path.is_empty() {
        // エラーを生成して、即時に返却する
        bail!("[main] パスが空です。");
    }

    Ok(())
}
