use std::fs::OpenOptions;
use std::io::{self, Write};

fn write_new_file(filepath: &str, text: &str) -> io::Result<()> {
    // ファイルを作成して開く
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true) // ファイルが存在するとエラー
        .open(filepath)?;

    // 文字列を書き込む
    writeln!(file, "{text}")?;

    Ok(())
}

fn main() {
    // ファイルパスは任意のパスに変更してください
    let filepath = r"D:\RustProject\rust-tech-sample-source\rust-basic\file_input_output\examples\output_example.txt";

    // 書き込み文字列を準備
    let write_str = "Rust Programming\nファイル入出力の基本\nテキストファイルの入出力";

    // ファイル書き込み関数を呼び出し、エラーが発生した場合はエラーを表示する
    if let Err(e) = write_new_file(filepath, write_str) {
        println!("Error: {e}");
    }
}
