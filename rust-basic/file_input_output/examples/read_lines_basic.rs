use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filepath: &str) -> io::Result<()> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    // ファイルを1行ずつ読み込む
    for line in reader.lines() {
        // line は Result<String, Error> なので値を取り出す
        let line = line?;
        println!("{} :文字数({})", line, line.chars().count());
    }

    Ok(())
}

fn main() {
    // ファイルパスは任意のパスに変更してください
    let filepath = r"D:\RustProject\rust-tech-sample-source\rust-basic\file_input_output\examples\input_example.txt";

    // ファイル読み込み関数を呼び出し、エラーが発生した場合はエラーを表示する
    if let Err(e) = read_lines(filepath) {
        println!("Error: {e}");
    }
}
