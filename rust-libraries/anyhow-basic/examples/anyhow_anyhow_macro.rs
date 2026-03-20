use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let path = "";

    if path.is_empty() {
        // エラーを生成する
        let err = anyhow!("[main] パスが空です。");

        // エラーを返却する
        return Err(err);
    }

    Ok(())
}
