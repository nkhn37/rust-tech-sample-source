use std::fs::OpenOptions;
use std::io::{self, Write};

fn write_append_file(filepath: &str, text: &str) -> io::Result<()> {
    // ファイルを追記モードで開く
    let mut file = OpenOptions::new()
        .append(true) // ファイルが存在しない場合、エラー
        .open(filepath)?;

    // 文字列を書き込む
    writeln!(file, "{text}")?;

    Ok(())
}

fn main() {
    let filepath = r"D:\RustProject\rust-tech-sample-source\rust-basic\file_input_output\examples\output_example.txt";

    // 追記する文字列を準備
    let append_str = "Rust 文字列の追記";

    // ファイル書き込み関数を呼び出し、エラーが発生した場合はエラーを表示する
    if let Err(e) = write_append_file(filepath, append_str) {
        println!("Error: {e}");
    }
}
