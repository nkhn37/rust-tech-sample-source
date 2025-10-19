use std::fs;
use std::path::Path;

pub fn list_examples() {
    println!("例を実行するには、以下のコマンドを実行してください。\n");

    // examplesディレクトリのパスを設定
    let examples_dir = Path::new("examples");

    // ディレクトリの存在確認
    if !examples_dir.exists() {
        eprintln!("examples ディレクトリが存在しません。");
        return;
    }

    // examples ディレクトリのデータを取得
    match fs::read_dir(examples_dir) {
        Ok(entries) => {
            // .rs ファイルのみを抽出
            let mut example_files: Vec<String> = entries
                .filter_map(|entry| entry.ok())
                .filter_map(|e| e.file_name().into_string().ok())
                .filter(|name| name.ends_with(".rs"))
                .collect();

            // ファイルが見つからなかった場合の処理
            if example_files.is_empty() {
                println!("examples ディレクトリに .rs ファイルが見つかりませんでした。");
                return;
            }

            // ファイル名をソートを実行する
            example_files.sort();

            // 実行用コマンドを表示
            for file_name in example_files {
                let example_name = file_name.trim_end_matches(".rs");
                println!("cargo run --example {}", example_name);
            }
        }
        Err(e) => {
            eprintln!("Error reading directory {}: {}", examples_dir.display(), e);
        }
    }
}
