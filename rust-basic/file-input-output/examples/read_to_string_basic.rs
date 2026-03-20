use std::fs;
use std::io::{self};

fn read_file(filepath: &str) -> io::Result<()> {
    // ファイルパスのデータを String 型で取得する
    let contents = fs::read_to_string(filepath)?;
    // 読み込んだ文字列を表示する
    println!("{contents}");

    Ok(())
}

fn main() {
    // ファイルパスは任意のパスに変更してください
    let filepath = r"D:\RustProject\rust-tech-sample-source\rust-basic\file_input_output\examples\input_example.txt";

    // ファイル読み込み関数を呼び出し、エラーが発生した場合はエラーを表示する
    if let Err(e) = read_file(filepath) {
        println!("Error: {e}");
    }
}
