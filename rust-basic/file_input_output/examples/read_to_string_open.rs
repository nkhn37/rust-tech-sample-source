use std::fs;
use std::io::{self, Read};

fn read_file(filepath: &str) -> io::Result<()> {
    let mut file = fs::File::open(filepath)?;
    let mut contents = String::new();

    // ファイルの内容を読み込み
    file.read_to_string(&mut contents)?;
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
